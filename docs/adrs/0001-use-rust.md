# ADR-001: Use Rust as the Implementation Language

## Status

Accepted

## Context

We need to choose an implementation language for `lua-mutation-test`. The tool must:

- Parse Lua source files quickly and reliably.
- Spawn many test processes efficiently (mutation testing is CPU- and I/O-bound).
- Be distributable as a single binary.
- Be maintainable by the core contributor team.

## Decision Drivers

- Performance for parsing and parallel test execution.
- Reliable binary distribution without runtime dependencies.
- Strong tooling and ecosystem for CLI applications.
- Team familiarity and willingness to maintain the codebase.

## Considered Options

### Option 1: Rust

- **Pros**: Fast, safe concurrency, single static binary, strong CLI ecosystem (clap),
  good tree-sitter bindings, cross-compilation support.
- **Cons**: Steeper learning curve for contributors not familiar with the borrow checker;
  slower compile times.

### Option 2: Lua

- **Pros**: Same language as the code under test; easy to inspect Lua internals; fast to
  prototype.
- **Cons**: Slower for CPU-intensive work; harder to distribute as a single binary;
  weaker static analysis and tooling for large projects.

### Option 3: Python

- **Pros**: Large ecosystem, rapid development, good test tooling.
- **Cons**: Requires Python runtime; slower execution; harder to ship as a standalone CLI.

## Decision

We will implement `lua-mutation-test` in **Rust**.

## Rationale

Rust provides the best balance of performance, reliability, and distribution simplicity
for a CLI mutation testing tool. The tree-sitter ecosystem has mature Rust bindings, and
Rust's concurrency model is well suited to running mutants in parallel.

## Consequences

### Positive

- Fast parsing and mutant execution.
- Single binary distribution.
- Strong type safety and concurrency guarantees.

### Negative

- Contributors may need to learn Rust.
- Compile times are longer than interpreted languages.

## References

- [Rust Programming Language](https://www.rust-lang.org/)
- [tree-sitter Rust bindings](https://github.com/tree-sitter/tree-sitter/tree/master/lib/binding_rust)
