use std::path::Path;
use std::process::Command;

/// Result of running a test command.
#[derive(Debug, Clone, PartialEq)]
pub struct TestResult {
    /// Whether the test command reported success.
    pub success: bool,
    /// Standard output captured from the test command.
    pub stdout: String,
    /// Standard error captured from the test command.
    pub stderr: String,
}

impl TestResult {
    /// Creates a new test result.
    pub fn new(success: bool, stdout: impl Into<String>, stderr: impl Into<String>) -> Self {
        Self {
            success,
            stdout: stdout.into(),
            stderr: stderr.into(),
        }
    }
}

/// Adapter that runs a collection of test files and reports the outcome.
pub trait Adapter {
    /// Runs the given test files and returns the test result.
    fn run(&self, test_files: &[&Path]) -> TestResult;
}

/// Built-in framework adapters.
#[derive(Debug, Clone, PartialEq)]
pub enum FrameworkAdapter {
    /// Runs tests with the `busted` command.
    Busted,
    /// Runs tests with `lua` using the luaunit entry logic.
    LuaUnit,
    /// Runs a user-provided shell command.
    Generic { command: String },
}

impl FrameworkAdapter {
    /// Creates an adapter from a framework name and optional custom command.
    pub fn from_config(framework: Option<&str>, command: Option<&str>) -> Self {
        match framework {
            Some("busted") => FrameworkAdapter::Busted,
            Some("luaunit") => FrameworkAdapter::LuaUnit,
            _ => FrameworkAdapter::Generic {
                command: command.unwrap_or("").to_string(),
            },
        }
    }
}

impl Adapter for FrameworkAdapter {
    fn run(&self, test_files: &[&Path]) -> TestResult {
        match self {
            FrameworkAdapter::Busted => run_command("busted", test_files),
            FrameworkAdapter::LuaUnit => run_luaunit(test_files),
            FrameworkAdapter::Generic { command } => run_shell_command(command),
        }
    }
}

fn run_command(program: &str, args: &[&Path]) -> TestResult {
    let output = Command::new(program)
        .args(args.iter().map(|p| p.as_os_str()))
        .output();

    match output {
        Ok(output) => TestResult::new(
            output.status.success(),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
        ),
        Err(e) => TestResult::new(false, "", format!("failed to execute {}: {}", program, e)),
    }
}

fn run_luaunit(test_files: &[&Path]) -> TestResult {
    if test_files.is_empty() {
        return TestResult::new(true, "", "");
    }

    let mut outputs = Vec::new();
    let mut errors = Vec::new();
    let mut all_success = true;

    for file in test_files {
        let result = run_command("lua", &[*file]);
        outputs.push(result.stdout.clone());
        errors.push(result.stderr.clone());
        if !result.success {
            all_success = false;
        }
    }

    TestResult::new(all_success, outputs.join(""), errors.join(""))
}

fn run_shell_command(command: &str) -> TestResult {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", command]).output()
    } else {
        Command::new("sh").args(["-c", command]).output()
    };

    match output {
        Ok(output) => TestResult::new(
            output.status.success(),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
        ),
        Err(e) => TestResult::new(false, "", format!("failed to execute shell command: {}", e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generic_adapter_runs_shell_command() {
        let adapter = FrameworkAdapter::Generic {
            command: "echo hello".to_string(),
        };
        let result = adapter.run(&[]);
        assert!(result.success);
        assert!(result.stdout.contains("hello"));
    }

    #[test]
    fn generic_adapter_reports_failure() {
        let adapter = FrameworkAdapter::Generic {
            command: "exit 1".to_string(),
        };
        let result = adapter.run(&[]);
        assert!(!result.success);
    }

    #[test]
    fn from_config_selects_busted_adapter() {
        let adapter = FrameworkAdapter::from_config(Some("busted"), None);
        assert!(matches!(adapter, FrameworkAdapter::Busted));
    }

    #[test]
    fn from_config_selects_luaunit_adapter() {
        let adapter = FrameworkAdapter::from_config(Some("luaunit"), None);
        assert!(matches!(adapter, FrameworkAdapter::LuaUnit));
    }

    #[test]
    fn from_config_defaults_to_generic_adapter() {
        let adapter = FrameworkAdapter::from_config(None, Some("lua run-tests.lua"));
        assert!(matches!(adapter, FrameworkAdapter::Generic { .. }));
    }
}
