## Why

Generating mutants by editing source text can produce invalid Lua or silently shift source locations, which corrupts test results and makes reports hard to interpret. Validating each mutated source and preserving source maps ensures that only runnable mutants are executed and that failures can be traced back to the original code.

## What Changes

- Re-parse every mutated source with the existing tree-sitter Lua parser to verify syntactic validity before execution.
- Mark mutants that fail validation as `error`, skip them from test runs, and include them in reports as errors.
- Write validated mutated sources to temporary files while preserving the original relative path structure.
- Preserve original line numbers where possible by avoiding insertions or deletions that add or remove lines.
- Introduce a `SourceMap` data structure that maps mutated source locations back to original source locations for debug output and reports.
- Add unit tests covering valid mutants, invalid mutants, temp file layout, line-number preservation, and source map accuracy.

## Capabilities

### New Capabilities
- `mutant-validation`: Validate mutated Lua sources for syntactic correctness, persist them to a temporary directory structure, and provide source maps for original-to-mutant location mapping.

### Modified Capabilities
<!-- No existing capabilities require requirement-level changes. -->

## Impact

- Additional parser invocation during mutant generation.
- New validation and source-map modules in the Rust codebase.
- Temporary file generation utilities integrated with the test runner.
- Changes to mutant result categorization to include an `error` state.
- Reporting modules will consume source-map data for accurate location output.
