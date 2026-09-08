//! Mutant validation, temporary file generation, and source mapping.

use crate::mutant::Mutant;
use crate::parser::Parser;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Outcome of validating a mutant.
#[derive(Debug, Clone, PartialEq)]
pub enum MutantOutcome {
    /// The mutant is syntactically valid and can be executed.
    Valid,
    /// The mutant is syntactically invalid and should be skipped.
    Error(String),
}

/// Validates that `source` is syntactically valid Lua.
pub fn validate_source(source: &str) -> MutantOutcome {
    let mut parser = match Parser::new() {
        Ok(p) => p,
        Err(e) => return MutantOutcome::Error(e.to_string()),
    };

    match parser.parse_source(source) {
        Ok(_) => MutantOutcome::Valid,
        Err(e) => MutantOutcome::Error(e.to_string()),
    }
}

/// Applies a mutant to the original source and returns the mutated source text.
pub fn apply_mutant(source: &str, mutant: &Mutant) -> String {
    let mut result = String::new();
    result.push_str(&source[..mutant.start_byte]);
    result.push_str(&mutant.replacement);
    result.push_str(&source[mutant.end_byte..]);
    result
}

/// Writes a validated mutant to a temporary directory preserving the original relative path.
///
/// The file is written to `<tmp_dir>/<relative_path>`.
pub fn write_mutant_to_temp(
    tmp_dir: impl AsRef<Path>,
    project_root: impl AsRef<Path>,
    mutant: &Mutant,
    source: &str,
) -> std::io::Result<PathBuf> {
    let relative = mutant
        .file
        .strip_prefix(project_root.as_ref())
        .unwrap_or(&mutant.file);
    let dest = tmp_dir.as_ref().join(relative);
    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&dest, apply_mutant(source, mutant))?;
    Ok(dest)
}

/// Maps mutated source line numbers back to original source line numbers.
///
/// For line-preserving mutations the map is 1:1 by line.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct SourceMap {
    /// (original_file, original_line) for each mutated_line (1-indexed).
    lines: HashMap<(String, usize), (PathBuf, usize)>,
}

impl SourceMap {
    /// Creates a source map for a single mutant assuming line-preserving edits.
    pub fn for_mutant(mutant: &Mutant, original_line_count: usize) -> Self {
        let mut lines = HashMap::new();
        for line in 1..=original_line_count {
            lines.insert(
                (mutant.id.clone(), line),
                (mutant.file.clone(), line),
            );
        }
        Self { lines }
    }

    /// Records a mapping from a mutated location to an original location.
    pub fn insert(
        &mut self,
        mutant_id: String,
        mutated_line: usize,
        original_file: PathBuf,
        original_line: usize,
    ) {
        self.lines
            .insert((mutant_id, mutated_line), (original_file, original_line));
    }

    /// Looks up the original location for a mutated line.
    pub fn lookup(&self, mutant_id: &str, mutated_line: usize) -> Option<&(PathBuf, usize)> {
        self.lines.get(&(mutant_id.to_string(), mutated_line))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mutant::{CandidateMutant, Mutant};

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
    fn validates_valid_lua() {
        assert_eq!(validate_source("local x = 1"), MutantOutcome::Valid);
    }

    #[test]
    fn invalidates_bad_lua() {
        let outcome = validate_source("local x =");
        assert!(
            matches!(outcome, MutantOutcome::Error(_)),
            "expected an error outcome"
        );
    }

    #[test]
    fn applies_mutant_replacement() {
        let source = "local x = 1 + 2";
        let plus_offset = source.find('+').unwrap();
        let mutant = Mutant::from_candidate(
            CandidateMutant {
                start_byte: plus_offset,
                end_byte: plus_offset + 1,
                replacement: "-".to_string(),
            },
            "arithmetic",
            PathBuf::from("src/foo.lua"),
            source,
        );
        assert_eq!(apply_mutant(source, &mutant), "local x = 1 - 2");
    }

    #[test]
    fn writes_mutant_preserving_relative_path() {
        let tmp = std::env::temp_dir().join(format!("lmt-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&tmp);
        let source = "local x = 1";
        let mutant = dummy_mutant(source);
        let path = write_mutant_to_temp(&tmp, "/", &mutant, source).unwrap();
        assert_eq!(path, tmp.join("src/foo.lua"));
        assert_eq!(std::fs::read_to_string(&path).unwrap(), source);
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn source_map_looks_up_original_line() {
        let source = "local x = 1\nlocal y = 2";
        let mutant = dummy_mutant(source);
        let map = SourceMap::for_mutant(&mutant, 2);
        assert_eq!(
            map.lookup(&mutant.id, 2),
            Some(&(PathBuf::from("src/foo.lua"), 2))
        );
    }
}
