//! Mutation operators for relational/logical operators and control-flow conditions.

use crate::ast::walk;
use crate::mutant::{CandidateMutant, Mutator};
use tree_sitter::Tree;

/// Mutates Lua relational operators in `binary_expression` nodes.
///
/// Each relational operator is replaced with the five other relational operators,
/// producing mutants such as `a == b` -> `a ~= b`.
#[derive(Debug, Clone, Copy, Default)]
pub struct RelationalOperatorMutator;

impl RelationalOperatorMutator {
    /// Creates a new relational operator mutator.
    pub fn new() -> Self {
        Self
    }

    fn replacements_for(op: &str) -> &'static [&'static str] {
        match op {
            "==" => &["~=", "<", ">", "<=", ">="],
            "~=" => &["==", "<", ">", "<=", ">="],
            "<" => &["==", "~=", ">", "<=", ">="],
            ">" => &["==", "~=", "<", "<=", ">="],
            "<=" => &["==", "~=", "<", ">", ">="],
            ">=" => &["==", "~=", "<", ">", "<="],
            _ => &[],
        }
    }
}

impl Mutator for RelationalOperatorMutator {
    fn id(&self) -> &'static str {
        "relational_operator"
    }

    fn generate(&self, source: &str, tree: &Tree) -> Vec<CandidateMutant> {
        let mut candidates = Vec::new();

        for node in walk(tree) {
            if node.kind() != "binary_expression" {
                continue;
            }

            let Some(op_node) = node.child_by_field_name("operator") else {
                continue;
            };

            let op_text = &source[op_node.byte_range()];
            for replacement in Self::replacements_for(op_text) {
                candidates.push(CandidateMutant {
                    start_byte: op_node.start_byte(),
                    end_byte: op_node.end_byte(),
                    replacement: replacement.to_string(),
                });
            }
        }

        candidates
    }
}

/// Mutates Lua logical operators in `binary_expression` nodes.
///
/// `and` becomes `or` and `or` becomes `and`.
#[derive(Debug, Clone, Copy, Default)]
pub struct LogicalOperatorMutator;

impl LogicalOperatorMutator {
    /// Creates a new logical operator mutator.
    pub fn new() -> Self {
        Self
    }

    fn replacement_for(op: &str) -> Option<&'static str> {
        match op {
            "and" => Some("or"),
            "or" => Some("and"),
            _ => None,
        }
    }
}

impl Mutator for LogicalOperatorMutator {
    fn id(&self) -> &'static str {
        "logical_operator"
    }

    fn generate(&self, source: &str, tree: &Tree) -> Vec<CandidateMutant> {
        let mut candidates = Vec::new();

        for node in walk(tree) {
            if node.kind() != "binary_expression" {
                continue;
            }

            let Some(op_node) = node.child_by_field_name("operator") else {
                continue;
            };

            let op_text = &source[op_node.byte_range()];
            if let Some(replacement) = Self::replacement_for(op_text) {
                candidates.push(CandidateMutant {
                    start_byte: op_node.start_byte(),
                    end_byte: op_node.end_byte(),
                    replacement: replacement.to_string(),
                });
            }
        }

        candidates
    }
}

/// Negates the condition expression of control-flow statements.
///
/// The condition of `if`, `while`, `repeat ... until`, and `elseif` is wrapped
/// in `not (...)`, e.g. `if x then ... end` becomes `if not (x) then ... end`.
#[derive(Debug, Clone, Copy, Default)]
pub struct ConditionNegationMutator;

impl ConditionNegationMutator {
    /// Creates a new condition negation mutator.
    pub fn new() -> Self {
        Self
    }
}

