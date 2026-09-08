//! Control-flow mutation operators.
//!
//! Generates mutants for conditional branches, loops, and return statements.

use crate::ast::{collect_nodes, node_text};
use crate::mutant::{CandidateMutant, Mutator};
use crate::parser::Parser;
use tree_sitter::{Node, Tree};

/// Mutates Lua control-flow constructs.
///
/// Branch mutations:
/// * Invert the `if` condition (`if cond then` -> `if not (cond) then`).
/// * Swap the `then` and `else` bodies.
/// * Remove the `else` branch.
/// * Remove each `elseif` branch.
///
/// Loop mutations:
/// * Replace a `while` condition with `false` or `true`.
/// * Increment/decrement numeric `for` start and end boundaries.
/// * Invert a `repeat ... until` condition.
///
/// Return mutations:
/// * Remove the `return` statement.
/// * Replace returned expressions with `nil`.
#[derive(Debug, Clone, Copy, Default)]
pub struct ControlFlowMutator;

impl ControlFlowMutator {
    /// Creates a new control-flow mutator.
    pub fn new() -> Self {
        Self
    }
}

impl Mutator for ControlFlowMutator {
    fn id(&self) -> &'static str {
        "control_flow"
    }

    fn generate(&self, source: &str, tree: &Tree) -> Vec<CandidateMutant> {
        let mut parser = Parser::new().expect("failed to create Lua parser");
        let mut candidates = Vec::new();

        for node in collect_nodes(
            tree,
            &[
                "if_statement",
                "while_statement",
                "for_statement",
                "repeat_statement",
                "return_statement",
            ],
        ) {
            match node.kind() {
                "if_statement" => mutate_if(source, node, &mut candidates),
                "while_statement" => mutate_while(source, node, &mut candidates),
                "for_statement" => mutate_for(source, node, &mut candidates),
                "repeat_statement" => mutate_repeat(source, node, &mut candidates),
                "return_statement" => mutate_return(source, node, &mut candidates),
                _ => {}
            }
        }

        candidates
            .into_iter()
            .filter(|candidate| {
                let mutated = replace_range(
                    source,
                    candidate.start_byte,
                    candidate.end_byte,
                    &candidate.replacement,
                );
                parser.parse_source(&mutated).is_ok()
            })
            .collect()
    }
}

fn mutate_if(source: &str, node: Node<'_>, candidates: &mut Vec<CandidateMutant>) {
    let condition = match node.child_by_field_name("condition") {
        Some(c) => c,
        None => return,
    };

    // Invert the if condition.
    candidates.push(CandidateMutant {
        start_byte: condition.start_byte(),
        end_byte: condition.end_byte(),
        replacement: format!("not ({})", node_text(source, condition)),
    });

    let consequence = match node.child_by_field_name("consequence") {
        Some(c) => c,
        None => return,
    };

    let alternatives: Vec<_> = {
        let mut cursor = node.walk();
        node.children(&mut cursor).collect()
    };
    let elseif_statements: Vec<_> = alternatives
        .iter()
        .filter(|n| n.kind() == "elseif_statement")
        .copied()
        .collect();
    let else_statement = alternatives
        .iter()
        .find(|n| n.kind() == "else_statement")
        .copied();

    // Swap then/else bodies if an else branch exists.
    if let Some(else_stmt) = else_statement {
        if let Some(else_body) = else_stmt.child_by_field_name("body") {
            let replacement = swap_ranges(
                source,
                node.start_byte(),
                consequence.start_byte(),
                consequence.end_byte(),
                else_body.start_byte(),
                else_body.end_byte(),
                node.end_byte(),
            );
            candidates.push(CandidateMutant {
                start_byte: node.start_byte(),
                end_byte: node.end_byte(),
                replacement,
            });
        }
    }

    // Remove the else branch.
    if let Some(else_stmt) = else_statement {
        candidates.push(CandidateMutant {
            start_byte: else_stmt.start_byte(),
            end_byte: else_stmt.end_byte(),
            replacement: String::new(),
        });
    }

    // Remove each elseif branch.
    for elseif in &elseif_statements {
        candidates.push(CandidateMutant {
            start_byte: elseif.start_byte(),
            end_byte: elseif.end_byte(),
            replacement: String::new(),
        });
    }
}

