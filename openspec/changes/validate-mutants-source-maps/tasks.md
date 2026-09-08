## 1. Implement mutant syntactic validation

- [x] 1.1 Add a validation function that re-parses mutated source with the existing Lua parser.
- [x] 1.2 Integrate validation into the mutant generation pipeline.
- [x] 1.3 Add unit tests for valid and invalid mutants.

## 2. Implement error marking for invalid mutants

- [x] 2.1 Add an `error` variant to the mutant result enum.
- [x] 2.2 Record invalid mutants as `error` and skip them during test execution.
- [x] 2.3 Ensure error mutants appear correctly in reports.
- [x] 2.4 Add tests verifying error marking and skipping behavior.

## 3. Implement temporary file generation

- [x] 3.1 Create a temporary directory for mutated sources.
- [x] 3.2 Write each valid mutant to a temporary file preserving the original relative path.
- [x] 3.3 Add tests verifying temporary file layout and content.

## 4. Preserve original line numbers

- [x] 4.1 Audit existing mutation operators to ensure they perform in-place replacements.
- [x] 4.2 Add generation-time checks or constraints to prevent insertions or deletions that add or remove lines.
- [x] 4.3 Add tests asserting line-number preservation.

## 5. Implement SourceMap

- [x] 5.1 Define a `SourceMap` data structure mapping mutated locations to original locations.
- [x] 5.2 Populate the source map during mutant generation.
- [x] 5.3 Expose source map data for debug output and reports.
- [x] 5.4 Add tests for source map accuracy.

## 6. Integration and documentation

- [x] 6.1 Wire validation, temp files, and source maps into the test runner integration.
- [x] 6.2 Document the validation behavior and source map usage.
- [x] 6.3 Add integration tests covering the full validation pipeline.
