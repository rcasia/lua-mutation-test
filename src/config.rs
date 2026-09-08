//! Project configuration and filtering.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::str::FromStr;

/// Default configuration file name.
pub const DEFAULT_CONFIG_PATH: &str = ".lua-mutation-test.toml";

/// Difficulty level controlling the trade-off between run time and completeness.
///
/// The cap applies per mutable item and per operator (a source region such as a
/// binary expression, condition, or statement). This keeps coverage broad: every
/// mutable site is still visited, but lower difficulties keep fewer mutations
/// per operator per site.
///
/// | Level | Mutants per operator per item |
/// |-------|-------------------------------|
/// | `very_easy` | 1 |
/// | `easy` | 2 |
/// | `normal` | 3 |
/// | `medium` | 5 |
/// | `hard` | 10 |
/// | `very_hard` | unlimited |
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Difficulty {
    VeryEasy,
    Easy,
    Normal,
    Medium,
    Hard,
    VeryHard,
}

impl Default for Difficulty {
    fn default() -> Self {
        Difficulty::VeryHard
    }
}

impl FromStr for Difficulty {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "very_easy" | "very easy" => Ok(Difficulty::VeryEasy),
            "easy" => Ok(Difficulty::Easy),
            "normal" => Ok(Difficulty::Normal),
            "medium" => Ok(Difficulty::Medium),
            "hard" => Ok(Difficulty::Hard),
            "very_hard" | "very hard" => Ok(Difficulty::VeryHard),
            _ => Err(format!("unknown difficulty: {s}")),
        }
    }
}

impl Difficulty {
    /// Returns the maximum number of mutants to keep per operator per mutable
    /// item for this difficulty. `None` means no limit.
    pub fn max_mutants_per_item(self) -> Option<usize> {
        match self {
            Difficulty::VeryEasy => Some(1),
            Difficulty::Easy => Some(2),
            Difficulty::Normal => Some(3),
            Difficulty::Medium => Some(5),
            Difficulty::Hard => Some(10),
            Difficulty::VeryHard => None,
        }
    }
}

/// Versioned project configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    /// Schema version.
    #[serde(default = "default_version")]
    pub version: String,

    /// Custom shell command used to run tests.
    pub test_command: Option<String>,

    /// Selected test framework adapter (e.g. "busted", "luaunit").
    pub framework: Option<String>,

    /// Timeout in seconds for each mutant test run.
    pub timeout: Option<u64>,

    /// Glob patterns used to discover Lua test files.
    #[serde(default = "default_test_globs")]
    pub test_globs: Vec<String>,

    /// Glob patterns used to discover Lua source files.
    #[serde(default = "default_source_globs")]
    pub source_globs: Vec<String>,

    /// Mutation operators to run.
    #[serde(default)]
    pub operators: Filter,

    /// Output report format(s).
    #[serde(default)]
    pub output: Vec<String>,

    /// Parallelism level (number of concurrent mutant runs).
    #[serde(default)]
    pub parallelism: Option<usize>,

    /// File include/exclude filters.
    #[serde(default)]
    pub files: Filter,

    /// Function name include/exclude filters.
    #[serde(default)]
    pub functions: Filter,

    /// Difficulty level controlling the per-file mutant cap.
    #[serde(default)]
    pub difficulty: Difficulty,
}

fn default_version() -> String {
    "1".to_string()
}

fn default_test_globs() -> Vec<String> {
    vec![
        "*_spec.lua".to_string(),
        "*_test.lua".to_string(),
        "test_*.lua".to_string(),
    ]
}

fn default_source_globs() -> Vec<String> {
    vec!["*.lua".to_string()]
}

