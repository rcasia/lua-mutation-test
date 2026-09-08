# Getting Started

## Prerequisites

- A Lua interpreter in your `PATH` (needed to run your test suite against mutants).
- (Optional) [Rust](https://www.rust-lang.org/tools/install) toolchain if you want to build from source.

## Installation

### From GitHub Releases

Download the pre-built binary for your platform from the
[releases page](https://github.com/rcasia/lua-mutation-test/releases) and place it
on your `PATH`.

The archive contains both `lua-mutation-test` and the shorter `lmut` alias.

### From crates.io

```bash
cargo install lua-mutation-test
```

### From source

See [CONTRIBUTING.md](https://github.com/rcasia/lua-mutation-test/blob/main/CONTRIBUTING.md)
for the development setup.

## Quick start

Run mutation testing against a file or directory:

```bash
lmut run <path-to-lua-file-or-directory>
```

Run with a custom test command and timeout:

```bash
lmut run src --test-command 'busted' --timeout 30
```

Create a sample configuration file:

```bash
lmut init
```

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
```

CLI flags override configuration file values.

## CLI Reference

See the [CLI Reference](cli-reference.md) for the complete list of subcommands,
options, and exit codes.

## Contributing

See [CONTRIBUTING.md](https://github.com/rcasia/lua-mutation-test/blob/main/CONTRIBUTING.md)
for development setup, workflow guidelines, and commit conventions.

## Versioning

This project follows [ZeroVer](https://0ver.org/): all releases remain in the `0.x`
range while the API and feature set stabilizes.
