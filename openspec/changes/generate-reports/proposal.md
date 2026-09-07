## Why

Mutation testing produces large amounts of result data, but users currently have no structured way to inspect scores, per-mutant outcomes, or diffs. Human-readable and machine-readable reports are needed to make results actionable and to integrate with CI dashboards.

## What Changes

- Add a reporting module that consumes mutation results and emits multiple output formats.
- Implement a CLI summary reporter showing the overall mutation score and counts per result category (killed, survived, error, timeout).
- Implement a CLI per-mutant reporter showing each mutant's diff and test result.
- Implement a JSON reporter that writes a structured file with full results, metadata, and timestamps.
- Implement an HTML reporter with sortable tables, per-mutant diff views, and embedded CSS for readability.
- Make the report output path configurable via CLI flag or config file.
- Include project metadata and generation timestamps in every report.
- Add unit and integration tests for each reporter.
- Update README with example report outputs.

## Capabilities

### New Capabilities
- `report-generation`: Produce CLI, JSON, and HTML reports from mutation testing results with configurable output paths and embedded metadata.

### Modified Capabilities
<!-- No existing capabilities require requirement-level changes. -->

## Impact

- New Rust reporting module and formatter abstractions.
- HTML/CSS template assets or inline string builders.
- CLI flag additions for report format and output directory.
- New dependencies may be needed for serialization (e.g., `serde_json`) and HTML generation.
- README documentation update with report examples.
