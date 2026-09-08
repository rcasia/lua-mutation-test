# Getting Started

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) toolchain (latest stable recommended).
- [Lua](https://www.lua.org/download.html) (required in future versions for running tests against mutants).

## Clone the repository

```bash
git clone git@github.com:rcasia/lua-mutation-test.git
cd lua-mutation-test
```

## Fetch the vendored grammar

The Lua parser grammar is vendored in `tree-sitter-lua/` (gitignored). Fetch it with:

```bash
./scripts/fetch-tree-sitter-lua.sh
```

## Build

```bash
cargo build
```

## Run tests

```bash
cargo test
```

## Install pre-commit hooks

This project uses [pre-commit](https://pre-commit.com/) to run checks before each
commit.

```bash
pip install pre-commit
pre-commit install
```

The hooks run file hygiene checks, `cargo fmt`, `cargo clippy`, `cargo test`, and
`mkdocs build`. Install the MkDocs dependencies so the docs hook can run:

```bash
pip install -r docs/requirements.txt
```

## Run the CLI

The current CLI demonstrates Lua parsing via tree-sitter:

```bash
cargo run -- <path-to-lua-file>
```

Once built, you can also run the binary directly:

```bash
./target/debug/lua-mutation-test <path-to-lua-file>
```

## Versioning

This project follows [ZeroVer](https://0ver.org/): all releases remain in the `0.x`
range while the API and feature set stabilizes.
