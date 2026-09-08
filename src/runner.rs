//! Mutant execution runner with process isolation and timeout enforcement.

use crate::mutant::Mutant;
use crate::mutant_validation::write_mutant_to_temp;
use crate::result::{interpret, MutantResult, RunnerSignal};
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

static RUN_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Result of running one mutant.
pub type MutantRun = MutantResult;

/// Configuration for the mutant runner.
#[derive(Debug, Clone)]
pub struct RunnerConfig {
    /// Command and arguments used to run tests.
    pub command: Vec<String>,
    /// Timeout per mutant run.
    pub timeout: Duration,
    /// Directory to use as the project root for resolving relative paths.
    pub project_root: PathBuf,
    /// Maximum bytes to keep from stdout/stderr snippets.
    pub snippet_limit: usize,
}

impl Default for RunnerConfig {
    fn default() -> Self {
        Self {
            command: Vec::new(),
            timeout: Duration::from_secs(30),
            project_root: PathBuf::from("."),
            snippet_limit: 1000,
        }
    }
}

/// Executes the configured test command against a single mutant.
pub fn run_mutant(config: &RunnerConfig, mutant: &Mutant, source: &str) -> MutantRun {
    let start = Instant::now();
    let run_id = RUN_COUNTER.fetch_add(1, Ordering::SeqCst);
    let tmp_dir = std::env::temp_dir().join(format!("lmt-mutant-{}-{}", mutant.id, run_id));
    let _ = std::fs::remove_dir_all(&tmp_dir);

    if let Err(e) = copy_dir_all(&config.project_root, &tmp_dir) {
        return interpret(
            mutant.clone(),
            RunnerSignal::RunnerError(format!("failed to copy project to temp dir: {e}")),
            "",
            "",
            config.snippet_limit,
        );
    }

    if let Err(e) = write_mutant_to_temp(&tmp_dir, &config.project_root, mutant, source) {
        let _ = std::fs::remove_dir_all(&tmp_dir);
        return interpret(
            mutant.clone(),
            RunnerSignal::RunnerError(format!("failed to write temp file: {e}")),
            "",
            "",
            config.snippet_limit,
        );
    }

    let (signal, stdout, stderr) = execute_command(config, &tmp_dir);
    let _ = std::fs::remove_dir_all(&tmp_dir);

    let _elapsed = start.elapsed();
    interpret(
        mutant.clone(),
        signal,
        &stdout,
        &stderr,
        config.snippet_limit,
    )
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    std::fs::create_dir_all(&dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            std::fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn execute_command(config: &RunnerConfig, temp_dir: &Path) -> (RunnerSignal, String, String) {
    if config.command.is_empty() {
        return (
            RunnerSignal::RunnerError("no test command configured".to_string()),
            String::new(),
            String::new(),
        );
    }

    let mut command = Command::new(&config.command[0]);
    command.args(&config.command[1..]);
    command.current_dir(temp_dir);
    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());

    let child = match command.spawn() {
        Ok(child) => child,
        Err(e) => {
            return (
                RunnerSignal::RunnerError(format!("failed to spawn test command: {e}")),
                String::new(),
                String::new(),
            );
        }
    };

    match wait_with_timeout(child, config.timeout) {
        Ok(output) => (
            RunnerSignal::Exited(output.status.code().unwrap_or(-1)),
            String::from_utf8_lossy(&output.stdout).to_string(),
            String::from_utf8_lossy(&output.stderr).to_string(),
        ),
        Err(_) => (RunnerSignal::TimedOut, String::new(), String::new()),
    }
}

fn wait_with_timeout(
    mut child: std::process::Child,
    timeout: Duration,
) -> Result<Output, std::io::Error> {
    let start = Instant::now();
    loop {
        match child.try_wait()? {
            Some(_status) => return child.wait_with_output(),
            None => {
                if start.elapsed() >= timeout {
                    let _ = child.kill();
                    let _ = child.wait();
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::TimedOut,
                        "test command timed out",
                    ));
                }
                std::thread::sleep(Duration::from_millis(10));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mutant::{CandidateMutant, Mutant};
    use std::path::PathBuf;

    fn temp_project_root() -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "lmt-runner-test-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0)
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn dummy_mutant(source: &str) -> (Mutant, PathBuf) {
        let project_root = temp_project_root();
        let mutant = Mutant::from_candidate(
            CandidateMutant {
                start_byte: 0,
                end_byte: 0,
                replacement: String::new(),
            },
            "dummy",
            project_root.join("src/foo.lua"),
            source,
        );
        (mutant, project_root)
    }

    fn config_with_root(project_root: PathBuf, command: Vec<String>) -> RunnerConfig {
        RunnerConfig {
            command,
            timeout: Duration::from_secs(1),
            project_root,
            snippet_limit: 1000,
        }
    }

    #[test]
    fn surviving_mutant_when_command_passes() {
        let source = "local x = 1";
        let (mutant, project_root) = dummy_mutant(source);
        let config = config_with_root(
            project_root,
            vec!["sh".to_string(), "-c".to_string(), "exit 0".to_string()],
        );
        let run = run_mutant(&config, &mutant, source);
        assert!(matches!(run, MutantResult::Survived { .. }));
    }

    #[test]
    fn killed_mutant_when_command_fails() {
        let source = "local x = 1";
        let (mutant, project_root) = dummy_mutant(source);
        let config = config_with_root(
            project_root,
            vec!["sh".to_string(), "-c".to_string(), "exit 1".to_string()],
        );
        let run = run_mutant(&config, &mutant, source);
        assert!(matches!(run, MutantResult::Killed { .. }));
    }

    #[test]
    fn records_timeout() {
        let source = "local x = 1";
        let (mutant, project_root) = dummy_mutant(source);
        let config = config_with_root(
            project_root,
            vec!["sh".to_string(), "-c".to_string(), "sleep 10".to_string()],
        );
        let config = RunnerConfig {
            timeout: Duration::from_millis(50),
            ..config
        };
        let run = run_mutant(&config, &mutant, source);
        assert!(matches!(run, MutantResult::TimedOut { .. }));
    }

    #[test]
    fn records_spawn_error() {
        let source = "local x = 1";
        let (mutant, project_root) = dummy_mutant(source);
        let config = config_with_root(
            project_root,
            vec!["this-command-does-not-exist-12345".to_string()],
        );
        let run = run_mutant(&config, &mutant, source);
        assert!(matches!(run, MutantResult::Error { .. }));
    }
}
