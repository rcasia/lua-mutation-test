//! AST traversal helpers built on tree-sitter.

use tree_sitter::{Node, Tree, TreeCursor};

/// Pre-order iterator over all nodes in a tree.
pub struct PreOrderIter<'tree> {
    cursor: TreeCursor<'tree>,
    started: bool,
}

impl<'tree> PreOrderIter<'tree> {
    fn new(tree: &'tree Tree) -> Self {
        Self {
            cursor: tree.root_node().walk(),
            started: false,
        }
    }
}

impl<'tree> Iterator for PreOrderIter<'tree> {
    type Item = Node<'tree>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.started {
            self.started = true;
            return Some(self.cursor.node());
        }

        if self.cursor.goto_first_child() {
            return Some(self.cursor.node());
        }

        loop {
            if self.cursor.goto_next_sibling() {
                return Some(self.cursor.node());
            }
            if !self.cursor.goto_parent() {
                return None;
            }
        }
    }
}

/// Returns an iterator that visits every node in `tree` in pre-order.
pub fn walk(tree: &Tree) -> PreOrderIter<'_> {
    PreOrderIter::new(tree)
}

/// Collects all nodes whose kind is contained in `kinds`.
///
/// `kinds` may contain one or more node kind strings.
pub fn collect_nodes<'tree>(tree: &'tree Tree, kinds: &[&str]) -> Vec<Node<'tree>> {
    walk(tree)
        .filter(|node| kinds.contains(&node.kind()))
        .collect()
}

/// Returns the original source text slice for `node`.
pub fn node_text<'source>(source: &'source str, node: Node<'_>) -> &'source str {
    &source[node.byte_range()]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Parser;

    #[test]
    fn walker_visits_all_nodes_in_pre_order() {
        let mut parser = Parser::new().unwrap();
        let tree = parser.parse_source("local x = 1").unwrap();
        let kinds: Vec<_> = walk(&tree).map(|n| n.kind().to_string()).collect();
        assert_eq!(kinds[0], "chunk");
        assert!(kinds.len() > 1);
        assert!(kinds.contains(&"identifier".to_string()));
        assert!(kinds.contains(&"number".to_string()));
    }

    #[test]
    fn collect_nodes_filters_by_single_kind() {
        let mut parser = Parser::new().unwrap();
        let tree = parser.parse_source("local x = 1 + 2").unwrap();
        let numbers = collect_nodes(&tree, &["number"]);
        assert_eq!(numbers.len(), 2);
        assert!(numbers.iter().all(|n| n.kind() == "number"));
    }

    #[test]
    fn collect_nodes_filters_by_multiple_kinds() {
        let mut parser = Parser::new().unwrap();
        let tree = parser.parse_source("local x = 1 + 'two'").unwrap();
        let nodes = collect_nodes(&tree, &["number", "string"]);
        assert_eq!(nodes.len(), 2);
    }

    #[test]
    fn node_text_returns_source_slice() {
        let mut parser = Parser::new().unwrap();
        let source = "abc = 1";
        let tree = parser.parse_source(source).unwrap();
        let identifiers = collect_nodes(&tree, &["identifier"]);
        assert_eq!(node_text(source, identifiers[0]), "abc");
    }
}
