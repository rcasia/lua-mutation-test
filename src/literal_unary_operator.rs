//! Literal and unary-operator mutation operators.
//!
//! These operators generate mutants by altering Lua literals and by removing
//! unary operators. Literal and unary-operator mutations are prone to producing
//! equivalent mutants; the most common cases are noted below.
//!
//! # Known equivalent-mutant cases
//!
//! * Boolean literals in contexts where the surrounding expression already
//!   forces a boolean coercion can be equivalent. For example, mutating
//!   `if not true then ... end` to `if not false then ... end` changes the
//!   condition, but a test that only asserts the *structure* of the branch
//!   rather than its truth value may kill both mutants or neither.
//! * Replacing a non-empty string with `""` is equivalent when the surrounding
//!   code treats all non-nil strings identically (for instance, only checking
//!   `if s then ... end`).
//! * Removing `not` from `not not x` yields `x`, which is behaviorally
//!   different, but mutating the inner literal of `not true` to `not false`
//!   can leave the overall expression truth value unchanged in some contexts.
//! * Replacing `nil` with `false` is equivalent in comparisons that coerce both
//!   values to boolean, such as `if x == nil then ... end` when the test only
//!   checks whether the branch is taken.

use crate::ast;
use crate::mutant::{CandidateMutant, Mutator};
use tree_sitter::{Node, Tree};

/// Mutates Lua literals: boolean, number, string, and `nil`.
#[derive(Debug, Clone, Copy, Default)]
pub struct LiteralMutator;

impl LiteralMutator {
    /// Creates a new literal mutator.
    pub fn new() -> Self {
        Self
    }
}

impl Mutator for LiteralMutator {
    fn id(&self) -> &'static str {
        "literal"
    }

    fn generate(&self, source: &str, tree: &Tree) -> Vec<CandidateMutant> {
        ast::walk(tree)
            .filter_map(|node| mutate_literal_node(source, node))
            .flat_map(|candidates| candidates.into_iter())
            .collect()
    }
}

fn mutate_literal_node(source: &str, node: Node<'_>) -> Option<Vec<CandidateMutant>> {
    let start = node.start_byte();
    let end = node.end_byte();
    let text = &source[start..end];

    match node.kind() {
        "true" => Some(vec![CandidateMutant {
            start_byte: start,
            end_byte: end,
            replacement: "false".to_string(),
        }]),
        "false" => Some(vec![CandidateMutant {
            start_byte: start,
            end_byte: end,
            replacement: "true".to_string(),
        }]),
        "nil" => Some(vec![CandidateMutant {
            start_byte: start,
            end_byte: end,
            replacement: "false".to_string(),
        }]),
        "string" => Some(vec![CandidateMutant {
            start_byte: start,
            end_byte: end,
            replacement: "\"\"".to_string(),
        }]),
        "number" => {
            let replacements = mutate_number(text);
            Some(
                replacements
                    .into_iter()
                    .map(|replacement| CandidateMutant {
                        start_byte: start,
                        end_byte: end,
                        replacement,
                    })
                    .collect(),
            )
        }
        _ => None,
    }
}

fn mutate_number(text: &str) -> Vec<String> {
    let mut replacements = Vec::new();

    if let Ok(n) = text.parse::<i64>() {
        replacements.push((n + 1).to_string());
        replacements.push((n - 1).to_string());
        replacements.push((-n).to_string());
        replacements.push("0".to_string());
        replacements.push("1".to_string());
    } else if let Ok(f) = text.parse::<f64>() {
        replacements.push((-f).to_string());
        replacements.push("0".to_string());
        replacements.push("1".to_string());
    } else {
        // Fallback for non-decimal numeric literals (e.g. hexadecimal).
        if text.starts_with('-') {
            replacements.push(text[1..].to_string());
        } else {
            replacements.push(format!("-{}", text));
        }
        replacements.push("0".to_string());
    }

    replacements
}

/// Removes unary operators by replacing the expression with its operand.
#[derive(Debug, Clone, Copy, Default)]
pub struct UnaryOperatorMutator;

impl UnaryOperatorMutator {
    /// Creates a new unary operator mutator.
    pub fn new() -> Self {
        Self
    }
}

