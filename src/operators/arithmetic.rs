//! Arithmetic operator mutation.
//!
//! Replaces Lua arithmetic operators in `binary_expression` nodes with a configured
//! set of alternative operators.

use crate::ast::collect_nodes;
use crate::mutant::{CandidateMutant, Mutator};
use tree_sitter::Tree;

/// Mutator that replaces arithmetic operators in Lua binary expressions.
#[derive(Debug, Clone, Copy, Default)]
pub struct ArithmeticOperatorMutator;

const MUTATOR_ID: &str = "arithmetic_operator";

impl Mutator for ArithmeticOperatorMutator {
    fn id(&self) -> &'static str {
        MUTATOR_ID
    }

    fn generate(&self, source: &str, tree: &Tree) -> Vec<CandidateMutant> {
        let mut mutants = Vec::new();

        for node in collect_nodes(tree, &["binary_expression"]) {
            let Some(op_node) = node.child_by_field_name("operator") else {
                continue;
            };

            let op_text = &source[op_node.byte_range()];
            let replacements = match op_text {
                "+" => &["-", "*", "/"][..],
                "-" => &["+", "*", "/"][..],
                "*" => &["+", "-", "/"][..],
                "/" => &["+", "-", "*"][..],
                "%" => &["*", "/"][..],
                "//" => &["*", "/"][..],
                "^" => &["*", "/"][..],
                _ => continue,
            };

            for replacement in replacements {
                mutants.push(CandidateMutant {
                    start_byte: op_node.start_byte(),
                    end_byte: op_node.end_byte(),
                    replacement: replacement.to_string(),
                });
            }
        }

        mutants
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mutant::MutantGenerator;
    use crate::parser::Parser;

    fn mutants_for(source: &str) -> Vec<crate::mutant::Mutant> {
        let mut parser = Parser::new().unwrap();
        let tree = parser.parse_source(source).unwrap();
        let generator = MutantGenerator::new(vec![Box::new(ArithmeticOperatorMutator)]);
        generator.generate("file.lua", source, &tree)
    }

    fn replacements_for(source: &str) -> Vec<String> {
        mutants_for(source)
            .into_iter()
            .map(|m| m.replacement)
            .collect()
    }

    #[test]
    fn replaces_plus_operator() {
        let replacements = replacements_for("local x = a + b");
        assert_eq!(replacements.len(), 3);
        assert!(replacements.contains(&"-".to_string()));
        assert!(replacements.contains(&"*".to_string()));
        assert!(replacements.contains(&"/".to_string()));
    }

    #[test]
    fn replaces_minus_operator() {
        let replacements = replacements_for("local x = a - b");
        assert_eq!(replacements.len(), 3);
        assert!(replacements.contains(&"+".to_string()));
        assert!(replacements.contains(&"*".to_string()));
        assert!(replacements.contains(&"/".to_string()));
    }

    #[test]
    fn replaces_multiply_operator() {
        let replacements = replacements_for("local x = a * b");
        assert_eq!(replacements.len(), 3);
        assert!(replacements.contains(&"+".to_string()));
        assert!(replacements.contains(&"-".to_string()));
        assert!(replacements.contains(&"/".to_string()));
    }

    #[test]
    fn replaces_divide_operator() {
        let replacements = replacements_for("local x = a / b");
        assert_eq!(replacements.len(), 3);
        assert!(replacements.contains(&"+".to_string()));
        assert!(replacements.contains(&"-".to_string()));
        assert!(replacements.contains(&"*".to_string()));
    }

    #[test]
    fn replaces_modulo_operator() {
        let replacements = replacements_for("local x = a % b");
        assert_eq!(replacements.len(), 2);
        assert!(replacements.contains(&"*".to_string()));
        assert!(replacements.contains(&"/".to_string()));
    }

    #[test]
    fn replaces_floor_division_operator() {
        let replacements = replacements_for("local x = a // b");
        assert_eq!(replacements.len(), 2);
        assert!(replacements.contains(&"*".to_string()));
        assert!(replacements.contains(&"/".to_string()));
    }

    #[test]
    fn replaces_exponent_operator() {
        let replacements = replacements_for("local x = a ^ b");
        assert_eq!(replacements.len(), 2);
        assert!(replacements.contains(&"*".to_string()));
        assert!(replacements.contains(&"/".to_string()));
    }

    #[test]
    fn ignores_non_arithmetic_binary_operators() {
        let mutants = mutants_for("local x = a == b");
        assert!(mutants.is_empty());
    }

    #[test]
    fn ignores_unary_minus() {
        let mutants = mutants_for("local x = -a");
        assert!(mutants.is_empty());
    }

    #[test]
    fn mutants_have_distinct_location_metadata() {
        let source = "return a + b * c";
        let mutants = mutants_for(source);
        assert_eq!(mutants.len(), 6);

        let mut keys = std::collections::HashSet::new();
        for mutant in &mutants {
            let key = (
                mutant.start_byte,
                mutant.end_byte,
                mutant.replacement.clone(),
            );
            assert!(
                keys.insert(key),
                "duplicate mutant metadata for {:?}",
                mutant
            );
        }
    }

    #[test]
    fn mutant_metadata_includes_original_and_replacement_operators() {
        let mutants = mutants_for("return a + b");
        assert_eq!(mutants.len(), 3);
        assert!(mutants.iter().all(|m| m.operator == "arithmetic_operator"));
        assert!(mutants.iter().all(|m| m.original == "+"));
        assert!(mutants
            .iter()
            .all(|m| ["-", "*", "/"].contains(&m.replacement.as_str())));
    }
}
