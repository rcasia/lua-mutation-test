## ADDED Requirements

### Requirement: Configuration file support
The system SHALL support a project configuration file named `.lua-mutation-test.toml` or `.lua-mutation-test.json`.

#### Scenario: Load TOML config from project root
- **WHEN** a `.lua-mutation-test.toml` file exists in the working directory
- **THEN** the system SHALL load configuration values from that file

#### Scenario: Load JSON config from project root
- **WHEN** a `.lua-mutation-test.json` file exists in the working directory
- **THEN** the system SHALL load configuration values from that file

### Requirement: Configurable execution settings
The system SHALL allow the configuration file to specify the test command, test framework, timeout, glob patterns, mutation operators, parallelism level, and output report formats.

#### Scenario: Configure test command and timeout
- **WHEN** the configuration file contains `test_command` and `timeout` values
- **THEN** the system SHALL use those values as defaults for the run

### Requirement: Config path CLI flag
The system SHALL provide a `--config` CLI flag that specifies the path to a configuration file.

#### Scenario: Explicit config path
- **WHEN** the user invokes the tool with `--config path/to/config.toml`
- **THEN** the system SHALL load the configuration from the specified path

### Requirement: Include and exclude file patterns
The system SHALL support include and exclude glob patterns for source files.

#### Scenario: Exclude test files from mutation
- **WHEN** the configuration file contains an exclude glob pattern matching test files
- **THEN** the system SHALL not generate mutants for files matching that pattern

### Requirement: Include and exclude mutation operators
The system SHALL support include and exclude lists for mutation operators by identifier.

#### Scenario: Disable a specific operator
- **WHEN** the configuration file excludes a specific operator identifier
- **THEN** the system SHALL not generate mutants using that operator

### Requirement: Include and exclude functions by name pattern
The system SHALL support include and exclude patterns for function names.

#### Scenario: Exclude helper functions from mutation
- **WHEN** the configuration file excludes functions matching a name pattern
- **THEN** the system SHALL not generate mutants inside functions matching that pattern

### Requirement: CLI flags override configuration values
The system SHALL allow explicit CLI flags to override values loaded from the configuration file.

#### Scenario: Override timeout via CLI
- **WHEN** the configuration file specifies a timeout and the user also passes `--timeout` on the CLI
- **THEN** the system SHALL use the CLI-provided timeout value
