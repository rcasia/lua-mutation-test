## ADDED Requirements

### Requirement: Stryker-compatible JSON report
The system SHALL write a JSON report that conforms to the Stryker Mutation Testing Report Schema version 2.x when the user selects the `stryker` output format.

#### Scenario: Generate Stryker report after a run
- **WHEN** the user requests the `stryker` report format
- **THEN** the system SHALL write a `mutation-testing-report.json` file containing `schemaVersion`, `thresholds`, a `files` dictionary keyed by relative source path, and per-mutant entries with `id`, `mutatorName`, `replacement`, `location`, and `status`

#### Scenario: Stryker report includes full source for each file
- **WHEN** the `stryker` report is generated
- **THEN** every entry in the `files` object SHALL include the full original source text of that file

#### Scenario: Stryker report mutant statuses map to schema values
- **WHEN** a mutant result is converted to the Stryker format
- **THEN** internal statuses SHALL map as follows: `Killed` → `Killed`, `Survived` → `Survived`, `Timeout` → `Timeout`, and `Error` → `RuntimeError`

#### Scenario: Stryker report locations use schema coordinates
- **WHEN** a mutant location is written to the Stryker report
- **THEN** the location SHALL use 1-based line numbers and 0-based column numbers with `start` and `end` objects containing `line` and `column`

### Requirement: Stryker report selectable from CLI
The system SHALL accept `stryker` as a valid value for the report output format CLI option.

#### Scenario: User selects Stryker format
- **WHEN** the user passes `--output-format stryker` to the `run` command
- **THEN** the system SHALL generate the Stryker-compatible JSON report instead of, or in addition to, other selected formats

## MODIFIED Requirements

### Requirement: JSON report
The system SHALL write a JSON report containing full mutation results, project metadata, and a generation timestamp.

#### Scenario: Generate JSON report after a run
- **WHEN** the user requests a JSON report
- **THEN** the system SHALL write a JSON file containing every mutant result, the overall score, project metadata, and an ISO-8601 timestamp

#### Scenario: JSON report remains distinct from Stryker report
- **WHEN** the user requests the default `json` format
- **THEN** the system SHALL write the existing internal JSON schema and SHALL NOT write the Stryker schema

## REMOVED Requirements

- (none)
