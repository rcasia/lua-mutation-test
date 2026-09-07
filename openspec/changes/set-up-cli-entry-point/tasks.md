## 1. Setup

- [x] 1.1 Add the `clap` dependency to `Cargo.toml`
- [x] 1.2 Create the `src/cli.rs` module

## 2. CLI Definition

- [x] 2.1 Define the top-level `Cli` struct with global options `--config`, `--verbose`, `--quiet`, `--version`, and `--help`
- [x] 2.2 Define the `run`, `list-operators`, and `init` subcommands using clap derive enums
- [x] 2.3 Define `RunArgs` with a positional path and optional flags `--test-command`, `--timeout`, and `--output`

## 3. Dispatch & Exit Codes

- [x] 3.1 Wire CLI parsing into `src/main.rs`
- [x] 3.2 Dispatch each subcommand to a placeholder handler
- [x] 3.3 Return exit code 0 for success, 1 for test failures/survived mutants, and 2 for CLI errors

## 4. Help Text, Tests & Documentation

- [x] 4.1 Add usage examples to the generated help text
- [x] 4.2 Add unit tests for argument parsing covering global options and each subcommand
- [x] 4.3 Update `README.md` with CLI usage examples
