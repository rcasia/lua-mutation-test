## 1. Setup

- [ ] 1.1 Add the `clap` dependency to `Cargo.toml`
- [ ] 1.2 Create the `src/cli.rs` module

## 2. CLI Definition

- [ ] 2.1 Define the top-level `Cli` struct with global options `--config`, `--verbose`, `--quiet`, `--version`, and `--help`
- [ ] 2.2 Define the `run`, `list-operators`, and `init` subcommands using clap derive enums
- [ ] 2.3 Define `RunArgs` with a positional path and optional flags `--test-command`, `--timeout`, and `--output`

## 3. Dispatch & Exit Codes

- [ ] 3.1 Wire CLI parsing into `src/main.rs`
- [ ] 3.2 Dispatch each subcommand to a placeholder handler
- [ ] 3.3 Return exit code 0 for success, 1 for test failures/survived mutants, and 2 for CLI errors

## 4. Help Text, Tests & Documentation

- [ ] 4.1 Add usage examples to the generated help text
- [ ] 4.2 Add unit tests for argument parsing covering global options and each subcommand
- [ ] 4.3 Update `README.md` with CLI usage examples
