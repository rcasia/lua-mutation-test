## Context

Full mutation runs can be expensive for large codebases. Many mutants are associated with files that did not change between runs, so their results can be reused. Incremental execution combined with a watch mode gives developers continuous, fast feedback during iterative development.

## Goals / Non-Goals

**Goals:**
- Skip mutants whose source files and hashes are unchanged since the last run.
- Detect file changes reliably using git or content hashes.
- Provide a `watch` subcommand that re-runs affected mutants on file changes.
- Persist incremental state across CLI invocations.
- Invalidate cached results when configuration changes.

**Non-Goals:**
- Cross-project or shared incremental caches.
- Perfect dependency tracking beyond the source file level.
- Real-time IDE-style notifications.

## Decisions

- **Cache location: `.lua-mutation-test/cache/`.** Rationale: co-located with project metadata and naturally gitignored.
- **Change detection: compare git tree state when available, fall back to SHA-256 file hashes.** Rationale: git is fast and authoritative in git repos; file hashes work outside version control.
- **Cache key includes file path, mutant hash, and a hash of the active configuration.** Rationale: ensures cache invalidation when operators or filters change.
- **Watch mode uses the `notify` crate with debouncing.** Rationale: `notify` is the standard Rust file-watching library; debouncing prevents duplicate runs during save bursts.

## Risks / Trade-offs

- **[Risk]** Stale cache when mutation operators or config change. → **Mitigation:** include a config hash in every cache entry key and invalidate entries with mismatched hashes.
- **[Risk]** File system events lost or duplicated. → **Mitigation:** debounce events and perform a lightweight full scan before each incremental run.
- **[Risk]** Cache corruption on abnormal termination. → **Mitigation:** write cache files atomically and treat unreadable cache as a miss.
