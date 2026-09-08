//! Mutant execution runner with process isolation and timeout enforcement.

use crate::mutant::Mutant;
use crate::mutant_validation::write_mutant_to_temp;
use crate::result::{interpret, MutantResult, RunnerSignal};
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::sync::atomic::{AtomicI32, AtomicU64, Ordering};
use std::time::{Duration, Instant};

static RUN_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Maximum number of child process group IDs we track for cleanup.
const MAX_TRACKED_PIDS: usize = 128;

/// Lock-free registry of active child process group IDs. Used by the interrupt
/// handler to kill whole process trees without taking a mutex.
static CHILD_PIDS: [AtomicI32; MAX_TRACKED_PIDS] = [const { AtomicI32::new(0) }; MAX_TRACKED_PIDS];

/// Registers a child process group ID so it can be killed on interrupt.
fn register_child_process(pid: i32) {
    for slot in &CHILD_PIDS {
        if slot
            .compare_exchange(0, pid, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
        {
            return;
        }
    }
}

/// Unregisters a child process group ID.
fn unregister_child_process(pid: i32) {
    for slot in &CHILD_PIDS {
        if slot.load(Ordering::SeqCst) == pid {
            slot.store(0, Ordering::SeqCst);
            return;
        }
    }
}

/// Kills every registered child process group. Safe to call from a signal
/// handler because it uses only atomic operations and `libc::kill`.
pub fn kill_all_child_processes() {
    #[cfg(unix)]
    {
        for slot in &CHILD_PIDS {
            let pid = slot.load(Ordering::SeqCst);
            if pid > 0 {
                unsafe {
                    // Negative PID sends the signal to the whole process group.
                    libc::kill(-pid, libc::SIGKILL);
                }
                slot.store(0, Ordering::SeqCst);
            }
        }
    }
}

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

/// Executes the configured test command against a single mutant using a fresh
/// temporary project copy. Prefer [`MutantRunner`] when running many mutants
/// sequentially, since it reuses a single project copy.
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
            0,
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
            0,
        );
    }

    let (signal, stdout, stderr) = execute_command(config, &tmp_dir);
    let _ = std::fs::remove_dir_all(&tmp_dir);

    let elapsed = start.elapsed();
    interpret(
        mutant.clone(),
        signal,
        &stdout,
        &stderr,
        config.snippet_limit,
        elapsed.as_millis() as u64,
    )
}

/// A reusable runner that keeps one project copy and applies one mutation at a
/// time. This avoids copying the whole project for every single mutant.
pub struct MutantRunner {
    config: RunnerConfig,
    temp_dir: PathBuf,
}

impl MutantRunner {
    /// Creates a runner by copying the project root into a temporary directory.
    pub fn new(config: RunnerConfig) -> Result<Self, String> {
        let run_id = RUN_COUNTER.fetch_add(1, Ordering::SeqCst);
        let temp_dir = std::env::temp_dir().join(format!("lmt-runner-{run_id}"));
        let _ = std::fs::remove_dir_all(&temp_dir);
        copy_dir_all(&config.project_root, &temp_dir)
            .map_err(|e| format!("failed to copy project to temp dir: {e}"))?;
        Ok(Self { config, temp_dir })
    }

    /// Applies a single mutant to the reusable project copy, runs the test
    /// command, and restores the mutated file to its original source.
    pub fn run(&mut self, mutant: &Mutant, source: &str) -> MutantResult {
        let relative = mutant
            .file
            .strip_prefix(&self.config.project_root)
            .unwrap_or(&mutant.file);
        let dest = self.temp_dir.join(relative);

        // Ensure the parent directory exists, then restore the original source.
        if let Some(parent) = dest.parent() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                return self.runner_error(mutant, &format!("failed to create temp dir: {e}"));
            }
        }
        if let Err(e) = std::fs::write(&dest, source) {
            return self.runner_error(mutant, &format!("failed to restore original file: {e}"));
        }

        // Apply the mutation.
        let mutated = crate::mutant_validation::apply_mutant(source, mutant);
        if let Err(e) = std::fs::write(&dest, mutated) {
            return self.runner_error(mutant, &format!("failed to write mutant file: {e}"));
        }

        // Run the tests against the mutated project copy.
        let start = Instant::now();
        let (signal, stdout, stderr) = execute_command(&self.config, &self.temp_dir);
        let duration_ms = start.elapsed().as_millis() as u64;

        // Restore the original source so the next mutant sees a clean project.
        let _ = std::fs::write(&dest, source);

        interpret(
            mutant.clone(),
            signal,
            &stdout,
            &stderr,
            self.config.snippet_limit,
            duration_ms,
        )
    }

    fn runner_error(&self, mutant: &Mutant, reason: &str) -> MutantResult {
        interpret(
            mutant.clone(),
            RunnerSignal::RunnerError(reason.to_string()),
            "",
            "",
            self.config.snippet_limit,
            0,
        )
    }
}

impl Drop for MutantRunner {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.temp_dir);
    }
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

    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        unsafe {
            command.pre_exec(|| {
                // Put the child in its own process group so we can kill the
                // whole tree (e.g., `make test` -> `nvim`) on timeout or
                // interrupt.
                libc::setpgid(0, 0);
                Ok(())
            });
        }
    }

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

    #[cfg(unix)]
    let pgid = child.id() as i32;
    #[cfg(unix)]
    register_child_process(pgid);

    let result = match wait_with_timeout(child, config.timeout) {
        Ok(output) => (
            RunnerSignal::Exited(output.status.code().unwrap_or(-1)),
            String::from_utf8_lossy(&output.stdout).to_string(),
            String::from_utf8_lossy(&output.stderr).to_string(),
        ),
        Err(_) => (RunnerSignal::TimedOut, String::new(), String::new()),
    };

    #[cfg(unix)]
    unregister_child_process(pgid);

    result
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
                    #[cfg(unix)]
                    {
                        let pgid = child.id() as i32;
                        unsafe {
                            // Kill the entire process group, not just the
                            // immediate child, so `make test` -> `nvim` trees
                            // are cleaned up.
                            libc::kill(-pgid, libc::SIGKILL);
                        }
                    }
                    #[cfg(not(unix))]
                    {
                        let _ = child.kill();
                    }
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
