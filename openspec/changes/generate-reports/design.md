## Context

The mutation testing pipeline is now able to parse Lua, generate mutants, run them with timeouts, and capture test results. The next step is to present those results in useful formats. The issue acceptance criteria require CLI, JSON, and HTML reports, configurable output paths, and embedded timestamps and metadata.

## Goals / Non-Goals

**Goals:**
- Implement CLI, JSON, and HTML reporters that consume the mutation result data structure.
- Provide a summary CLI view and a per-mutant CLI view with diffs.
- Generate JSON reports containing full results, project metadata, and timestamps.
- Generate HTML reports with sortable tables and diff views for each mutant.
- Make report output paths configurable via existing CLI/config mechanisms.
- Add tests for each reporter and update README examples.

**Non-Goals:**
- Real-time streaming report updates during mutant execution.
- PDF or other report formats beyond CLI, JSON, and HTML.
- Complex interactive HTML features such as search or server-side rendering.
- Changing the mutation result data model itself.

## Decisions

- **Decision:** Implement a single `Reporter` trait with concrete implementations for CLI summary, CLI per-mutant, JSON, and HTML reporters.
  - **Rationale:** A shared trait makes it easy to add formats later and allows the CLI to select reporters from a list without branching logic.
  - **Alternative considered:** Separate standalone functions for each format. Rejected because it duplicates path handling and result aggregation logic.
- **Decision:** Build HTML with inline string templates and embedded CSS rather than pulling in a full template engine.
  - **Rationale:** Keeps dependencies minimal while still producing readable, sortable tables. The reports are small and self-contained.
  - **Alternative considered:** Use a Rust HTML templating crate. Rejected to avoid extra dependencies for a single feature.
- **Decision:** Embed timestamps and project metadata in JSON and HTML output only.
  - **Rationale:** CLI output is intended for quick human inspection; machine-readable metadata belongs in file-based reports.
  - **Alternative considered:** Add metadata to CLI output too. Rejected to keep CLI output concise.

## Risks / Trade-offs

- [Risk] Large mutation runs may produce large HTML files.
  - → Mitigation: Keep the HTML template simple and consider paging or lazy rendering if file sizes become problematic.
- [Risk] Sortable tables require inline JavaScript, which may be blocked by strict content-security policies in some CI viewers.
  - → Mitigation: Provide plain HTML tables with semantic headers; sorting can be optional and degrade gracefully.
- [Risk] Mutant diffs rely on the original source being available at report time.
  - → Mitigation: Store original source snippets in the mutation result data structure so reports are self-contained.

## Migration Plan

No migration required. Reporting is a new feature. Existing CLI behavior remains unchanged unless report flags are provided.

## Open Questions

- Should the CLI summary reporter also write to a file when an output path is provided, or only the JSON/HTML reporters?
- Which exact metadata fields (project name, version, source paths) should be included in JSON/HTML reports?