fn mutate_while(_source: &str, node: Node<'_>, candidates: &mut Vec<CandidateMutant>) {
    let Some(condition) = node.child_by_field_name("condition") else {
        return;
    };

    for replacement in ["false", "true"] {
        candidates.push(CandidateMutant {
            start_byte: condition.start_byte(),
            end_byte: condition.end_byte(),
            replacement: replacement.to_string(),
        });
    }
}

fn mutate_for(source: &str, node: Node<'_>, candidates: &mut Vec<CandidateMutant>) {
    let Some(clause) = node.child_by_field_name("clause") else {
        return;
    };
    if clause.kind() != "for_numeric_clause" {
        return;
    }

    let Some(start) = clause.child_by_field_name("start") else {
        return;
    };
    let Some(end) = clause.child_by_field_name("end") else {
        return;
    };

    let start_text = node_text(source, start);
    let end_text = node_text(source, end);

    for (field, text) in [(start, start_text), (end, end_text)] {
        if let Ok(value) = text.parse::<i64>() {
            for delta in [-1, 1] {
                candidates.push(CandidateMutant {
                    start_byte: field.start_byte(),
                    end_byte: field.end_byte(),
                    replacement: (value + delta).to_string(),
                });
            }
        }
    }
}

fn mutate_repeat(source: &str, node: Node<'_>, candidates: &mut Vec<CandidateMutant>) {
    let Some(condition) = node.child_by_field_name("condition") else {
        return;
    };

    candidates.push(CandidateMutant {
        start_byte: condition.start_byte(),
        end_byte: condition.end_byte(),
        replacement: format!("not ({})", node_text(source, condition)),
    });
}

fn mutate_return(_source: &str, node: Node<'_>, candidates: &mut Vec<CandidateMutant>) {
    // Remove the return statement entirely.
    candidates.push(CandidateMutant {
        start_byte: node.start_byte(),
        end_byte: node.end_byte(),
        replacement: String::new(),
    });

    // Replace returned expressions with nil.
    let mut cursor = node.walk();
    let children: Vec<_> = node.children(&mut cursor).collect();
    if let Some(expr_list) = children.iter().find(|n| n.kind() == "expression_list") {
        candidates.push(CandidateMutant {
            start_byte: expr_list.start_byte(),
            end_byte: expr_list.end_byte(),
            replacement: "nil".to_string(),
        });
    }
}

fn swap_ranges(
    source: &str,
    outer_start: usize,
    a_start: usize,
    a_end: usize,
    b_start: usize,
    b_end: usize,
    outer_end: usize,
) -> String {
    let before_a = &source[outer_start..a_start];
    let a = &source[a_start..a_end];
    let between = &source[a_end..b_start];
    let b = &source[b_start..b_end];
    let after_b = &source[b_end..outer_end];
    format!("{}{}{}{}{}", before_a, b, between, a, after_b)
}

