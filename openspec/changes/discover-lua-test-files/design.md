## Context

The tool currently lacks any test-discovery mechanism. Mutation testing requires a reliable baseline test run that proves the original code passes before mutants are evaluated. The acceptance criteria call for pattern-based discovery, configurable globs, adapters for popular Lua frameworks, and a generic command adapter as a fallback.

## Goals / Non-Goals

**Goals:**
- Discover test files using configurable glob patterns.
- Provide first-class adapters for `busted` and `luaunit`.
- Provide a generic command adapter for unsupported frameworks.
- Run the original source through the selected adapter and record the baseline outcome.

**Non-Goals:**
- Running tests against mutants (handled by a separate change).
- Installing Lua test frameworks; the tool assumes they are available on the host.
- Parallel test discovery across multiple projects.

## Decisions

- **Decision:** Use glob patterns resolved by the Rust standard library or the `glob` crate rather than a Lua-based file search.
  - **Rationale:** Keeps discovery fast, deterministic, and independent of the host Lua interpreter.
  - **Alternative considered:** Shell out to `find` or `ls`. Rejected to avoid platform differences.
- **Decision:** Model adapters as a trait/enum with a common `run(&self, test_files: &[Path]) -> TestResult` interface.
  - **Rationale:** Abstracts framework-specific invocation details and makes adding new adapters trivial.
  - **Alternative considered:** Inline command construction in the runner. Rejected because it couples framework knowledge with execution logic.
- **Decision:** Default to the generic command adapter when no framework is detected.
  - **Rationale:** Users can always provide a shell command, ensuring the tool works even without built-in adapters.
- **Decision:** Baseline runs use the same adapter path as mutant runs.
  - **Rationale:** Guarantees that a failing baseline surfaces configuration issues before any mutants are generated.

## Risks / Trade-offs

- [Risk] Glob patterns may match non-test files if user patterns are too broad.
  - → Mitigation: Document default patterns and allow explicit inclusion/exclusion overrides.
- [Risk] Framework adapters may fail if the expected binary or module layout differs.
  - → Mitigation: Expose framework-specific options in configuration and provide the generic adapter as an escape hatch.
