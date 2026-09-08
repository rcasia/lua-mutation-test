## Why

Mutation operators must inspect AST nodes, find nodes by kind, and map byte ranges back to human-readable source locations. Reusable traversal and source-location utilities prevent every operator from reimplementing the same tree-walking logic.

## What Changes

- Add an `ast` module with cursor-based traversal helpers.
- Provide a pre-order tree walker and functions to collect nodes matching one or more kinds.
- Add a `position` module that maps byte offsets to line/column positions.
- Add a helper to extract the original source text for a node as a slice, avoiding unnecessary clones.
- Prefer iterators and borrowed data where possible.
- Add unit tests and public API documentation.

## Capabilities

### New Capabilities
- `ast-traversal`: AST traversal, node collection by kind, and source-location mapping.

### Modified Capabilities
<!-- No existing capabilities are modified by this change. -->

## Impact

- New `src/ast.rs` and `src/position.rs` modules.
- Mutation operator modules will depend on these helpers.
- Parser module supplies the `Tree` and source string consumed by the utilities.
- Test suite gains traversal and position-mapping tests.
