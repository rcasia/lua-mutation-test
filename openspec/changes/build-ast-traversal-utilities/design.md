## Context

Once Lua source is parsed into a tree-sitter AST, mutation operators need to navigate nodes, filter by kind, and report source locations. tree-sitter provides a low-level cursor API; this change adds higher-level, reusable utilities on top of it.

## Goals / Non-Goals

**Goals:**
- Provide a pre-order AST walker.
- Provide helpers to collect nodes matching one or more kinds.
- Map byte offsets to line/column positions.
- Extract a node's original source text as a slice.
- Support both production code and tests with a documented public API.

**Non-Goals:**
- Implementing mutation logic or source rewriting.
- Building a full AST query language.
- Modifying the tree-sitter tree structure.

## Decisions

- **Expose traversal as an iterator**. A pre-order iterator built on `TreeCursor` is idiomatic in Rust and lets callers compose `filter`/`map` without manual cursor bookkeeping.
- **Collect-by-kind accepts a slice of kind strings**. A single helper `collect_nodes(tree, kinds)` covers both single-kind and multi-kind searches.
- **Position mapping uses a line-start offset table**. Given a source string, build a `Vec<usize>` of byte offsets where each line begins. Mapping an offset is then a binary search (or linear scan for small files) plus a column calculation.
- **Return 1-indexed line and byte-column positions**. This matches the values users see in editors and tree-sitter's `Point` row semantics.
- **Node text extraction returns `&str` via the source slice**. Because `Node::byte_range()` returns byte offsets, `&source[range]` gives the original text without cloning.

## Risks / Trade-offs

- **[Risk]** Rebuilding the line-start table for every position lookup is `O(n)`.  
  **Mitigation**: Build the table once per source string and pass it to mapping functions; expose a `PositionMap` type if needed.
- **[Risk]** Collecting all matching nodes eagerly allocates a `Vec`.  
  **Mitigation**: Provide both `collect_nodes` for convenience and the underlying iterator for lazy/efficient use cases.
- **[Risk]** Byte-column values may confuse users expecting UTF-8 character columns.  
  **Mitigation**: Document that positions are byte-based, consistent with tree-sitter.

## Open Questions

- Should the walker skip anonymous nodes by default? (Defer; default to visiting all nodes and add a filter helper later.)
- Should `collect_nodes` accept a predicate closure in addition to kind names? (Defer; kind-based collection covers the current need.)
