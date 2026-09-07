## ADDED Requirements

### Requirement: Walk the AST
The system SHALL provide a helper that walks a tree-sitter AST in pre-order and yields every node.

#### Scenario: Visit all nodes
- **WHEN** the walker traverses a parsed Lua tree
- **THEN** it yields every node exactly once in pre-order

### Requirement: Collect nodes by kind
The system SHALL provide functions to collect nodes whose kind matches one or more provided kind strings.

#### Scenario: Collect single kind
- **WHEN** the helper receives the kind `"binary_expression"`
- **THEN** it returns all nodes in the tree with kind `binary_expression`

#### Scenario: Collect multiple kinds
- **WHEN** the helper receives the kinds `["number", "string"]`
- **THEN** it returns all nodes whose kind is either `number` or `string`

### Requirement: Map byte offset to position
The system SHALL map a byte offset in the source string to a 1-indexed line and byte-column position.

#### Scenario: Offset in first line
- **WHEN** the helper maps offset 5 in the source `"if x then\nend"`
- **THEN** it returns line 1, column 6

#### Scenario: Offset in second line
- **WHEN** the helper maps offset 10 in the source `"if x then\nend"`
- **THEN** it returns line 2, column 1

### Requirement: Extract node source text
The system SHALL return the original source text for a node as a slice of the source string.

#### Scenario: Extract identifier text
- **WHEN** the helper receives a node spanning bytes 0..3 in the source `"abc = 1"`
- **THEN** it returns `"abc"`

#### Scenario: Extract binary expression text
- **WHEN** the helper receives a node spanning bytes 0..9 in the source `"a + b * c"`
- **THEN** it returns the corresponding substring without cloning the source
