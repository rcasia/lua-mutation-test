//! Mutant execution runner with process isolation and timeout enforcement.

use crate::mutant::Mutant;
use crate::mutant_validation::{apply_mutant, write_mutant_to_temp};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{Duration, Instant};

/// Outcome of executing a single mutant.
#[derive(Debug, Clone, PartialEq)]
pub enum MutantOutcome {
    /// The test suite failed, meaning the mutant was detected.
    Killed,
    /// The test suite passed, meaning the mutant survived.
    Survived,
    /// The test command did not finish within the timeout.
    TimedOut,
    /// The test command could not be spawned or another error occurred.
    Errored(String),
}

/// Result of running one mutant.
#[derive(Debug, Clone, PartialEq)]
pub struct MutantRun {
    pub mutant: Mutant,
    pub outcome: MutantOutcome,
    pub stdout: String,
    pub stderr: String,
    pub duration: Duration,
}

/// Configuration for the mutant runner.
#[derive(Debug, Clone)]
pub struct RunnerConfig {
    /// Command and arguments used to run tests.
    pub command: Vec<String>,
    /// Timeout per mutant run.
    pub timeout: Duration,
    /// Directory to use as the project root for resolving relative paths.
    pub project_root: PathBuf,
}

impl Default for RunnerConfig {
    fn default() -> Self {
        Self {
            command: Vec::new(),
            timeout: Duration::from_secs(30),
            project_root: PathBuf::from("."),
        }
    }
}

/// Executes the configured test command against a single mutant.
pub fn run_mutant(config: &RunnerConfig, mutant: &Mutant, source: &str) -> MutantRun {
    let start = Instant::now();
    let tmp_dir = std::env::temp_dir().join(format!("lmt-mutant-{}", mutant.id));
    let _ = std::fs::remove_dir_all(&tmp_dir);

    let temp_path = match write_mutant_to_temp(&tmp_dir, &config.project_root, mutant, source) {
        Ok(path) => path,
        Err(e) => {
            return MutantRun {
                mutant: mutant.clone(),
                outcome: MutantOutcome::Errored(format!("failed to write temp file: {e}")),
                stdout: String::new(),
                stderr: String::new(),
                duration: start.elapsed(),
            };
        }
    };

    let outcome = execute_command(config, &temp_path);
    let _ = std::fs::remove_dir_all(&tmp_dir);

    MutantRun {
        mutant: mutant.clone(),
        outcome: outcome.outcome,
        stdout: outcome.stdout,
        stderr: outcome.stderr,
        duration: start.elapsed(),
    }
}

struct RawOutcome {
    outcome: MutantOutcome,
    stdout: String,
    stderr: String,
}

fn execute_command(config: &RunnerConfig, _temp_path: &Path) -> RawOutcome {
    if config.command.is_empty() {
        return RawOutcome {
            outcome: MutantOutcome::Errored("no test command configured".to_string()),
            stdout: String::new(),
            stderr: String::new(),
        };
    }

    let mut command = Command::new(&config.command[0]);
    command.args(&config.command[1..]);
    command.current_dir(&config.project_root);

    let child = match command.spawn() {
        Ok(child) => child,
        Err(e) => {
            return RawOutcome {
                outcome: MutantOutcome::Errored(format!("failed to spawn test command: {e}")),
                stdout: String::new(),
                stderr: String::new(),
            };
        }
    };

    match wait_with_timeout(child, config.timeout) {
        Ok(output) => classify_output(output),
        Err(_) => RawOutcome {
            outcome: MutantOutcome::TimedOut,
            stdout: String::new(),
            stderr: String::new(),
        },
    }
}

fn wait_with_timeout(
    mut child: std::process::Child,
    timeout: Duration,
) -> Result<Output, std::io::Error> {
    let start = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                return Ok(Output {
                    status,
                    stdout: Vec::new(),
                    stderr: Vec::new(),
                });
            }
            Ok(None) => {
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
            Err(e) => return Err(e),
        }
    }
}

fn classify_output(output: Output) -> RawOutcome {
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let outcome = if output.status.success() {
        MutantOutcome::Survived
    } else {
        MutantOutcome::Killed
    };

    RawOutcome {
        outcome,
        stdout,
        stderr,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mutant::{CandidateMutant, Mutant};
    use std::path::PathBuf;

    fn dummy_mutant(source: &str) -> Mutant {
        Mutant::from_candidate(
            CandidateMutant {
                start_byte: 0,
                end_byte: 0,
                replacement: String::new(),
            },
            "dummy",
            PathBuf::from("src/foo.lua"),
            source,
        )
    }

    #[test]
    fn surviving_mutant_when_command_passes() {
        let source = "local x = 1";
        let mutant = dummy_mutant(source);
        let config = RunnerConfig {
            command: vec!["sh".to_string(), "-c".to_string(), "exit 0".to_string()],
            timeout: Duration::from_secs(1),
            project_root: PathBuf::from("."),
        };
        let run = run_mutant(&config, &mutant, source);
        assert_eq!(run.outcome, MutantOutcome::Survived);
    }

    #[test]
    fn killed_mutant_when_command_fails() {
        let source = "local x = 1";
        let mutant = dummy_mutant(source);
        let config = RunnerConfig {
            command: vec!["sh".to_string(), "-c".to_string(), "exit 1".to_string()],
            timeout: Duration::from_secs(1),
            project_root: PathBuf::from("."),
        };
        let run = run_mutant(&config, &mutant, source);
        assert_eq!(run.outcome, MutantOutcome::Killed);
    }

    #[test]
    fn records_timeout() {
        let source = "local x = 1";
        let mutant = dummy_mutant(source);
        let config = RunnerConfig {
            command: vec!["sh".to_string(), "-c".to_string(), "sleep 10".to_string()],
            timeout: Duration::from_millis(50),
            project_root: PathBuf::from("."),
        };
        let run = run_mutant(&config, &mutant, source);
        assert_eq!(run.outcome, MutantOutcome::TimedOut);
    }

    #[test]
    fn records_spawn_error() {
        let source = "local x = 1";
        let mutant = dummy_mutant(source);
        let config = RunnerConfig {
            command: vec!["this-command-does-not-exist-12345".to_string()],
            timeout: Duration::from_secs(1),
            project_root: PathBuf::from("."),
        };
        let run = run_mutant(&config, &mutant, source);
        assert!(matches!(run.outcome, MutantOutcome::Errored(_)));
    }
}