impl Mutator for UnaryOperatorMutator {
    fn id(&self) -> &'static str {
        "unary_operator"
    }

    fn generate(&self, source: &str, tree: &Tree) -> Vec<CandidateMutant> {
        ast::walk(tree)
            .filter(|node| node.kind() == "unary_expression")
            .filter_map(|node| {
                let operator = node.child_by_field_name("operator")?;
                let operand = node.child_by_field_name("operand")?;
                let op_text = operator.utf8_text(source.as_bytes()).ok()?;
                if !matches!(op_text, "-" | "not" | "#" | "~") {
                    return None;
                }
                Some(CandidateMutant {
                    start_byte: node.start_byte(),
                    end_byte: node.end_byte(),
                    replacement: source[operand.start_byte()..operand.end_byte()].to_string(),
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mutant::MutantGenerator;
    use crate::parser::Parser;

    fn mutants_for(source: &str, mutator: Box<dyn Mutator>) -> Vec<crate::mutant::Mutant> {
        let mut parser = Parser::new().unwrap();
        let tree = parser.parse_source(source).unwrap();
        let generator = MutantGenerator::new(vec![mutator]);
        generator.generate("file.lua", source, &tree)
    }

    #[test]
    fn boolean_true_becomes_false() {
        let mutants = mutants_for("local x = true", Box::new(LiteralMutator::new()));
        assert_eq!(mutants.len(), 1);
        assert_eq!(mutants[0].original, "true");
        assert_eq!(mutants[0].replacement, "false");
    }

    #[test]
    fn boolean_false_becomes_true() {
        let mutants = mutants_for("local x = false", Box::new(LiteralMutator::new()));
        assert_eq!(mutants.len(), 1);
        assert_eq!(mutants[0].original, "false");
        assert_eq!(mutants[0].replacement, "true");
    }

    #[test]
    fn number_mutations_include_increment_decrement_sign_flip_and_zero_one() {
        let mutants = mutants_for("local x = 5", Box::new(LiteralMutator::new()));
        let replacements: Vec<_> = mutants.iter().map(|m| m.replacement.as_str()).collect();
        assert!(replacements.contains(&"6"));
        assert!(replacements.contains(&"4"));
        assert!(replacements.contains(&"-5"));
        assert!(replacements.contains(&"0"));
        assert!(replacements.contains(&"1"));
    }

    #[test]
    fn string_is_replaced_with_empty_string() {
        let mutants = mutants_for(r#"local x = "hello""#, Box::new(LiteralMutator::new()));
        assert_eq!(mutants.len(), 1);
        assert_eq!(mutants[0].original, "\"hello\"");
        assert_eq!(mutants[0].replacement, "\"\"");
    }

    #[test]
    fn nil_is_replaced_with_false() {
        let mutants = mutants_for("local x = x == nil", Box::new(LiteralMutator::new()));
        let nil_mutant = mutants.iter().find(|m| m.original == "nil").unwrap();
        assert_eq!(nil_mutant.replacement, "false");
    }

    #[test]
    fn unary_minus_is_removed() {
        let mutants = mutants_for("local x = -y", Box::new(UnaryOperatorMutator::new()));
        assert_eq!(mutants.len(), 1);
        assert_eq!(mutants[0].original, "-y");
        assert_eq!(mutants[0].replacement, "y");
    }

    #[test]
    fn unary_not_is_removed() {
        let mutants = mutants_for("local x = not y", Box::new(UnaryOperatorMutator::new()));
        assert_eq!(mutants.len(), 1);
        assert_eq!(mutants[0].original, "not y");
        assert_eq!(mutants[0].replacement, "y");
    }

    #[test]
    fn unary_length_is_removed() {
        let mutants = mutants_for("local x = #y", Box::new(UnaryOperatorMutator::new()));
        assert_eq!(mutants.len(), 1);
        assert_eq!(mutants[0].original, "#y");
        assert_eq!(mutants[0].replacement, "y");
    }

    #[test]
    fn unary_bitwise_not_is_removed() {
        let mutants = mutants_for("local x = ~y", Box::new(UnaryOperatorMutator::new()));
        assert_eq!(mutants.len(), 1);
        assert_eq!(mutants[0].original, "~y");
        assert_eq!(mutants[0].replacement, "y");
    }

    #[test]
    fn every_mutant_carries_location_metadata() {
        let literal_mutants = mutants_for("local x = true", Box::new(LiteralMutator::new()));
        let unary_mutants = mutants_for("local x = -y", Box::new(UnaryOperatorMutator::new()));
        assert!(!literal_mutants.is_empty());
        assert!(!unary_mutants.is_empty());
        for m in literal_mutants.iter().chain(unary_mutants.iter()) {
            assert!(m.start_byte < m.end_byte);
            assert!(m.line > 0);
            assert!(m.column > 0);
            assert!(!m.replacement.is_empty());
        }
    }
}
