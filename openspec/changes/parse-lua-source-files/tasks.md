## 1. Setup

- [ ] 1.1 Create the `src/parser.rs` module
- [ ] 1.2 Define a `ParseError` type that can carry a file path and a byte offset

## 2. Parser Wrapper

- [ ] 2.1 Implement a `Parser` wrapper that loads `tree_sitter_lua::LANGUAGE.into()`
- [ ] 2.2 Implement `parse_source(source: &str) -> Result<Tree, ParseError>`
- [ ] 2.3 Implement `parse_file(path: impl AsRef<Path>) -> Result<Tree, ParseError>` by reading the file and delegating to `parse_source`

## 3. Error Handling

- [ ] 3.1 Build a line-start offset table from the source text
- [ ] 3.2 Convert the error byte offset into 1-indexed line and byte-column values
- [ ] 3.3 Include the file path in errors produced by `parse_file`

## 4. Tests

- [ ] 4.1 Add a test for parsing valid Lua source
- [ ] 4.2 Add a test for parsing invalid Lua source and verifying location info
- [ ] 4.3 Add tests for Lua with comments, strings, and multiline strings
- [ ] 4.4 Add a test for parsing a missing file
