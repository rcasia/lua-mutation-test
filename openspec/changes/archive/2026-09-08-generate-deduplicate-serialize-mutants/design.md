## Context

Mutants are currently produced implicitly by individual mutation operators. This change centralizes the `Mutant` type and the generation pipeline so the rest of the system has a consistent view of what mutants exist, where they are located, and how they transform the source.

## Goals / Non-Goals

**Goals:**
- Define a `Mutant` struct with id, file path, operator, byte range, line/column, original text, and replacement text.
- Build a `MutantGenerator` that composes mutators and produces mutants for a parsed file.
- Deduplicate mutants by `(file, location, replacement)`.
- Serialize and deserialize mutants to JSON.
- Provide a CLI command to list mutants without running tests.

**Non-Goals:**
- Implementing every possible mutation operator.
- Running mutants or scoring results.
- Storing mutants in a database or long-term cache format beyond JSON.

## Decisions

- **Stable IDs derived from content.** Compute the mutant id as a deterministic hash of `(file_path, start_byte, end_byte, replacement)`. This avoids random UUIDs and makes caching and incremental runs reproducible.
- **Generator owns deduplication.** Operators emit candidates; the generator filters duplicates before exposing results. This keeps operators simple and places the deduplication policy in one location.
- **Use a trait for mutators.** A `Mutator` trait with a `generate(&self, source: &str, tree: &Tree) -> Vec<CandidateMutant>` interface allows new operators to be added without changing the generator.
- **JSON serialization via `serde`.** serde is already common in the Rust ecosystem and lets users inspect mutants with standard tools.
- **CLI list command prints JSON lines by default.** This is easy to consume from scripts and tests; a human-readable format can be added later.

## Risks / Trade-offs

- **[Risk]** Hash collisions in content-based IDs.  
  **Mitigation:** Use a 64-bit or 128-bit hash (e.g., xxhash or Blake3 truncated). Document collision probability and prefer Blake3 if collision resistance is critical.
- **[Risk]** Deduplication by replacement text is sensitive to whitespace.  
  **Mitigation:** Use byte ranges directly from the tree-sitter nodes so identical source spans produce identical replacements.
- **[Risk]** Large files may generate many mutants and memory pressure.  
  **Mitigation:** Generator returns a `Vec`; streaming can be introduced later without changing the public API.