/// Default operator weights used only for internal prioritization when a
/// difficulty cap limits the number of mutants per file. Higher values are kept
/// first. This is not exposed in the configuration file.
fn default_operator_weights() -> HashMap<String, i32> {
    [
        ("arithmetic_operator".to_string(), 100),
        ("relational_operator".to_string(), 90),
        ("logical_operator".to_string(), 80),
        ("condition_negation".to_string(), 70),
        ("control_flow".to_string(), 60),
    ]
    .into_iter()
    .collect()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: default_version(),
            test_command: None,
            framework: None,
            timeout: None,
            test_globs: default_test_globs(),
            source_globs: default_source_globs(),
            operators: Filter::default(),
            output: Vec::new(),
            parallelism: None,
            files: Filter::default(),
            functions: Filter::default(),
            difficulty: Difficulty::default(),
        }
    }
}

impl Config {
    /// Creates a new configuration with the provided test glob patterns.
    pub fn new(test_globs: Vec<String>) -> Self {
        Self {
            test_globs,
            ..Self::default()
        }
    }

    /// Sets the custom test command for the generic adapter.
    pub fn with_test_command(mut self, command: impl Into<String>) -> Self {
        self.test_command = Some(command.into());
        self
    }

    /// Sets the framework adapter.
    pub fn with_framework(mut self, framework: impl Into<String>) -> Self {
        self.framework = Some(framework.into());
        self
    }

    /// Returns the internal priority weight for an operator. Used when a
    /// difficulty cap requires selecting which mutants to keep per file.
    pub fn operator_weight(&self, operator: &str) -> i32 {
        default_operator_weights()
            .get(operator)
            .copied()
            .unwrap_or(0)
    }
}

impl Config {
    /// Loads configuration from a TOML or JSON file.
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let path = path.as_ref();
        let contents =
            std::fs::read_to_string(path).map_err(|e| ConfigError::Read(e.to_string()))?;

        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        let config: Config = if ext.eq_ignore_ascii_case("json") {
            serde_json::from_str(&contents).map_err(|e| ConfigError::Parse(e.to_string()))?
        } else {
            toml::from_str(&contents).map_err(|e| ConfigError::Parse(e.to_string()))?
        };

        if config.version != "1" {
            return Err(ConfigError::UnsupportedVersion(config.version));
        }

        Ok(config)
    }

    /// Merges another config into this one, with `other` taking precedence.
    pub fn merge(&mut self, other: Config) {
        if let Some(v) = other.test_command {
            self.test_command = Some(v);
        }
        if let Some(v) = other.framework {
            self.framework = Some(v);
        }
        if let Some(v) = other.timeout {
            self.timeout = Some(v);
        }
        if !other.test_globs.is_empty() {
            self.test_globs = other.test_globs;
        }
        if !other.source_globs.is_empty() {
            self.source_globs = other.source_globs;
        }
        if !other.output.is_empty() {
            self.output = other.output;
        }
        if let Some(v) = other.parallelism {
            self.parallelism = Some(v);
        }
        if !other.operators.include.is_empty() || !other.operators.exclude.is_empty() {
            self.operators = other.operators;
        }
        if !other.files.include.is_empty() || !other.files.exclude.is_empty() {
            self.files = other.files;
        }
        if !other.functions.include.is_empty() || !other.functions.exclude.is_empty() {
            self.functions = other.functions;
        }
        self.difficulty = other.difficulty;
    }
}

/// Include/exclude filter for identifiers, file paths, or function names.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Filter {
    /// Items to include. If non-empty, only matching items are kept.
    #[serde(default)]
    pub include: Vec<String>,

    /// Items to exclude. Always takes precedence over include.
    #[serde(default)]
    pub exclude: Vec<String>,
}

impl Filter {
    /// Returns true if `value` passes the filter.
    ///
    /// Patterns are treated as glob patterns when they contain `*` or `?`, otherwise
    /// as literal strings.
    pub fn matches(&self, value: &str) -> bool {
        if self
            .exclude
            .iter()
            .any(|pattern| matches_pattern(value, pattern))
        {
            return false;
        }
        if self.include.is_empty() {
            return true;
        }
        self.include
            .iter()
            .any(|pattern| matches_pattern(value, pattern))
    }
}

fn matches_pattern(value: &str, pattern: &str) -> bool {
    if pattern.contains('*') || pattern.contains('?') {
        glob_match(pattern, value)
    } else {
        value == pattern
    }
}

