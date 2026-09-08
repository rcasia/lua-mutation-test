//! Heuristic detection of likely-equivalent mutants.
//!
//! Perfect equivalence detection is undecidable, but lightweight static rules
//! catch common arithmetic identities such as `x + 0` or `x * 1` before they
//! are executed.

use crate::mutant::CandidateMutant;
use tree_sitter::Tree;

/// Classifies a candidate mutant as likely equivalent.
///
/// Returns `Some(reason)` when the mutation is detected as an obvious
/// arithmetic identity, or `None` when the mutant should be executed normally.
pub fn classify(source: &str, tree: &Tree, mutant: &CandidateMutant) -> Option<String> {
    let node = tree
        .root_node()
        .descendant_for_byte_range(mutant.start_byte, mutant.end_byte)?;
    let parent = node.parent()?;
    if parent.kind() != "binary_expression" {
        return None;
    }

    let op_text = &source[node.byte_range()];
    let left = parent.child_by_field_name("left")?;
    let right = parent.child_by_field_name("right")?;
    let left_text = source[left.byte_range()].trim();
    let right_text = source[right.byte_range()].trim();

    match (op_text, mutant.replacement.as_str()) {
        // x + 0  ->  x - 0
        // 0 + x  ->  0 - x
        ("+", "-") if is_zero(left_text) || is_zero(right_text) => {
            Some("arithmetic identity: adding or subtracting zero".to_string())
        }
        // x - 0  ->  x + 0
        ("-", "+") if is_zero(right_text) => {
            Some("arithmetic identity: subtracting or adding zero".to_string())
        }
        // x * 1  ->  x / 1  or  x // 1
        // 1 * x  ->  1 / x  or  1 // x
        ("*", "/" | "//") if is_one(left_text) || is_one(right_text) => {
            Some("arithmetic identity: multiplying or dividing by one".to_string())
        }
        // x / 1  ->  x * 1
        // x // 1 ->  x * 1
        ("/" | "//", "*") if is_one(right_text) => {
            Some("arithmetic identity: dividing or multiplying by one".to_string())
        }
        _ => None,
    }
}

fn is_zero(text: &str) -> bool {
    text == "0" || text == "0.0"
}

fn is_one(text: &str) -> bool {
    text == "1" || text == "1.0"
}

#[cfg(test)]
mod tests {
    use crate::mutant::MutantGenerator;
    use crate::operators::arithmetic::ArithmeticOperatorMutator;
    use crate::parser::Parser;

    fn classify_source(source: &str) -> Vec<(String, String, String)> {
        let mut parser = Parser::new().unwrap();
        let tree = parser.parse_source(source).unwrap();
        let generator = MutantGenerator::new(vec![Box::new(ArithmeticOperatorMutator)]);
        generator
            .generate_with_equivalents("file.lua", source, &tree)
            .1
            .into_iter()
            .map(|m| (m.original, m.replacement, m.equivalent_reason.unwrap()))
            .collect()
    }

    #[test]
    fn flags_plus_to_minus_with_zero_operand() {
        let equivalents = classify_source("local x = a + 0");
        assert_eq!(equivalents.len(), 1);
        assert_eq!(equivalents[0].0, "+");
        assert_eq!(equivalents[0].1, "-");
        assert!(equivalents[0].2.contains("zero"));
    }

    #[test]
    fn flags_minus_to_plus_with_zero_operand() {
        let equivalents = classify_source("local x = a - 0");
        assert_eq!(equivalents.len(), 1);
        assert_eq!(equivalents[0].0, "-");
        assert_eq!(equivalents[0].1, "+");
        assert!(equivalents[0].2.contains("zero"));
    }

    #[test]
    fn flags_multiply_to_divide_with_one_operand() {
        let equivalents = classify_source("local x = a * 1");
        assert_eq!(equivalents.len(), 1);
        assert_eq!(equivalents[0].0, "*");
        assert_eq!(equivalents[0].1, "/");
        assert!(equivalents[0].2.contains("one"));
    }

    #[test]
    fn flags_divide_to_multiply_with_one_operand() {
        let equivalents = classify_source("local x = a / 1");
        assert_eq!(equivalents.len(), 1);
        assert_eq!(equivalents[0].0, "/");
        assert_eq!(equivalents[0].1, "*");
        assert!(equivalents[0].2.contains("one"));
    }

    #[test]
    fn does_not_flag_non_identity_mutations() {
        let mut parser = Parser::new().unwrap();
        let tree = parser.parse_source("local x = a + b").unwrap();
        let generator = MutantGenerator::new(vec![Box::new(ArithmeticOperatorMutator)]);
        let (_, equivalents) =
            generator.generate_with_equivalents("file.lua", "local x = a + b", &tree);
        assert!(equivalents.is_empty());
    }
}
