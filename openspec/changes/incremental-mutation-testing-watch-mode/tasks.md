## 1. Cache design and storage

- [x] 1.1 Define the cache schema keyed by file path and mutant hash
- [x] 1.2 Implement cache read/write to `.lua-mutation-test/cache/`

## 2. Change detection

- [x] 2.1 Implement git-based change detection
- [x] 2.2 Add fallback file-hash change detection

## 3. Incremental run logic

- [x] 3.1 Filter mutants based on changed files
- [x] 3.2 Integrate the cache with the test runner

## 4. Watch mode

- [x] 4.1 Add a `watch` subcommand to the CLI
- [x] 4.2 Integrate the `notify` crate for file-system events
- [x] 4.3 Debounce file events and trigger an incremental re-run

## 5. Cache invalidation and tests

- [x] 5.1 Invalidate the cache when configuration changes
- [x] 5.2 Add tests for cache hits, misses, and invalidation
- [x] 5.3 Add a watch-mode integration test
