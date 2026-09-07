/// Configuration for test discovery and execution.
#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    /// Glob patterns used to discover Lua test files.
    pub test_globs: Vec<String>,
    /// Selected test framework adapter (e.g. "busted", "luaunit").
    pub framework: Option<String>,
    /// Custom shell command used by the generic adapter.
    pub test_command: Option<String>,
}

impl Config {
    /// Creates a new configuration with the provided glob patterns.
    pub fn new(test_globs: Vec<String>) -> Self {
        Self {
            test_globs,
            framework: None,
            test_command: None,
        }
    }

    /// Sets the framework adapter.
    pub fn with_framework(mut self, framework: impl Into<String>) -> Self {
        self.framework = Some(framework.into());
        self
    }

    /// Sets the custom test command for the generic adapter.
    pub fn with_test_command(mut self, command: impl Into<String>) -> Self {
        self.test_command = Some(command.into());
        self
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            test_globs: vec![
                "*_spec.lua".to_string(),
                "*_test.lua".to_string(),
                "test_*.lua".to_string(),
            ],
            framework: None,
            test_command: None,
        }
    }
}
