## 1. Setup

- [x] 1.1 Create the `src/parser.rs` module
- [x] 1.2 Define a `ParseError` type that can carry a file path and a byte offset

## 2. Parser Wrapper

- [x] 2.1 Implement a `Parser` wrapper that loads `tree_sitter_lua::LANGUAGE.into()`
- [x] 2.2 Implement `parse_source(source: &str) -> Result<Tree, ParseError>`
- [x] 2.3 Implement `parse_file(path: impl AsRef<Path>) -> Result<Tree, ParseError>` by reading the file and delegating to `parse_source`

## 3. Error Handling

- [x] 3.1 Build a line-start offset table from the source text
- [x] 3.2 Convert the error byte offset into 1-indexed line and byte-column values
- [x] 3.3 Include the file path in errors produced by `parse_file`

## 4. Tests

- [x] 4.1 Add a test for parsing valid Lua source
- [x] 4.2 Add a test for parsing invalid Lua source and verifying location info
- [x] 4.3 Add tests for Lua with comments, strings, and multiline strings
- [x] 4.4 Add a test for parsing a missing file
