## 1. Cache design and storage

- [ ] 1.1 Define the cache schema keyed by file path and mutant hash
- [ ] 1.2 Implement cache read/write to `.lua-mutation-test/cache/`

## 2. Change detection

- [ ] 2.1 Implement git-based change detection
- [ ] 2.2 Add fallback file-hash change detection

## 3. Incremental run logic

- [ ] 3.1 Filter mutants based on changed files
- [ ] 3.2 Integrate the cache with the test runner

## 4. Watch mode

- [ ] 4.1 Add a `watch` subcommand to the CLI
- [ ] 4.2 Integrate the `notify` crate for file-system events
- [ ] 4.3 Debounce file events and trigger an incremental re-run

## 5. Cache invalidation and tests

- [ ] 5.1 Invalidate the cache when configuration changes
- [ ] 5.2 Add tests for cache hits, misses, and invalidation
- [ ] 5.3 Add a watch-mode integration test
