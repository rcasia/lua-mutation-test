//! Mutant representation, generation, and serialization.

use crate::mutant_validation::{apply_mutant, validate_source, MutantOutcome};
use crate::position::byte_offset_to_position;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use tree_sitter::Tree;

/// A mutation operator that can generate candidate mutants from a parsed file.
pub trait Mutator: Send + Sync {
    /// Returns the operator's stable identifier.
    fn id(&self) -> &'static str;

    /// Generates candidate mutants from the given source and parsed tree.
    fn generate(&self, source: &str, tree: &Tree) -> Vec<CandidateMutant>;
}

/// Intermediate mutant produced by an operator before final metadata is attached.
#[derive(Debug, Clone, PartialEq)]
pub struct CandidateMutant {
    pub start_byte: usize,
    pub end_byte: usize,
    pub replacement: String,
}

/// A concrete mutant with stable id and source-location metadata.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mutant {
    /// Stable content-based identifier.
    pub id: String,
    /// Operator that produced this mutant.
    pub operator: String,
    /// Path to the source file.
    pub file: PathBuf,
    /// Byte range in the original source.
    pub start_byte: usize,
    pub end_byte: usize,
    /// 1-indexed source position.
    pub line: usize,
    pub column: usize,
    /// Original source text being replaced.
    pub original: String,
    /// Replacement source text.
    pub replacement: String,
    /// Reason this mutant is classified as likely equivalent, if any.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub equivalent_reason: Option<String>,
}

impl Mutant {
    /// Constructs a `Mutant` from a candidate and source metadata.
    pub fn from_candidate(
        candidate: CandidateMutant,
        operator: &str,
        file: impl AsRef<Path>,
        source: &str,
    ) -> Self {
        let file = file.as_ref().to_path_buf();
        let original = source[candidate.start_byte..candidate.end_byte].to_string();
        let position = byte_offset_to_position(source, candidate.start_byte)
            .unwrap_or(crate::position::Position { line: 1, column: 1 });
        let id = compute_mutant_id(
            &file,
            candidate.start_byte,
            candidate.end_byte,
            &candidate.replacement,
        );

        Self {
            id,
            operator: operator.to_string(),
            file,
            start_byte: candidate.start_byte,
            end_byte: candidate.end_byte,
            line: position.line,
            column: position.column,
            original,
            replacement: candidate.replacement,
            equivalent_reason: None,
        }
    }

    /// Marks this mutant as likely equivalent with the given reason.
    pub fn with_equivalent_reason(mut self, reason: impl Into<String>) -> Self {
        self.equivalent_reason = Some(reason.into());
        self
    }
}

