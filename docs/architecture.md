# Architecture

This page describes the high-level architecture of `lua-mutation-test`.

## Overview

`lua-mutation-test` is a command-line tool written in Rust. It reads Lua source files,
generates mutants, runs a Lua test suite against each mutant, and reports mutation
scores.

## Components

```text
+----------------+      +-------------------+      +------------------+
| CLI / Config   |----->| Mutant Generator  |----->| Mutant Runner    |
+----------------+      +-------------------+      +------------------+
                                |                           |
                                v                           v
                       +----------------+          +------------------+
                       | tree-sitter    |          | Lua test suite   |
                       | Lua parser     |          | (busted/luaunit/ |
                       +----------------+          | custom command)  |
                                                   +------------------+
                                |
                                v
                       +------------------+
                       | Reporter         |
                       | (CLI/JSON/HTML)  |
                       +------------------+
```

### CLI / Config

- Parses command-line arguments and configuration files.
- Selects files, operators, and test runner.
- Entry point for `run`, `list-mutants`, `list-operators`, and `init` subcommands.
- See the [CLI Reference](cli-reference.md) for the complete command documentation.

### Lua Parser

- Uses a vendored [tree-sitter-lua](https://github.com/tree-sitter-grammars/tree-sitter-lua)
  grammar to parse Lua source into an AST.
- Provides AST traversal utilities and source-location mapping.

### Mutant Generator

- Applies configured mutation operators to AST nodes.
- Produces one or more mutants per mutation point.
- Deduplicates mutants and validates that mutated sources are syntactically valid.

### Mutant Runner

- Writes each mutant to a temporary file.
- Runs the project's test suite in an isolated process.
- Enforces timeouts and captures exit codes and output.

### Reporter

- Aggregates results into mutation scores.
- Generates CLI summaries, JSON reports, and HTML reports.

## Data flow

1. The CLI discovers Lua source files and test files based on configuration.
2. The parser produces an AST for each source file.
3. The mutant generator walks the AST and creates mutants.
4. The runner executes tests against each mutant.
5. The reporter categorizes mutants (killed, survived, timed out, error) and computes scores.

## Technology choices

See the [Architecture Decision Records](adrs/index.md) for the reasoning behind the
main technology and process choices.
