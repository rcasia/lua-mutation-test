use std::path::Path;
use tree_sitter::{Parser as TSParser, Tree};

/// Error returned when parsing Lua source fails.
#[derive(Debug)]
pub struct ParseError {
    pub path: Option<std::path::PathBuf>,
    pub source: String,
    pub offset: usize,
}

impl ParseError {
    /// Returns the 1-indexed line and byte-column of the error.
    pub fn location(&self) -> (usize, usize) {
        byte_offset_to_line_column(&self.source, self.offset)
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (line, column) = self.location();
        if let Some(path) = &self.path {
            write!(
                f,
                "parse error at {}:{}:{} (byte offset {})",
                path.display(),
                line,
                column,
                self.offset
            )
        } else {
            write!(
                f,
                "parse error at {}:{} (byte offset {})",
                line, column, self.offset
            )
        }
    }
}

impl std::error::Error for ParseError {}

/// Wrapper around the tree-sitter parser configured for Lua.
pub struct Parser {
    parser: TSParser,
}

impl Parser {
    /// Creates a new parser configured with the vendored Lua language.
    pub fn new() -> Result<Self, tree_sitter::LanguageError> {
        let mut parser = TSParser::new();
        parser.set_language(&tree_sitter_lua::LANGUAGE.into())?;
        Ok(Self { parser })
    }

    /// Parses a Lua source string.
    pub fn parse_source(&mut self, source: &str) -> Result<Tree, ParseError> {
        let tree = self.parser.parse(source, None).ok_or_else(|| ParseError {
            path: None,
            source: source.to_string(),
            offset: 0,
        })?;

        if let Some(error_node) = find_first_error_node(tree.root_node()) {
            return Err(ParseError {
                path: None,
                source: source.to_string(),
                offset: error_node.start_byte(),
            });
        }

        Ok(tree)
    }

    /// Reads a Lua file and parses its contents.
    pub fn parse_file(&mut self, path: impl AsRef<Path>) -> Result<Tree, ParseError> {
        let path = path.as_ref();
        let source = std::fs::read_to_string(path).map_err(|_| ParseError {
            path: Some(path.to_path_buf()),
            source: String::new(),
            offset: 0,
        })?;

        self.parse_source(&source).map_err(|mut err| {
            err.path = Some(path.to_path_buf());
            err
        })
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new().expect("failed to initialize Lua parser")
    }
}

fn find_first_error_node(node: tree_sitter::Node) -> Option<tree_sitter::Node> {
    if node.kind() == "ERROR" {
        return Some(node);
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if let Some(error) = find_first_error_node(child) {
            return Some(error);
        }
    }
    None
}

fn build_line_start_table(source: &str) -> Vec<usize> {
    let mut offsets = vec![0];
    for (i, c) in source.char_indices() {
        if c == '\n' {
            offsets.push(i + 1);
        }
    }
    offsets
}

fn byte_offset_to_line_column(source: &str, offset: usize) -> (usize, usize) {
    let line_starts = build_line_start_table(source);
    let line = line_starts.partition_point(|&start| start <= offset);
    let line_start = line_starts[line - 1];
    let column = offset - line_start + 1;
    (line, column)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_lua_source() {
        let mut parser = Parser::new().unwrap();
        let source = "if a == b then return true end";
        let tree = parser.parse_source(source).unwrap();
        assert_eq!(tree.root_node().kind(), "chunk");
    }

    #[test]
    fn reports_error_location_for_invalid_lua() {
        let mut parser = Parser::new().unwrap();
        let source = "if a == then return true end";
        let err = parser.parse_source(source).expect_err("should fail");
        assert_eq!(err.path, None);
        let (line, column) = err.location();
        assert_eq!(line, 1);
        assert!(column > 0 && column <= source.len());
    }

    #[test]
    fn parses_lua_with_comments_strings_and_multiline_strings() {
        let mut parser = Parser::new().unwrap();
        let source = r#"
            -- line comment
            local a = "hello"
            local b = [[
                multiline
                string
            ]]
            --[[ block comment ]]
            return a, b
        "#;
        let tree = parser.parse_source(source).unwrap();
        assert_eq!(tree.root_node().kind(), "chunk");
    }

    #[test]
    fn reports_missing_file_error() {
        let mut parser = Parser::new().unwrap();
        let err = parser
            .parse_file("does-not-exist.lua")
            .expect_err("should fail");
        assert_eq!(
            err.path,
            Some(std::path::PathBuf::from("does-not-exist.lua"))
        );
    }
}
