## 1. Design reporting abstractions

- [ ] 1.1 Define a `Reporter` trait and a shared mutation result input data structure.
- [ ] 1.2 Define an enum or registry for supported report formats (CLI summary, CLI per-mutant, JSON, HTML).

## 2. Implement CLI reporters

- [ ] 2.1 Implement the CLI summary reporter with overall score and per-category counts.
- [ ] 2.2 Implement the CLI per-mutant reporter with diff and result output.
- [ ] 2.3 Add unit tests for CLI reporters.

## 3. Implement JSON reporter

- [ ] 3.1 Define the JSON report schema including results, metadata, and timestamp.
- [ ] 3.2 Implement JSON serialization of mutation results.
- [ ] 3.3 Add unit and integration tests for JSON report generation.

## 4. Implement HTML reporter

- [ ] 4.1 Build the HTML report layout with sortable result tables.
- [ ] 4.2 Add per-mutant diff views and embedded CSS.
- [ ] 4.3 Add tests verifying HTML report structure and content.

## 5. Integrate reports with CLI and config

- [ ] 5.1 Add CLI flags for report format and output path.
- [ ] 5.2 Wire report generation into the main run flow after results are collected.
- [ ] 5.3 Support output path configuration from the config file.

## 6. Documentation and examples

- [ ] 6.1 Add example CLI, JSON, and HTML report outputs to README.
- [ ] 6.2 Document report flags and output path options.
