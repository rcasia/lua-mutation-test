## Why

Before mutants can be run or scored, the system needs a single, well-defined representation of a mutant and a deterministic way to generate them from parsed Lua source. Defining a central `Mutant` type, a generator pipeline, and stable serialization enables caching, inspection, and reuse across operators and test runs.

## What Changes

- Define a `Mutant` struct with stable id, file path, operator name, byte range, line/column, original text, and replacement text.
- Implement a `MutantGenerator` that accepts a list of mutators and produces mutants for a parsed file.
- Deduplicate generated mutants by the tuple `(file, location, replacement)`.
- Add JSON serialization and deserialization for mutants to support caching and offline inspection.
- Add a CLI subcommand to list generated mutants without executing tests.
- Add unit tests for the generator, deduplication, and serialization round-trips.

## Capabilities

### New Capabilities
- `mutant-generator`: Generates, deduplicates, and serializes mutants from a parsed Lua file using a configurable list of mutation operators.

### Modified Capabilities
- None.

## Impact

- Introduces new core domain types used by the mutation operators and the test runner.
- Adds a new CLI subcommand for mutant listing.
- Enables future caching and incremental mutation testing by providing stable IDs and JSON serialization.
