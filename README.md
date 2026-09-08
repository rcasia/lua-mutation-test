# lua-mutation-test

> ⚠️ **Work in Progress** ⚠️
>
> This project is in early development and is **not yet ready for production use**.
> APIs, CLI flags, and behavior may change at any time until a stable release is published.
> Contributions and feedback are welcome, but please expect rough edges.

A mutation testing tool for Lua, written in Rust.

`lua-mutation-test` parses Lua source code with [tree-sitter](https://tree-sitter.github.io/tree-sitter/),
generates mutants, runs your test suite against them, and reports mutation scores.

---

[![Rust](https://github.com/rcasia/lua-mutation-test/actions/workflows/rust.yml/badge.svg)](https://github.com/rcasia/lua-mutation-test/blob/main/.github/workflows/rust.yml)
[![GitHub Release](https://img.shields.io/github/v/release/rcasia/lua-mutation-test)](https://github.com/rcasia/lua-mutation-test/releases)
[![Docs](https://img.shields.io/badge/docs-gh--pages-blue)](https://rcasia.github.io/lua-mutation-test/)

## Features

### Current

- Parse Lua source code using a vendored [tree-sitter-lua](https://github.com/tree-sitter-grammars/tree-sitter-lua) grammar.
- Generate mutants from Lua source code (arithmetic, relational, logical, and control-flow operators).
- Run a given Lua test suite against each mutant with process isolation and timeouts.
- Compute and report mutation scores (summary, per-mutant, JSON, CTRF, and HTML).
- Static heuristics that detect likely-equivalent mutants (e.g., `x + 0`) before execution.
- Configurable mutation operators and ignore patterns.

### Planned

- Additional mutation operators.
- More equivalent-mutant heuristics.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) toolchain (latest stable recommended).
- [Lua](https://www.lua.org/download.html) (required for running tests against mutants and for the
  integration test suite).

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

5. Run the benchmarks:

   ```bash
   cargo bench
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

See the [CLI Reference](https://rcasia.github.io/lua-mutation-test/cli-reference/) for the
full list of commands and options.

Once built, you can also run the binary directly. The shorter alias `lmut` is available and preferred in examples; the full `lua-mutation-test` name works identically:

```bash
./target/debug/lmut run <path-to-lua-file-or-directory>
```

> Note: The project is a work in progress. APIs, CLI flags, and behavior may change.

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
lmut --config path/to/config.toml run src
```

CLI flags override configuration file values.

Generate reports after a run:

```bash
lmut run src --report-format json --report-output report.json
lmut run src --report-format ctrf --report-output ctrf-report.json
lmut run src --report-format html --report-output report.html
```

## Architecture

- **Rust CLI**: Entry point and orchestration.
- **tree-sitter Lua parser**: Vendored grammar used to parse Lua source into an AST.
- **Mutant generator**: Applies configurable mutation operators and detects likely-equivalent mutants with static heuristics.
- **Test runner**: Runs the test suite against each mutant in an isolated temporary copy of the project.
- **Reporter**: Computes mutation scores and emits summary, per-mutant, JSON, CTRF, or HTML reports.

See [docs/architecture.md](docs/architecture.md) for more details, including the
list of known equivalent-mutant patterns.

## Versioning

This project follows [ZeroVer](https://0ver.org/): all releases will remain in the
`0.x` range for the foreseeable future while the API and feature set stabilizes.

## Contributing

This project follows [trunk-based development](https://trunkbaseddevelopment.com/).
Work on short-lived branches or directly on `main`, keep commits small and focused,
and push to `origin/main` frequently.

## Pre-commit hooks

This project uses [pre-commit](https://pre-commit.com/) to run checks before each
commit. Install it with:

```bash
pip install pre-commit
pre-commit install
```

The configured hooks run file hygiene checks, `cargo fmt`, `cargo clippy`,
`cargo test`, and `mkdocs build`. Make sure you have the Rust toolchain and
MkDocs dependencies installed (`pip install -r docs/requirements.txt`) so all
hooks can run.

## License

This project is licensed under the [Apache License 2.0](LICENSE). You may use,
modify, and distribute it freely, including for commercial purposes, subject to
the terms and conditions of the license.
