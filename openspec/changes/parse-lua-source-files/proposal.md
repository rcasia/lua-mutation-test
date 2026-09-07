## Why

All mutation operators need a parsed representation of Lua source code. Wrapping the vendored `tree-sitter-lua` grammar in a small, testable parser module provides the stable foundation every downstream component will use.

## What Changes

- Create a `parser` module that wraps tree-sitter parser initialization.
- Load the Lua language using `tree_sitter_lua::LANGUAGE.into()`.
- Implement `parse_file(path) -> Result<Tree, ParseError>` for parsing a `.lua` file.
- Implement `parse_source(source) -> Result<Tree, ParseError>` for parsing an in-memory string.
- Introduce a `ParseError` type that carries the file path (when applicable) and line/column information.
- Add unit tests covering valid Lua, invalid Lua, missing files, and edge cases such as comments, strings, and multiline strings.

## Capabilities

### New Capabilities
- `lua-source-parser`: parse Lua files and in-memory source strings into tree-sitter ASTs.

### Modified Capabilities
<!-- No existing capabilities are modified by this change. -->

## Impact

- New `src/parser.rs` module and `ParseError` type.
- `src/main.rs` will eventually delegate parsing to the new module instead of inline parser setup.
- Test suite gains parser-focused tests.
- No changes to mutation operators or reporting.
