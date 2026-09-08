## Why

Users currently must pass all options through CLI flags every time they run the tool. A project-level configuration file and filtering controls are needed to make the tool usable in real repositories, to persist settings, and to let users narrow the mutation scope to relevant files, operators, and functions.

## What Changes

- Introduce a project configuration file format (`.lua-mutation-test.toml` or `.lua-mutation-test.json`) with a versioned schema.
- Support configurable items: test command, test framework, timeout, file glob patterns, mutation operators, parallelism level, and output report formats.
- Add a `--config` CLI flag to specify a custom configuration path.
- Implement include/exclude filtering for source files using glob patterns.
- Implement include/exclude filtering for mutation operators by identifier.
- Implement include/exclude filtering for functions by name pattern.
- Ensure CLI flags override values from the configuration file.
- Integrate the loaded configuration and filters into the CLI argument resolution and mutant generator pipeline.
- Add unit and integration tests covering config parsing, defaults, and filtering logic.

## Capabilities

### New Capabilities
- `configuration-filtering`: Load project-level configuration and apply include/exclude filters to files, operators, and functions.

### Modified Capabilities
<!-- No existing capabilities require requirement-level changes. -->

## Impact

- New configuration module using the `toml` crate and JSON fallback parsing.
- CLI argument merging logic between config file values and explicit flags.
- Filter integration in mutant generation and file discovery stages.
- Documentation updates for configuration schema and examples.
- Potential schema versioning to support future breaking changes.
