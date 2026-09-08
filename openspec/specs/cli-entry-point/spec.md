# cli-entry-point Specification

## Purpose
TBD - created by archiving change set-up-cli-entry-point. Update Purpose after archive.
## Requirements
### Requirement: Parse global options
The system SHALL parse the top-level options `--config`, `--verbose`, `--quiet`, `--version`, and `--help`.

#### Scenario: Show version
- **WHEN** the user runs `lua-mutation-test --version`
- **THEN** the program prints the version and exits with code 0

#### Scenario: Show help
- **WHEN** the user runs `lua-mutation-test --help`
- **THEN** the program prints help text that includes usage examples and exits with code 0

### Requirement: Parse subcommands
The system SHALL support the subcommands `run`, `list-operators`, and `init`.

#### Scenario: Run subcommand
- **WHEN** the user runs `lua-mutation-test run <path>`
- **THEN** the program dispatches to the run workflow

#### Scenario: List operators subcommand
- **WHEN** the user runs `lua-mutation-test list-operators`
- **THEN** the program prints the available mutation operators

#### Scenario: Init subcommand
- **WHEN** the user runs `lua-mutation-test init`
- **THEN** the program creates a sample configuration file

### Requirement: Provide run flags
The `run` subcommand SHALL accept a positional path argument and the optional flags `--test-command`, `--timeout`, and `--output`.

#### Scenario: Run with default options
- **WHEN** the user runs `lua-mutation-test run path/to/file.lua`
- **THEN** the program parses the path with the optional flags set to their defaults

#### Scenario: Run with all flags
- **WHEN** the user runs `lua-mutation-test run src --test-command 'busted' --timeout 30 --output json`
- **THEN** the program parses all provided flags and the path

### Requirement: Return meaningful exit codes
The system SHALL exit with code 0 on success, code 1 when tests fail or mutants survive, and code 2 on CLI errors.

#### Scenario: Success exit code
- **WHEN** the CLI parses and executes a valid command successfully
- **THEN** the program exits with code 0

#### Scenario: Test failure exit code
- **WHEN** the run workflow reports that mutants survived
- **THEN** the program exits with code 1

#### Scenario: CLI error exit code
- **WHEN** the user provides an unknown flag or invalid arguments
- **THEN** the program exits with code 2

