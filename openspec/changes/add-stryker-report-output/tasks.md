## 1. Data model and serialization

- [ ] 1.1 Add `StrykerReport` structs matching the Mutation Testing Report Schema v2 (`schemaVersion`, `thresholds`, `files`, `FileResult`, `Mutant`, `Location`, `Position`).
- [ ] 1.2 Implement `Serialize` for all Stryker report types using `serde`.
- [ ] 1.3 Add unit tests that serialize a minimal report and assert expected JSON keys.

## 2. Mapping from internal results

- [ ] 2.1 Implement conversion from internal `MutantResult` status to Stryker `MutantStatus` (`Killed`, `Survived`, `Timeout`, `RuntimeError`).
- [ ] 2.2 Implement conversion from internal byte-offset locations to Stryker 1-based line / 0-based column `Location` using source line-start cache.
- [ ] 2.3 Add unit tests for status mapping and location conversion edge cases (single-line and multi-line mutants).

## 3. Report generation integration

- [ ] 3.1 Add a `StrykerReport` formatter in the report module that builds the report from `MutationRunResult` and writes `mutation-testing-report.json`.
- [ ] 3.2 Add `stryker` as a valid value for the CLI `--output-format` / `--report-format` option.
- [ ] 3.3 Route the `stryker` format value to the new formatter in report dispatch logic.
- [ ] 3.4 Add an integration test that runs `lmut run --output-format stryker` and verifies the generated `mutation-testing-report.json` parses and contains the expected schema version.

## 4. Validation and quality

- [ ] 4.1 Add a test that validates the generated report against the published Stryker Mutation Testing Report Schema v2 JSON schema (fetched or vendored).
- [ ] 4.2 Run `cargo test` and ensure no new failures.
- [ ] 4.3 Run `cargo clippy` and resolve any new warnings introduced by the change.

## 5. Documentation

- [ ] 5.1 Update `README.md` with an example of `--output-format stryker` and the dashboard upload command.
- [ ] 5.2 Update `docs/getting-started.md` to mention the Stryker report format.
- [ ] 5.3 Update `docs/cli-reference.md` to list `stryker` as a valid output format value.
- [ ] 5.4 Update `docs/architecture.md` to document the status mapping and Stryker schema integration.