fn replace_range(source: &str, start: usize, end: usize, replacement: &str) -> String {
    let mut result = String::with_capacity(source.len() - (end - start) + replacement.len());
    result.push_str(&source[..start]);
    result.push_str(replacement);
    result.push_str(&source[end..]);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mutant::MutantGenerator;
    use crate::parser::Parser;

    fn mutants_for(source: &str) -> Vec<crate::mutant::Mutant> {
        let mut parser = Parser::new().unwrap();
        let tree = parser.parse_source(source).unwrap();
        let generator = MutantGenerator::new(vec![Box::new(ControlFlowMutator)]);
        generator.generate("file.lua", source, &tree)
    }

    fn replacement_texts(source: &str) -> Vec<String> {
        mutants_for(source)
            .into_iter()
            .map(|m| m.replacement)
            .collect()
    }

    #[test]
    fn inverts_if_condition() {
        let mutants = mutants_for("if x > 0 then print('positive') end");
        assert!(mutants.iter().any(|m| m.replacement == "not (x > 0)"));
    }

    #[test]
    fn swaps_then_and_else_bodies() {
        let mutants = mutants_for("if x then A() else B() end");
        let swapped = mutants
            .iter()
            .find(|m| m.start_byte == 0 && m.replacement == "if x then B() else A() end");
        assert!(swapped.is_some(), "expected swapped branch mutant");
    }

    #[test]
    fn removes_else_branch() {
        let source = "if x then A() else B() end";
        let mutants = mutants_for(source);
        let removed = mutants.iter().find(|m| {
            m.replacement.is_empty()
                && replace_range(source, m.start_byte, m.end_byte, &m.replacement)
                    == "if x then A()  end"
        });
        assert!(removed.is_some(), "expected else removal mutant");
    }

    #[test]
    fn removes_elseif_branch() {
        let source = "if x then A() elseif y then B() end";
        let mutants = mutants_for(source);
        let removed = mutants.iter().find(|m| {
            m.replacement.is_empty()
                && replace_range(source, m.start_byte, m.end_byte, &m.replacement)
                    == "if x then A()  end"
        });
        assert!(removed.is_some(), "expected elseif removal mutant");
    }

    #[test]
    fn mutates_while_condition_to_false_and_true() {
        let replacements = replacement_texts("while cond do work() end");
        assert!(replacements.contains(&"false".to_string()));
        assert!(replacements.contains(&"true".to_string()));
    }

    #[test]
    fn mutates_numeric_for_boundaries() {
        let replacements = replacement_texts("for i = 1, 10 do work() end");
        assert!(replacements.contains(&"2".to_string()));
        assert!(replacements.contains(&"0".to_string()));
        assert!(replacements.contains(&"11".to_string()));
        assert!(replacements.contains(&"9".to_string()));
    }

    #[test]
    fn inverts_repeat_until_condition() {
        let mutants = mutants_for("repeat work() until done");
        assert!(mutants.iter().any(|m| m.replacement == "not (done)"));
    }

    #[test]
    fn removes_return_statement() {
        let mutants = mutants_for("local function f() return x + 1 end");
        assert!(mutants.iter().any(|m| m.replacement.is_empty()));
    }

    #[test]
    fn replaces_return_expression_with_nil() {
        let mutants = mutants_for("local function f() return x + 1 end");
        assert!(mutants.iter().any(|m| m.replacement == "nil"));
    }

    #[test]
    fn all_generated_mutants_are_valid_lua() {
        let source = r#"
            local function f(x)
                if x > 0 then
                    return x
                elseif x < 0 then
                    return -x
                else
                    return 0
                end
            end

            local function g(n)
                local sum = 0
                for i = 1, n do
                    sum = sum + i
                end
                while sum > 0 do
                    sum = sum - 1
                end
                repeat
                    n = n - 1
                until n <= 0
                return sum
            end
        "#;

        let mut parser = Parser::new().unwrap();
        let tree = parser.parse_source(source).unwrap();
        let mutator = ControlFlowMutator;
        let candidates = mutator.generate(source, &tree);

        for candidate in &candidates {
            let mutated = replace_range(
                source,
                candidate.start_byte,
                candidate.end_byte,
                &candidate.replacement,
            );
            assert!(
                parser.parse_source(&mutated).is_ok(),
                "invalid mutant: {mutated}"
            );
        }

        assert!(!candidates.is_empty());
    }

    #[test]
    fn handles_nested_control_flow() {
        let mutants = mutants_for("if x then if y then A() else B() end end");
        assert!(!mutants.is_empty());
        let operators: Vec<_> = mutants.iter().map(|m| m.operator.clone()).collect();
        assert!(operators.iter().all(|op| op == "control_flow"));
    }

    #[test]
    fn skips_empty_else_swap_when_no_else() {
        let mutants = mutants_for("if x then A() end");
        assert!(!mutants
            .iter()
            .any(|m| m.replacement == "if x then  else A() end"));
    }
}
