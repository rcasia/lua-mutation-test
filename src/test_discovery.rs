use glob::{glob_with, MatchOptions};
use std::path::{Path, PathBuf};

/// Default glob patterns for discovering Lua test files.
pub const DEFAULT_TEST_GLOBS: &[&str] = &["*_spec.lua", "*_test.lua", "test_*.lua"];

/// Discovers Lua test files under `root` matching the provided glob patterns.
///
/// Patterns are resolved relative to `root`. Results are sorted deterministically
/// and deduplicated.
pub fn discover_tests(root: &Path, globs: &[String]) -> Vec<PathBuf> {
    let mut discovered = std::collections::BTreeSet::new();
    let options = MatchOptions {
        case_sensitive: true,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };

    for pattern in globs {
        let full_pattern = root.join(pattern);
        let pattern_str = match full_pattern.to_str() {
            Some(s) => s,
            None => continue,
        };

        let matches = match glob_with(pattern_str, options) {
            Ok(m) => m,
            Err(_) => continue,
        };

        for path in matches.flatten() {
            if path.is_file() {
                discovered.insert(path);
            }
        }
    }

    discovered.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn temp_dir_with_files(files: &[&str]) -> tempfile::TempDir {
        let dir = tempfile::tempdir().unwrap();
        for file in files {
            let path = dir.path().join(file);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).unwrap();
            }
            fs::write(path, "-- test").unwrap();
        }
        dir
    }

    #[test]
    fn discovers_default_test_patterns() {
        let dir =
            temp_dir_with_files(&["foo_spec.lua", "bar_test.lua", "test_baz.lua", "helper.lua"]);
        let globs: Vec<String> = DEFAULT_TEST_GLOBS.iter().map(|s| s.to_string()).collect();
        let tests = discover_tests(dir.path(), &globs);

        let names: Vec<String> = tests
            .iter()
            .map(|p| p.file_name().unwrap().to_string_lossy().to_string())
            .collect();

        assert!(names.contains(&"foo_spec.lua".to_string()));
        assert!(names.contains(&"bar_test.lua".to_string()));
        assert!(names.contains(&"test_baz.lua".to_string()));
        assert!(!names.contains(&"helper.lua".to_string()));
    }

    #[test]
    fn discovers_custom_glob_pattern() {
        let dir = temp_dir_with_files(&[
            "tests/unit/foo.lua",
            "tests/integration/bar.lua",
            "src/main.lua",
        ]);
        let tests = discover_tests(dir.path(), &["tests/**/*.lua".to_string()]);

        assert_eq!(tests.len(), 2);
    }

    #[test]
    fn returns_empty_when_no_patterns_match() {
        let dir = temp_dir_with_files(&["main.lua", "helper.lua"]);
        let globs: Vec<String> = DEFAULT_TEST_GLOBS.iter().map(|s| s.to_string()).collect();
        let tests = discover_tests(dir.path(), &globs);
        assert!(tests.is_empty());
    }
}
