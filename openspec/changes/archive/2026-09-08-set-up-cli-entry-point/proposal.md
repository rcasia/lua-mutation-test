## Why

`lua-mutation-test` currently has no user-facing command-line interface. Establishing a CLI entry point is the first step toward making the tool usable and provides the foundation for all subsequent workflows (run, list operators, init).

## What Changes

- Add `clap` as a dependency and use derive macros for the CLI definition.
- Define top-level global options: `--config`, `--verbose`, `--quiet`, `--version`, and `--help`.
- Define subcommands: `run`, `list-operators`, and `init`.
- `run` accepts a positional path (file or directory) and optional flags `--test-command`, `--timeout`, and `--output`.
- `list-operators` prints the available mutation operators.
- `init` creates a sample configuration file.
- Define meaningful exit codes: `0` for success, `1` for test failures or survived mutants, and `2` for CLI errors.
- Include usage examples in the help text and update the README.

## Capabilities

### New Capabilities
- `cli-entry-point`: argument parsing and subcommand dispatch for the top-level `lua-mutation-test` binary.

### Modified Capabilities
<!-- No existing capabilities are modified by this change. -->

## Impact

- `Cargo.toml`: new `clap` dependency.
- `src/main.rs`: replace the current placeholder with CLI parsing and dispatch.
- `src/cli.rs`: new module for CLI structs and argument definitions.
- `README.md`: usage examples and command reference.
- Unit tests for argument parsing.
