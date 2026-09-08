# lua-mutation-test

> ⚠️ **Work in Progress** ⚠️
>
> This project is in early development and is **not yet ready for production use**.
> APIs, CLI flags, and behavior may change at any time until a stable release is published.
> Contributions and feedback are welcome, but please expect rough edges.

A mutation testing tool for Lua, written in Rust.

`lua-mutation-test` parses Lua source code with [tree-sitter](https://tree-sitter.github.io/tree-sitter/)
and will eventually generate mutants, run your test suite against them, and report
mutation scores.

---

[![Rust](https://github.com/rcasia/lua-mutation-test/actions/workflows/rust.yml/badge.svg)](https://github.com/rcasia/lua-mutation-test/blob/main/.github/workflows/rust.yml)
[![GitHub Release](https://img.shields.io/github/v/release/rcasia/lua-mutation-test)](https://github.com/rcasia/lua-mutation-test/releases)
[![Docs](https://img.shields.io/badge/docs-gh--pages-blue)](https://rcasia.github.io/lua-mutation-test/)

## Features

### Current

- Parse Lua source code using a vendored [tree-sitter-lua](https://github.com/tree-sitter-grammars/tree-sitter-lua) grammar.
- Rust CLI scaffolding.

### Planned

- Generate mutants from Lua source code (e.g., relational operator swaps, arithmetic
  operator swaps, boolean literal flips).
- Run a given Lua test suite against each mutant.
- Compute and report mutation scores.
- Configurable mutation operators and ignore patterns.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) toolchain (latest stable recommended).
- [Lua](https://www.lua.org/download.html) (required in future versions for running
  tests against mutants).

## Setup

1. Clone the repository:

   ```bash
   git clone git@github.com:rcasia/lua-mutation-test.git
   cd lua-mutation-test
   ```

2. Fetch the vendored tree-sitter Lua grammar:

   ```bash
   ./scripts/fetch-tree-sitter-lua.sh
   ```

   The grammar is cloned into `tree-sitter-lua/` (gitignored).

3. Build the project:

   ```bash
   cargo build
   ```

4. Run the tests:

   ```bash
   cargo test
   ```

## Usage

Run mutation testing against a file or directory:

```bash
cargo run -- run <path-to-lua-file-or-directory>
```

Run with a custom test command and timeout:

```bash
cargo run -- run src --test-command 'busted' --timeout 30
```

List available mutation operators:

```bash
cargo run -- list-operators
```

Create a sample configuration file:

```bash
cargo run -- init
```

Once built, you can also run the binary directly:

```bash
./target/debug/lua-mutation-test run <path-to-lua-file-or-directory>
```

> Note: Full mutation-testing functionality is still being implemented. The current
> CLI provides the entry point and dispatch layer for upcoming workflows.

## Configuration

Create a `.lua-mutation-test.toml` (or `.lua-mutation-test.json`) file in your project root:

```toml
version = "1"
test_command = "busted"
timeout = 30
test_globs = ["*_spec.lua", "*_test.lua", "test_*.lua"]
source_globs = ["*.lua"]

[files]
exclude = ["*_test.lua", "*_spec.lua"]

[operators]
exclude = ["control_flow"]

[functions]
exclude = ["helpers_*"]
```

Pass an explicit config path with `--config`:

```bash
lua-mutation-test --config path/to/config.toml run src
```

CLI flags override configuration file values.

## Architecture

- **Rust CLI**: Entry point and orchestration.
- **tree-sitter Lua parser**: Vendored grammar used to parse Lua source into an AST.
- Future components will include mutant generation, test runner integration, and
  score reporting.

## Versioning

This project follows [ZeroVer](https://0ver.org/): all releases will remain in the
`0.x` range for the foreseeable future while the API and feature set stabilizes.

## Contributing

This project follows [trunk-based development](https://trunkbaseddevelopment.com/).
Work on short-lived branches or directly on `main`, keep commits small and focused,
and push to `origin/main` frequently.

## License

This project is licensed under the [Apache License 2.0](LICENSE). You may use,
modify, and distribute it freely, including for commercial purposes, subject to
the terms and conditions of the license.
