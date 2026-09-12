## Context

The project currently emits three report formats: a CLI summary, a JSON report with internal schema, and an HTML report. None of these formats can be consumed by the Stryker Dashboard (`dashboard.stryker-mutator.io`), the most widely used online host for mutation testing reports. The Stryker project defines an open [Mutation Testing Report Schema](https://github.com/stryker-mutator/mutation-testing-elements/tree/master/packages/report-schema) (version 2.x) that any tool can implement and upload via a simple HTTP PUT.

## Goals / Non-Goals

**Goals:**
- Emit a Stryker-compatible `mutation-testing-report.json` file from a mutation run.
- Reuse existing per-mutant metadata (location, original code, replacement, status) without changing mutation or execution semantics.
- Allow users to trigger the new format through the existing `--output-format` / `--report-format` CLI option.
- Document the upload workflow and add automated tests for serialization and CLI selection.

**Non-Goals:**
- Uploading the report to the dashboard automatically from the tool (out of scope; users use `curl` or CI).
- Supporting the older v1 Stryker schema.
- Changing the existing internal JSON or HTML report schemas.
- Adding GitHub OAuth, badge generation, or dashboard configuration inside this tool.

## Decisions

1. **Report format option value**: use `stryker` as the CLI enum value (`--output-format stryker`). It is short, unambiguous, and matches the dominant project name. The output file will be named `mutation-testing-report.json` inside the configured report directory.

2. **Schema version**: target schema version `2.0` (current stable). The schema requires `schemaVersion`, `thresholds`, and `files`; we will also provide `projectRoot` and per-file `source` fields so the dashboard can render source annotations.

3. **Mapping mutant statuses**:
   - `Killed` → `Killed`
   - `Survived` → `Survived`
   - `Timeout` → `Timeout`
   - `Error` (e.g. test command failed to run) → `RuntimeError`
   - Mutants excluded by filters or not executed → `NoCoverage` (only if we know they were never run)

4. **Location format**: Stryker uses 1-based lines and 0-based columns. Our internal representation is byte/char offsets; we will convert using the source text and line-start cache already used by the HTML report.

5. **Thresholds**: default to `{ "high": 80, "low": 60, "break": null }` until a configurable threshold system exists. This satisfies the schema and gives meaningful badge coloring without inventing new CLI flags.

6. **Implementation location**: add a `StrykerReport` struct in the report module next to `HtmlReport` and `JsonReport`. It will implement a shared `Report` trait if one exists, otherwise produce bytes directly.

## Risks / Trade-offs

- **[Risk] Schema drift** → Mitigation: pin to schema version `2.0` in `schemaVersion`, validate our output against the published JSON schema in an integration test, and only bump versions in a dedicated change.
- **[Risk] Status mapping ambiguity** → Mitigation: document the mapping in `docs/architecture.md` and keep a small, exhaustive enum match so any future status must be handled explicitly.
- **[Risk] Large JSON files** → Mitigation: stream the output through `serde_json` to the configured file path; the report is only generated when requested.

## Migration Plan

Not applicable. This is an additive feature; existing reports and default behavior remain unchanged.

## Open Questions

- Should the internal JSON report be renamed or aliased to avoid confusion with the Stryker JSON report? (Recommendation: keep internal format as `json`; Stryker format as `stryker`.)
