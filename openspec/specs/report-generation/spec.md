# report-generation Specification

## Purpose
TBD - created by archiving change generate-reports. Update Purpose after archive.
## Requirements
### Requirement: CLI summary report
The system SHALL emit a CLI summary containing the overall mutation score and counts for each result category.

#### Scenario: Run completes with mixed results
- **WHEN** the mutation run finishes
- **THEN** the CLI summary SHALL display the overall mutation score and the counts of killed, survived, error, and timeout mutants

### Requirement: Per-mutant CLI output
The system SHALL emit per-mutant CLI output that includes the mutant identifier, the diff against the original source, and the test result.

#### Scenario: Inspect a killed mutant
- **WHEN** a mutant is killed by the test suite
- **THEN** the CLI output SHALL show the mutant identifier, the unified diff of the mutation, and the result label `killed`

### Requirement: JSON report
The system SHALL write a JSON report containing full mutation results, project metadata, and a generation timestamp.

#### Scenario: Generate JSON report after a run
- **WHEN** the user requests a JSON report
- **THEN** the system SHALL write a JSON file containing every mutant result, the overall score, project metadata, and an ISO-8601 timestamp

### Requirement: HTML report
The system SHALL write an HTML report with sortable result tables and per-mutant diff views.

#### Scenario: Generate HTML report after a run
- **WHEN** the user requests an HTML report
- **THEN** the system SHALL write an HTML file containing a sortable table of mutants, their results, and expandable diff views for each mutant

### Requirement: Configurable report output path
The system SHALL allow the user to configure the directory or file path for generated reports.

#### Scenario: Specify report output directory
- **WHEN** the user provides a report output path
- **THEN** the system SHALL write the requested reports to that path

### Requirement: Reports include timestamps and project metadata
The system SHALL include a generation timestamp and project metadata in every JSON and HTML report.

#### Scenario: Read report metadata
- **WHEN** a JSON or HTML report is generated
- **THEN** the report SHALL contain a generation timestamp and project metadata such as source paths and tool version

