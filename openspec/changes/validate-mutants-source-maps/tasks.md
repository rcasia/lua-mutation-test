## 1. Implement mutant syntactic validation

- [ ] 1.1 Add a validation function that re-parses mutated source with the existing Lua parser.
- [ ] 1.2 Integrate validation into the mutant generation pipeline.
- [ ] 1.3 Add unit tests for valid and invalid mutants.

## 2. Implement error marking for invalid mutants

- [ ] 2.1 Add an `error` variant to the mutant result enum.
- [ ] 2.2 Record invalid mutants as `error` and skip them during test execution.
- [ ] 2.3 Ensure error mutants appear correctly in reports.
- [ ] 2.4 Add tests verifying error marking and skipping behavior.

## 3. Implement temporary file generation

- [ ] 3.1 Create a temporary directory for mutated sources.
- [ ] 3.2 Write each valid mutant to a temporary file preserving the original relative path.
- [ ] 3.3 Add tests verifying temporary file layout and content.

## 4. Preserve original line numbers

- [ ] 4.1 Audit existing mutation operators to ensure they perform in-place replacements.
- [ ] 4.2 Add generation-time checks or constraints to prevent insertions or deletions that add or remove lines.
- [ ] 4.3 Add tests asserting line-number preservation.

## 5. Implement SourceMap

- [ ] 5.1 Define a `SourceMap` data structure mapping mutated locations to original locations.
- [ ] 5.2 Populate the source map during mutant generation.
- [ ] 5.3 Expose source map data for debug output and reports.
- [ ] 5.4 Add tests for source map accuracy.

## 6. Integration and documentation

- [ ] 6.1 Wire validation, temp files, and source maps into the test runner integration.
- [ ] 6.2 Document the validation behavior and source map usage.
- [ ] 6.3 Add integration tests covering the full validation pipeline.
