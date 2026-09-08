## 1. Design reporting abstractions

- [x] 1.1 Define a `Reporter` trait and a shared mutation result input data structure.
- [x] 1.2 Define an enum or registry for supported report formats (CLI summary, CLI per-mutant, JSON, HTML).

## 2. Implement CLI reporters

- [x] 2.1 Implement the CLI summary reporter with overall score and per-category counts.
- [x] 2.2 Implement the CLI per-mutant reporter with diff and result output.
- [x] 2.3 Add unit tests for CLI reporters.

## 3. Implement JSON reporter

- [x] 3.1 Define the JSON report schema including results, metadata, and timestamp.
- [x] 3.2 Implement JSON serialization of mutation results.
- [x] 3.3 Add unit and integration tests for JSON report generation.

## 4. Implement HTML reporter

- [x] 4.1 Build the HTML report layout with sortable result tables.
- [x] 4.2 Add per-mutant diff views and embedded CSS.
- [x] 4.3 Add tests verifying HTML report structure and content.

## 5. Integrate reports with CLI and config

- [x] 5.1 Add CLI flags for report format and output path.
- [x] 5.2 Wire report generation into the main run flow after results are collected.
- [x] 5.3 Support output path configuration from the config file.

## 6. Documentation and examples

- [x] 6.1 Add example CLI, JSON, and HTML report outputs to README.
- [x] 6.2 Document report flags and output path options.
