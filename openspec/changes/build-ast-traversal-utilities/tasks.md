## 1. Setup

- [ ] 1.1 Create the `src/ast.rs` module
- [ ] 1.2 Create the `src/position.rs` module

## 2. AST Traversal

- [ ] 2.1 Implement a pre-order tree walker using `TreeCursor`
- [ ] 2.2 Implement `collect_nodes` that filters by a single node kind
- [ ] 2.3 Extend `collect_nodes` to accept multiple node kinds

## 3. Source Location Mapping

- [ ] 3.1 Build a line-start offset table from a source string
- [ ] 3.2 Implement `byte_offset_to_position` returning 1-indexed line and byte-column

## 4. Helpers, Tests & Documentation

- [ ] 4.1 Implement a helper that returns the source text slice for a given node
- [ ] 4.2 Add unit tests for the walker, kind collection, position mapping, and text extraction
- [ ] 4.3 Add rustdoc comments to the public API
