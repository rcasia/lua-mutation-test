## Context

The mutant generation pipeline edits Lua source text to produce variants. Without validation, a malformed edit can yield invalid Lua that fails at test time and is hard to distinguish from a real test failure. In addition, reports need accurate source locations, which requires mapping mutated positions back to the original file. The issue acceptance criteria require syntactic validation, error marking, temporary file generation, line-number preservation, and a source map.

## Goals / Non-Goals

**Goals:**
- Re-parse every mutated source with the existing tree-sitter Lua parser.
- Mark invalid mutants as `error` and exclude them from test runs.
- Write valid mutated sources to temporary files while preserving relative path structure.
- Preserve original line numbers by avoiding edits that insert or remove lines.
- Provide a `SourceMap` for mapping mutated locations back to original source locations.
- Add tests for validation, temp files, line numbers, and source maps.

**Non-Goals:**
- Semantic validation or detection of equivalent mutants.
- Rewriting the parser; reuse the existing tree-sitter Lua integration.
- Source maps that map column-level changes beyond line preservation.

## Decisions

- **Decision:** Validate mutants immediately after generation by running the mutated source through the existing parser.
  - **Rationale:** Reuses proven infrastructure and catches syntax errors before the test runner sees them.
  - **Alternative considered:** Let the Lua interpreter report syntax errors during test runs. Rejected because it conflates mutant syntax errors with test failures.
- **Decision:** Write mutants to a temporary directory structured as `<tmp>/<relative-path>/<mutant-id>.lua`.
  - **Rationale:** Preserving the relative path structure lets the test command resolve `require` paths as if they were in the project tree.
  - **Alternative considered:** Flatten all mutants into a single directory. Rejected because it breaks relative module loading.
- **Decision:** Preserve original line numbers by restricting mutations to in-place replacements that do not add or remove lines.
  - **Rationale:** Keeps debug output and stack traces aligned with the original source without needing complex column mapping.
  - **Alternative considered:** Full source map with line/column deltas. Rejected to keep the first iteration simple.
- **Decision:** Model the `SourceMap` as a mapping from `(mutant_id, mutated_line)` to `(original_file, original_line)`.
  - **Rationale:** Sufficient for the reporting and debug use cases described in the issue.

## Risks / Trade-offs

- [Risk] Re-parsing every mutant adds overhead to generation time.
  - → Mitigation: Validation is a single parse per mutant and runs during generation; it can be skipped only if later profiling shows it to be a bottleneck.
- [Risk] Temp directories may not be cleaned up if the process crashes.
  - → Mitigation: Use a temporary directory crate that registers cleanup handlers and document manual cleanup for abnormal exits.
- [Risk] In-place replacements limit some potentially useful mutations.
  - → Mitigation: Accept this constraint for now; line-preserving mutations cover the operators already planned.

## Migration Plan

No migration required. Validation and source maps are additive; existing valid mutants continue to run unchanged.

## Open Questions

- Should invalid mutants be reported separately in the final mutation score denominator?
- Should the temporary directory be configurable or always use the system temp directory?
