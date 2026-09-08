# lua-source-parser Specification

## Purpose
TBD - created by archiving change parse-lua-source-files. Update Purpose after archive.
## Requirements
### Requirement: Initialize parser with vendored grammar
The system SHALL load the Lua language using `tree_sitter_lua::LANGUAGE.into()`.

#### Scenario: Successful parser creation
- **WHEN** a parser wrapper is instantiated
- **THEN** the underlying tree-sitter parser is configured with the tree-sitter-lua language

### Requirement: Parse a Lua file
The system SHALL parse a `.lua` file and return a tree-sitter `Tree` whose root node represents the source chunk.

#### Scenario: Valid Lua file
- **WHEN** `parse_file` reads a valid Lua file
- **THEN** it returns a `Tree` whose root node kind is `chunk`

#### Scenario: File does not exist
- **WHEN** `parse_file` reads a path that does not exist
- **THEN** it returns a `ParseError` indicating the missing file path

### Requirement: Parse Lua source string
The system SHALL parse an in-memory Lua source string and return a tree-sitter `Tree`.

#### Scenario: Valid source string
- **WHEN** `parse_source` receives valid Lua source
- **THEN** it returns a `Tree` whose root node kind is `chunk`

#### Scenario: Invalid source string
- **WHEN** `parse_source` receives invalid Lua source
- **THEN** it returns a `ParseError` with location information

### Requirement: Report parse errors with location
A `ParseError` SHALL include the file path when parsing a file, and the line and column of the failure.

#### Scenario: Syntax error in file
- **WHEN** a file contains a Lua syntax error
- **THEN** the returned `ParseError` includes the file path and the line/column of the error

### Requirement: Handle edge cases
The system SHALL correctly parse Lua source containing comments, strings, and multiline strings.

#### Scenario: Multiline string
- **WHEN** the parser receives Lua source with a multiline string literal
- **THEN** it returns a valid `Tree` covering the entire source

#### Scenario: Source with comments
- **WHEN** the parser receives Lua source with line and block comments
- **THEN** it returns a valid `Tree` and preserves comment nodes in the AST