impl Mutator for ConditionNegationMutator {
    fn id(&self) -> &'static str {
        "condition_negation"
    }

    fn generate(&self, source: &str, tree: &Tree) -> Vec<CandidateMutant> {
        let mut candidates = Vec::new();

        for node in walk(tree) {
            let condition = match node.kind() {
                "if_statement" | "while_statement" | "repeat_statement" | "elseif_statement" => {
                    node.child_by_field_name("condition")
                }
                _ => None,
            };

            let Some(condition) = condition else {
                continue;
            };

            let original = &source[condition.byte_range()];
            candidates.push(CandidateMutant {
                start_byte: condition.start_byte(),
                end_byte: condition.end_byte(),
                replacement: format!("not ({})", original),
            });
        }

        candidates
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mutant::MutantGenerator;
    use crate::parser::Parser;

    fn generate(mutator: Box<dyn Mutator>, source: &str) -> Vec<crate::mutant::Mutant> {
        let mut parser = Parser::new().unwrap();
        let tree = parser.parse_source(source).unwrap();
        let generator = MutantGenerator::new(vec![mutator]);
        generator.generate("file.lua", source, &tree)
    }

    #[test]
    fn relational_operator_replaces_equality() {
        let source = "return a == b";
        let mutants = generate(Box::new(RelationalOperatorMutator::new()), source);
        assert_eq!(mutants.len(), 5);

        let replacements: Vec<_> = mutants.iter().map(|m| m.replacement.as_str()).collect();
        assert!(replacements.contains(&"~="));
        assert!(replacements.contains(&"<"));
        assert!(replacements.contains(&">"));
        assert!(replacements.contains(&"<="));
        assert!(replacements.contains(&">="));
        assert!(mutants.iter().all(|m| m.original == "=="));
    }

    #[test]
    fn relational_operator_replaces_not_equal() {
        let source = "return a ~= b";
        let mutants = generate(Box::new(RelationalOperatorMutator::new()), source);
        assert_eq!(mutants.len(), 5);

        let replacements: Vec<_> = mutants.iter().map(|m| m.replacement.as_str()).collect();
        assert!(replacements.contains(&"=="));
        assert!(!replacements.contains(&"~="));
        assert!(mutants.iter().all(|m| m.original == "~="));
    }

    #[test]
    fn relational_operator_replaces_less_than() {
        let source = "return a < b";
        let mutants = generate(Box::new(RelationalOperatorMutator::new()), source);
        let replacements: Vec<_> = mutants.iter().map(|m| m.replacement.as_str()).collect();
        assert!(replacements.contains(&"=="));
        assert!(replacements.contains(&"~="));
        assert!(replacements.contains(&">"));
        assert!(replacements.contains(&"<="));
        assert!(replacements.contains(&">="));
        assert!(!replacements.contains(&"<"));
    }

    #[test]
    fn relational_operator_replaces_greater_than() {
        let source = "return a > b";
        let mutants = generate(Box::new(RelationalOperatorMutator::new()), source);
        let replacements: Vec<_> = mutants.iter().map(|m| m.replacement.as_str()).collect();
        assert!(replacements.contains(&"=="));
        assert!(replacements.contains(&"~="));
        assert!(replacements.contains(&"<"));
        assert!(replacements.contains(&"<="));
        assert!(replacements.contains(&">="));
        assert!(!replacements.contains(&">"));
    }

    #[test]
    fn relational_operator_replaces_less_than_or_equal() {
        let source = "return a <= b";
        let mutants = generate(Box::new(RelationalOperatorMutator::new()), source);
        let replacements: Vec<_> = mutants.iter().map(|m| m.replacement.as_str()).collect();
        assert!(replacements.contains(&"=="));
        assert!(replacements.contains(&"~="));
        assert!(replacements.contains(&"<"));
        assert!(replacements.contains(&">"));
        assert!(replacements.contains(&">="));
        assert!(!replacements.contains(&"<="));
    }

    #[test]
    fn relational_operator_replaces_greater_than_or_equal() {
        let source = "return a >= b";
        let mutants = generate(Box::new(RelationalOperatorMutator::new()), source);
        let replacements: Vec<_> = mutants.iter().map(|m| m.replacement.as_str()).collect();
        assert!(replacements.contains(&"=="));
        assert!(replacements.contains(&"~="));
        assert!(replacements.contains(&"<"));
        assert!(replacements.contains(&">"));
        assert!(replacements.contains(&"<="));
        assert!(!replacements.contains(&">="));
    }

    #[test]
    fn logical_operator_swaps_and_with_or() {
        let source = "return a and b";
        let mutants = generate(Box::new(LogicalOperatorMutator::new()), source);
        assert_eq!(mutants.len(), 1);
        assert_eq!(mutants[0].original, "and");
        assert_eq!(mutants[0].replacement, "or");
    }

    #[test]
    fn logical_operator_swaps_or_with_and() {
        let source = "return a or b";
        let mutants = generate(Box::new(LogicalOperatorMutator::new()), source);
        assert_eq!(mutants.len(), 1);
        assert_eq!(mutants[0].original, "or");
        assert_eq!(mutants[0].replacement, "and");
    }

    #[test]
    fn condition_negation_negates_if_condition() {
        let mutants = generate(Box::new(ConditionNegationMutator::new()), "if x then end");
        assert_eq!(mutants.len(), 1);
        assert_eq!(mutants[0].original, "x");
        assert_eq!(mutants[0].replacement, "not (x)");
    }

    #[test]
    fn condition_negation_negates_while_condition() {
        let mutants = generate(Box::new(ConditionNegationMutator::new()), "while x do end");
        assert_eq!(mutants.len(), 1);
        assert_eq!(mutants[0].original, "x");
        assert_eq!(mutants[0].replacement, "not (x)");
    }

    #[test]
    fn condition_negation_negates_repeat_until_condition() {
        let mutants = generate(Box::new(ConditionNegationMutator::new()), "repeat until x");
        assert_eq!(mutants.len(), 1);
        assert_eq!(mutants[0].original, "x");
        assert_eq!(mutants[0].replacement, "not (x)");
    }

    #[test]
    fn records_operator_location_metadata() {
        let source = "return a == b";
        let mutants = generate(Box::new(RelationalOperatorMutator::new()), source);
        assert_eq!(mutants.len(), 5);

        let first = mutants.iter().find(|m| m.original == "==").unwrap();
        assert_eq!(first.operator, "relational_operator");
        assert_eq!(first.start_byte, 9);
        assert_eq!(first.end_byte, 11);
        assert_eq!(first.line, 1);
        assert_eq!(first.column, 10);
    }
}
