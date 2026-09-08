//! Normalized mutant run results and interpreter.

use crate::mutant::Mutant;

/// Outcome of a mutant test run.
#[derive(Debug, Clone, PartialEq)]
pub enum MutantResult {
    /// The test suite passed, so the mutant survived.
    Survived {
        mutant: Mutant,
        stdout_snippet: String,
        stderr_snippet: String,
    },
    /// The test suite failed, so the mutant was killed.
    Killed {
        mutant: Mutant,
        stdout_snippet: String,
        stderr_snippet: String,
    },
    /// The test run did not finish within the timeout.
    TimedOut {
        mutant: Mutant,
        stdout_snippet: String,
        stderr_snippet: String,
    },
    /// The runner could not execute the mutant (e.g., missing executable, invalid mutant).
    Error {
        mutant: Mutant,
        reason: String,
        stdout_snippet: String,
        stderr_snippet: String,
    },
}

/// Signal that distinguishes runner failures from test failures.
#[derive(Debug, Clone, PartialEq)]
pub enum RunnerSignal {
    /// The runner exited with the given code.
    Exited(i32),
    /// The runner exceeded the configured timeout.
    TimedOut,
    /// The runner failed to execute (missing executable, crash, etc.).
    RunnerError(String),
}

/// Interprets raw runner output into a normalized `MutantResult`.
///
/// Output snippets are capped to the last `snippet_limit` bytes.
pub fn interpret(
    mutant: Mutant,
    signal: RunnerSignal,
    stdout: &str,
    stderr: &str,
    snippet_limit: usize,
) -> MutantResult {
    let stdout_snippet = snippet(stdout, snippet_limit);
    let stderr_snippet = snippet(stderr, snippet_limit);

    match signal {
        RunnerSignal::RunnerError(reason) => MutantResult::Error {
            mutant,
            reason,
            stdout_snippet,
            stderr_snippet,
        },
        RunnerSignal::TimedOut => MutantResult::TimedOut {
            mutant,
            stdout_snippet,
            stderr_snippet,
        },
        RunnerSignal::Exited(0) => MutantResult::Survived {
            mutant,
            stdout_snippet,
            stderr_snippet,
        },
        RunnerSignal::Exited(_) => MutantResult::Killed {
            mutant,
            stdout_snippet,
            stderr_snippet,
        },
    }
}

fn snippet(text: &str, limit: usize) -> String {
    if text.len() <= limit {
        text.to_string()
    } else {
        text.chars().rev().take(limit).collect::<Vec<_>>().into_iter().rev().collect()
    }
}

impl MutantResult {
    /// Returns a reference to the associated mutant.
    pub fn mutant(&self) -> &Mutant {
        match self {
            MutantResult::Survived { mutant, .. } => mutant,
            MutantResult::Killed { mutant, .. } => mutant,
            MutantResult::TimedOut { mutant, .. } => mutant,
            MutantResult::Error { mutant, .. } => mutant,
        }
    }

    /// Returns the outcome category as a string.
    pub fn category(&self) -> &'static str {
        match self {
            MutantResult::Survived { .. } => "survived",
            MutantResult::Killed { .. } => "killed",
            MutantResult::TimedOut { .. } => "timed_out",
            MutantResult::Error { .. } => "error",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mutant::{CandidateMutant, Mutant};
    use std::path::PathBuf;

    fn dummy_mutant() -> Mutant {
        Mutant::from_candidate(
            CandidateMutant {
                start_byte: 0,
                end_byte: 0,
                replacement: String::new(),
            },
            "dummy",
            PathBuf::from("src/foo.lua"),
            "local x = 1",
        )
    }

    #[test]
    fn exit_zero_maps_to_survived() {
        let result = interpret(dummy_mutant(), RunnerSignal::Exited(0), "", "", 100);
        assert!(matches!(result, MutantResult::Survived { .. }));
    }

    #[test]
    fn non_zero_exit_maps_to_killed() {
        let result = interpret(dummy_mutant(), RunnerSignal::Exited(1), "", "", 100);
        assert!(matches!(result, MutantResult::Killed { .. }));
    }

    #[test]
    fn timeout_maps_to_timed_out() {
        let result = interpret(dummy_mutant(), RunnerSignal::TimedOut, "", "", 100);
        assert!(matches!(result, MutantResult::TimedOut { .. }));
    }

    #[test]
    fn runner_error_maps_to_error() {
        let result = interpret(
            dummy_mutant(),
            RunnerSignal::RunnerError("missing executable".to_string()),
            "",
            "",
            100,
        );
        assert!(matches!(result, MutantResult::Error { .. }));
    }

    #[test]
    fn captures_output_snippets() {
        let stdout = "hello world";
        let stderr = "error details";
        let result = interpret(dummy_mutant(), RunnerSignal::Exited(1), stdout, stderr, 100);
        match result {
            MutantResult::Killed {
                stdout_snippet,
                stderr_snippet,
                ..
            } => {
                assert_eq!(stdout_snippet, stdout);
                assert_eq!(stderr_snippet, stderr);
            }
            _ => panic!("expected killed result"),
        }
    }

    #[test]
    fn bounds_long_output_snippets() {
        let output = "a".repeat(200);
        let result = interpret(dummy_mutant(), RunnerSignal::Exited(1), &output, "", 50);
        match result {
            MutantResult::Killed { stdout_snippet, .. } => {
                assert_eq!(stdout_snippet.len(), 50);
            }
            _ => panic!("expected killed result"),
        }
    }
}
