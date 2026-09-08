## Context

The project currently has a minimal `src/main.rs` that only demonstrates loading the tree-sitter Lua parser. There is no command-line interface, so users cannot invoke the tool or discover its capabilities.

## Goals / Non-Goals

**Goals:**
- Define a small, consistent CLI surface using `clap` derive macros.
- Parse global options and three subcommands: `run`, `list-operators`, `init`.
- Produce meaningful exit codes and helpful help text with examples.
- Wire argument parsing into the binary entry point.

**Non-Goals:**
- Fully implementing the `run`, `list-operators`, or `init` workflows; this change is only the entry point and dispatch layer.
- Designing the full configuration file schema (a minimal sample is sufficient for `init`).
- Adding logging/tracing infrastructure beyond `--verbose` and `--quiet` flags.

## Decisions

- **Use `clap` derive macros** over a manual builder API. Derive macros are concise, keep flags and subcommands in one place, and generate help text automatically.
- **Split CLI types into `src/cli.rs`**. Keeping the argument definitions separate from `main.rs` makes them easier to unit test and avoids cluttering the entry point.
- **Exit codes**: `0` success, `1` test failures / survived mutants, `2` CLI errors. This mirrors common Unix conventions and gives scripts a clear signal.
- **Top-level options as global flags**: `--config`, `--verbose`, `--quiet`, `--version`, and `--help` apply before any subcommand.
- **`run` accepts a positional path plus optional flags**: the path is required, while `--test-command`, `--timeout`, and `--output` have sensible defaults or are optional.

## Risks / Trade-offs

- **[Risk]** Adding `clap` increases compile time and dependency surface.  
  **Mitigation**: Choose the default feature set and keep the CLI minimal; only enable features actually used.
- **[Risk]** Over-engineering the CLI before downstream workflows exist.  
  **Mitigation**: Accept only the flags listed in the acceptance criteria; placeholder handlers for subcommands.
- **[Risk]** Exit-code semantics may need adjustment once the run workflow is implemented.  
  **Mitigation**: Document the mapping in the spec so future changes are explicit.

## Open Questions

- What should the default test command be for `run`? (Deferred to the run workflow implementation.)
- Should `--output` accept only specific values (e.g., `json`, `html`) or a free-form path? (Deferred; for now treat as a string option.)
