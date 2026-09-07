## Context

The CLI entry point currently accepts all options as command-line flags. Real projects need a persistent, shareable configuration file and the ability to include or exclude files, operators, and functions. The issue acceptance criteria define the configuration schema, the `--config` flag, and the precedence of CLI flags over config values.

## Goals / Non-Goals

**Goals:**
- Define and parse a versioned project configuration file in TOML or JSON.
- Support configuration of test command, framework, timeout, glob patterns, operators, parallelism, and output formats.
- Implement include/exclude filters for files, operators, and functions.
- Add a `--config` CLI flag to load a specific config file.
- Ensure explicit CLI flags override config file values.
- Integrate configuration and filters into the CLI and mutant generator.
- Add tests for parsing, defaults, and filtering behavior.

**Non-Goals:**
- GUI or interactive configuration setup.
- Remote or environment-variable-based configuration beyond what the CLI already supports.
- Auto-discovery of test frameworks beyond the configured test command.

## Decisions

- **Decision:** Use the `toml` crate as the primary parser and fall back to JSON when the file extension is `.json`.
  - **Rationale:** The issue explicitly recommends `toml`; JSON fallback improves ergonomics for teams that prefer JSON.
  - **Alternative considered:** Support only TOML. Rejected because JSON is listed in the acceptance criteria.
- **Decision:** Represent filters as `include` and `exclude` lists for files, operators, and functions, where exclusion takes precedence.
  - **Rationale:** Simple, predictable semantics and easy to document.
  - **Alternative considered:** Complex query language for filters. Rejected because it adds parsing complexity without clear benefit.
- **Decision:** Merge config values and CLI flags with explicit flags winning.
  - **Rationale:** Matches common CLI conventions and the acceptance criteria.
  - **Alternative considered:** Config file always overrides flags. Rejected because it contradicts the requirement that CLI flags override config file values.
- **Decision:** Version the config schema with a `version` field and fail fast on unsupported major versions.
  - **Rationale:** Allows future schema evolution with clear error messages.

## Risks / Trade-offs

- [Risk] Config file errors may be hard for users to diagnose.
  - → Mitigation: Provide detailed, line-numbered parse error messages and link to documentation.
- [Risk] Glob patterns can accidentally exclude all source files.
  - → Mitigation: Validate that at least one source file remains after filtering and emit a clear warning.
- [Risk] Operator name filtering depends on stable operator identifiers.
  - → Mitigation: Document operator identifiers in the README and config schema docs.

## Migration Plan

No migration required. Configuration loading is optional; existing CLI-only usage continues to work with default values.

## Open Questions

- Should the default config file name be `.lua-mutation-test.toml` in the current working directory or search parent directories?
- Should unknown config keys be ignored or treated as errors to help catch typos?
