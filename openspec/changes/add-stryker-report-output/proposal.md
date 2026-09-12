## Why

The project already generates HTML and CTRF reports, but there is no way to publish mutation results to the Stryker Dashboard, the de facto online host for mutation testing reports. Adding a Stryker-compatible JSON report output lets users upload results to `dashboard.stryker-mutator.io` and display a mutation score badge, without trying to force mutation data into code-coverage formats like lcov or Coveralls.

## What Changes

- Add a new `--output-format stryker` (or `--report-format stryker`) option to the `run` command that emits a `mutation-testing-report.json` file.
- The JSON output will conform to the [Stryker Mutation Testing Report Schema v2](https://github.com/stryker-mutator/mutation-testing-elements/tree/master/packages/report-schema).
- Extend the existing report generation pipeline with a new `StrykerReport` formatter, reusing per-mutant metadata (location, original, replacement, status) already collected by the runner.
- Update CLI help, `README.md`, `docs/getting-started.md`, `docs/cli-reference.md`, and `docs/architecture.md` to document the new format and the dashboard upload workflow.
- Add unit tests for JSON serialization and integration tests for the new CLI flag.

## Capabilities

### New Capabilities

- (none)

### Modified Capabilities

- `report-generation`: Add Stryker Mutation Testing Report Schema v2 as a supported output format. Requirements change only for the report generator; no changes to mutation operators, scoring semantics, or execution behavior.

## Impact

- `src/report.rs` / report module: new formatter and serialization logic.
- `src/cli.rs`: new accepted value for the output-format/report-format argument.
- Documentation: `README.md`, `docs/getting-started.md`, `docs/cli-reference.md`, `docs/architecture.md`.
- Tests: new unit and CLI integration tests.
- No breaking changes to existing HTML or CTRF reports.