fn glob_match(pattern: &str, value: &str) -> bool {
    glob_match_bytes(pattern.as_bytes(), value.as_bytes())
}

fn glob_match_bytes(pattern: &[u8], value: &[u8]) -> bool {
    match pattern.split_first() {
        None => value.is_empty(),
        Some((b'*', rest)) => {
            // `*` matches zero or more characters.
            if rest.is_empty() {
                return true;
            }
            for i in 0..=value.len() {
                if glob_match_bytes(rest, &value[i..]) {
                    return true;
                }
            }
            false
        }
        Some((b'?', rest)) => match value.split_first() {
            None => false,
            Some((_, v_rest)) => glob_match_bytes(rest, v_rest),
        },
        Some((&p, rest)) => match value.split_first() {
            None => false,
            Some((&v, v_rest)) => p == v && glob_match_bytes(rest, v_rest),
        },
    }
}

/// Errors that can occur when loading configuration.
#[derive(Debug, PartialEq)]
pub enum ConfigError {
    Read(String),
    Parse(String),
    UnsupportedVersion(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::Read(msg) => write!(f, "failed to read config: {msg}"),
            ConfigError::Parse(msg) => write!(f, "failed to parse config: {msg}"),
            ConfigError::UnsupportedVersion(v) => {
                write!(f, "unsupported config version: {v}")
            }
        }
    }
}

impl std::error::Error for ConfigError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_toml_config() {
        let source = r#"
version = "1"
test_command = "busted"
timeout = 30
"#;
        let config: Config = toml::from_str(source).unwrap();
        assert_eq!(config.test_command, Some("busted".to_string()));
        assert_eq!(config.timeout, Some(30));
        assert_eq!(config.version, "1");
    }

    #[test]
    fn parses_json_config() {
        let source = r#"{"version":"1","framework":"luaunit","parallelism":4}"#;
        let config: Config = serde_json::from_str(source).unwrap();
        assert_eq!(config.framework, Some("luaunit".to_string()));
        assert_eq!(config.parallelism, Some(4));
    }

    #[test]
    fn filter_excludes_take_precedence() {
        let filter = Filter {
            include: vec!["*.lua".to_string()],
            exclude: vec!["*_test.lua".to_string()],
        };
        assert!(filter.matches("foo.lua"));
        assert!(!filter.matches("foo_test.lua"));
    }

    #[test]
    fn filter_include_restricts() {
        let filter = Filter {
            include: vec!["math_*".to_string()],
            exclude: Vec::new(),
        };
        assert!(filter.matches("math_add"));
        assert!(!filter.matches("string_concat"));
    }

    #[test]
    fn glob_match_basic() {
        assert!(glob_match("*.lua", "foo.lua"));
        assert!(!glob_match("*.lua", "foo.txt"));
        assert!(glob_match("test_*", "test_foo"));
        assert!(glob_match("?at", "cat"));
    }

    #[test]
    fn parses_difficulty() {
        let source = r#"
version = "1"
difficulty = "easy"
"#;
        let config: Config = toml::from_str(source).unwrap();
        assert_eq!(config.difficulty, Difficulty::Easy);
        assert_eq!(config.difficulty.max_mutants_per_item(), Some(2));
    }

    #[test]
    fn default_difficulty_is_unlimited() {
        let config: Config = toml::from_str("version = \"1\"").unwrap();
        assert_eq!(config.difficulty, Difficulty::VeryHard);
        assert_eq!(config.difficulty.max_mutants_per_item(), None);
    }

    #[test]
    fn difficulty_caps_are_gradual() {
        assert_eq!(Difficulty::VeryEasy.max_mutants_per_item(), Some(1));
        assert_eq!(Difficulty::Easy.max_mutants_per_item(), Some(2));
        assert_eq!(Difficulty::Normal.max_mutants_per_item(), Some(3));
        assert_eq!(Difficulty::Medium.max_mutants_per_item(), Some(5));
        assert_eq!(Difficulty::Hard.max_mutants_per_item(), Some(10));
        assert_eq!(Difficulty::VeryHard.max_mutants_per_item(), None);
    }
}
