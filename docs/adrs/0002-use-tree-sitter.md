# ADR-002: Use tree-sitter for Lua Parsing

## Status

Accepted

## Context

The tool needs a robust way to parse Lua source code into an AST so it can identify
mutation points (operators, literals, control-flow nodes) and apply source-level
changes without breaking syntax.

## Decision Drivers

- Correct and up-to-date Lua grammar support.
- Ability to map AST nodes back to source byte ranges for precise edits.
- Incremental parsing support for watch mode (future).
- Mature bindings for Rust.

## Considered Options

### Option 1: tree-sitter

- **Pros**: Incremental parser, error-resilient, extensive grammar ecosystem, excellent
  Rust bindings, maps nodes to byte ranges.
- **Cons**: Requires compiling a C parser; grammars are separate repositories.

### Option 2: Hand-written Lua parser in Rust

- **Pros**: Full control over AST shape; no external C dependencies.
- **Cons**: High implementation cost; risk of parser bugs; maintenance burden for Lua
  syntax updates.

### Option 3: Existing Lua parser crate (e.g., `full_moon` for Rust)

- **Pros**: Pure Rust, easy to integrate.
- **Cons**: Less mature than tree-sitter; smaller community; may not expose all
  source-location details needed for mutation.

## Decision

We will use **tree-sitter** with the official **tree-sitter-lua** grammar to parse Lua.

## Rationale

tree-sitter is battle-tested, provides precise source locations, and has an active
community maintaining Lua grammar. The incremental parsing capability will also be
valuable for future watch-mode and incremental mutation testing features.

## Consequences

### Positive

- Accurate AST with byte-range metadata.
- Error-resilient parsing.
- Incremental parsing ready for future features.

### Negative

- Build depends on a C parser.
- Need to manage the grammar as a dependency (see ADR-003).

## References

- [tree-sitter](https://tree-sitter.github.io/tree-sitter/)
- [tree-sitter-lua grammar](https://github.com/tree-sitter-grammars/tree-sitter-lua)
