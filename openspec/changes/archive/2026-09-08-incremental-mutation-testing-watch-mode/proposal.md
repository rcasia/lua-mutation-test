## Why

Re-running every mutant after a small source change is wasteful because most mutants are unaffected by the change. Caching results per file and per mutant hash, detecting changed files, and re-running only affected mutants dramatically shortens the feedback loop during development.

## What Changes

- Cache mutation results keyed by source file path and mutant hash.
- Detect changed source files since the last run using git status or file content hashes.
- Re-run only mutants whose source file or dependencies have changed.
- Add a `watch` subcommand that re-runs mutation tests on file system changes.
- Persist incremental state in `.lua-mutation-test/cache/`.
- Invalidate the cache when mutation testing configuration changes.

## Capabilities

### New Capabilities
- `incremental-mutation-testing`: Cache mutation results and selectively re-run mutants based on changed files and configuration state.

### Modified Capabilities
- None

## Impact

- Test runner, result collection, and reporting pipeline.
- CLI with a new `watch` subcommand.
- New dependency on the `notify` crate for file-system watching.
- New `.lua-mutation-test/cache/` storage directory.
