## Context

`lua-mutation-test` already vendors `tree-sitter-lua` and the current `src/main.rs` loads the language with `tree_sitter_lua::LANGUAGE.into()`. The next step is to encapsulate that initialization behind a reusable parser API so mutation operators can parse Lua files and strings without duplicating parser setup or error handling.

## Goals / Non-Goals

**Goals:**
- Provide `parse_file` and `parse_source` functions that return a tree-sitter `Tree`.
- Load the Lua grammar via `tree_sitter_lua::LANGUAGE.into()`.
- Report parse errors with file path and line/column information.
- Make the parser usable from both production code and unit tests.

**Non-Goals:**
- Implementing mutation operators or AST traversal (handled in separate changes).
- Custom grammar changes; the vendored grammar is used as-is.
- Async or streaming parsing.

## Decisions

- **Load the language with `tree_sitter_lua::LANGUAGE.into()`**, as established by the project. This avoids any custom `build.rs` and relies on the vendored crate.
- **Wrap `tree_sitter::Parser` in a small `Parser` struct**. The wrapper owns the parser instance and exposes `parse_file` and `parse_source`. This centralizes language setup and makes the API discoverable.
- **`parse_file` reads the file into memory and delegates to `parse_source`**. Reusing `parse_source` keeps the two paths consistent and simplifies testing.
- **`ParseError` is an owned error type** containing:
  - An optional file path.
  - The source text (or a copy) so offsets can be translated to line/column.
  - The byte offset where parsing failed.
- **Line/column are computed from the source text** using a line-start offset table, returning 1-indexed line and 1-indexed byte-column values matching tree-sitter conventions.

## Risks / Trade-offs

- **[Risk]** `tree_sitter::Parser` is not `Send`/`Sync` in all versions, which could complicate parallel execution later.  
  **Mitigation**: Create a parser per task when parallelism is introduced; this change keeps the wrapper simple.
- **[Risk]** Storing the full source in `ParseError` duplicates memory on failure.  
  **Mitigation**: Parse errors are the exceptional path; the extra copy is acceptable for clear diagnostics.
- **[Risk]** Parser reuse across multiple calls may retain state.  
  **Mitigation**: Use `parse(source, None)` with no old tree; the wrapper can create a fresh parser if needed.

## Open Questions

- Should column numbers be byte offsets or UTF-8 character counts? (Start with byte offsets for simplicity and consistency with tree-sitter.)
- Should `parse_file` accept `&str`, `&Path`, or `PathBuf`? (Use `&Path` or `AsRef<Path>` for flexibility.)