fn compute_mutant_id(file: &Path, start_byte: usize, end_byte: usize, replacement: &str) -> String {
    let mut hasher = DefaultHasher::new();
    file.to_string_lossy().hash(&mut hasher);
    start_byte.hash(&mut hasher);
    end_byte.hash(&mut hasher);
    replacement.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

/// Generates and deduplicates mutants for a source file using a set of mutators.
pub struct MutantGenerator {
    mutators: Vec<Box<dyn Mutator>>,
}

impl MutantGenerator {
    /// Creates a generator with the given mutators.
    pub fn new(mutators: Vec<Box<dyn Mutator>>) -> Self {
        Self { mutators }
    }

    /// Generates mutants for the given file and parsed source.
    ///
    /// Likely-equivalent mutants are filtered out and mutants that do not
    /// produce syntactically valid Lua are discarded. Use
    /// [`Self::generate_with_equivalents`] when equivalent mutants should be
    /// reported separately.
    pub fn generate(&self, file: impl AsRef<Path>, source: &str, tree: &Tree) -> Vec<Mutant> {
        self.generate_with_equivalents(file, source, tree).0
    }

    /// Generates mutants and returns both executable mutants and likely-equivalent
    /// mutants detected by static heuristics.
    ///
    /// Mutants that do not produce syntactically valid Lua are discarded.
    pub fn generate_with_equivalents(
        &self,
        file: impl AsRef<Path>,
        source: &str,
        tree: &Tree,
    ) -> (Vec<Mutant>, Vec<Mutant>) {
        let file = file.as_ref();
        let mut seen = HashSet::new();
        let mut executable = Vec::new();
        let mut equivalent = Vec::new();

        for mutator in &self.mutators {
            for candidate in mutator.generate(source, tree) {
                let key = (
                    file.to_path_buf(),
                    candidate.start_byte,
                    candidate.end_byte,
                    candidate.replacement.clone(),
                );
                if !seen.insert(key) {
                    continue;
                }

                let mutant = Mutant::from_candidate(candidate.clone(), mutator.id(), file, source);
                let mutated = apply_mutant(source, &mutant);
                if !matches!(validate_source(&mutated), MutantOutcome::Valid) {
                    continue;
                }

                if let Some(reason) =
                    crate::equivalent_heuristics::classify(source, tree, &candidate)
                {
                    equivalent.push(mutant.with_equivalent_reason(reason));
                } else {
                    executable.push(mutant);
                }
            }
        }

        (executable, equivalent)
    }

    /// Generates mutants and keeps only those that are syntactically valid Lua.
    ///
    /// Returns `(valid, invalid, equivalent)` where `valid` contains executable
    /// mutants, `invalid` is always empty because invalid mutants are discarded,
    /// and `equivalent` contains mutants classified as likely equivalent by
    /// static heuristics.
    pub fn generate_validated(
        &self,
        file: impl AsRef<Path>,
        source: &str,
        tree: &Tree,
    ) -> (Vec<Mutant>, Vec<(Mutant, String)>, Vec<Mutant>) {
        let (valid, equivalent) = self.generate_with_equivalents(file, source, tree);
        (valid, Vec::new(), equivalent)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Parser;

    struct DummyMutator;

    impl Mutator for DummyMutator {
        fn id(&self) -> &'static str {
            "dummy"
        }

        fn generate(&self, _source: &str, tree: &Tree) -> Vec<CandidateMutant> {
            crate::ast::collect_nodes(tree, &["number"])
                .into_iter()
                .map(|node| CandidateMutant {
                    start_byte: node.start_byte(),
                    end_byte: node.end_byte(),
                    replacement: "0".to_string(),
                })
                .collect()
        }
    }

    #[test]
    fn generates_mutants_from_candidates() {
        let mut parser = Parser::new().unwrap();
        let source = "local x = 1 + 2";
        let tree = parser.parse_source(source).unwrap();
        let generator = MutantGenerator::new(vec![Box::new(DummyMutator)]);
        let mutants = generator.generate("file.lua", source, &tree);
        assert_eq!(mutants.len(), 2);
        assert!(mutants.iter().all(|m| m.operator == "dummy"));
    }

    #[test]
    fn mutant_ids_are_stable() {
        let mut parser = Parser::new().unwrap();
        let source = "local x = 1";
        let tree = parser.parse_source(source).unwrap();
        let generator = MutantGenerator::new(vec![Box::new(DummyMutator)]);
        let first = generator.generate("file.lua", source, &tree);
        let second = generator.generate("file.lua", source, &tree);
        assert_eq!(first[0].id, second[0].id);
    }

    #[test]
    fn deduplicates_identical_mutants() {
        struct AnotherDummyMutator;

        impl Mutator for AnotherDummyMutator {
            fn id(&self) -> &'static str {
                "another_dummy"
            }

            fn generate(&self, _source: &str, tree: &Tree) -> Vec<CandidateMutant> {
                crate::ast::collect_nodes(tree, &["number"])
                    .into_iter()
                    .map(|node| CandidateMutant {
                        start_byte: node.start_byte(),
                        end_byte: node.end_byte(),
                        replacement: "0".to_string(),
                    })
                    .collect()
            }
        }

        let mut parser = Parser::new().unwrap();
        let source = "local x = 1";
        let tree = parser.parse_source(source).unwrap();
        let generator =
            MutantGenerator::new(vec![Box::new(DummyMutator), Box::new(AnotherDummyMutator)]);
        let mutants = generator.generate("file.lua", source, &tree);
        assert_eq!(mutants.len(), 1);
    }

    #[test]
    fn mutant_serializes_to_json() {
        let mut parser = Parser::new().unwrap();
        let source = "local x = 1";
        let tree = parser.parse_source(source).unwrap();
        let generator = MutantGenerator::new(vec![Box::new(DummyMutator)]);
        let mutants = generator.generate("file.lua", source, &tree);
        let json = serde_json::to_string(&mutants[0]).unwrap();
        let round_trip: Mutant = serde_json::from_str(&json).unwrap();
        assert_eq!(mutants[0], round_trip);
    }

    struct ValidAndInvalidMutator;

    impl Mutator for ValidAndInvalidMutator {
        fn id(&self) -> &'static str {
            "valid_and_invalid"
        }

        fn generate(&self, _source: &str, tree: &Tree) -> Vec<CandidateMutant> {
            // Replace the first number literal with a valid or an invalid value.
            let number = crate::ast::collect_nodes(tree, &["number"])
                .into_iter()
                .next()
                .unwrap();
            vec![
                CandidateMutant {
                    start_byte: number.start_byte(),
                    end_byte: number.end_byte(),
                    replacement: "2".to_string(),
                },
                CandidateMutant {
                    start_byte: number.start_byte(),
                    end_byte: number.end_byte(),
                    replacement: "(".to_string(),
                },
            ]
        }
    }

    #[test]
    fn discards_invalid_lua_mutants() {
        let mut parser = Parser::new().unwrap();
        let source = "local x = 1";
        let tree = parser.parse_source(source).unwrap();
        let generator = MutantGenerator::new(vec![Box::new(ValidAndInvalidMutator)]);

        let mutants = generator.generate("file.lua", source, &tree);
        assert_eq!(mutants.len(), 1);
        assert_eq!(mutants[0].replacement, "2");

        let (executable, equivalent) =
            generator.generate_with_equivalents("file.lua", source, &tree);
        assert_eq!(executable.len(), 1);
        assert!(equivalent.is_empty());

        let (valid, invalid, equivalent) = generator.generate_validated("file.lua", source, &tree);
        assert_eq!(valid.len(), 1);
        assert!(invalid.is_empty());
        assert!(equivalent.is_empty());
    }
}
