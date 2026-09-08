## 1. Domain Model

- [x] 1.1 Define `Mutant` struct with id, file path, operator, byte range, line/column, original text, and replacement text
- [x] 1.2 Define a `Mutator` trait for operators to implement
- [x] 1.3 Define a `CandidateMutant` or equivalent intermediate type used by operators

## 2. Stable IDs

- [x] 2.1 Implement deterministic id generation from `(file_path, start_byte, end_byte, replacement)`
- [x] 2.2 Add tests verifying identical mutants receive identical ids across regeneration runs

## 3. MutantGenerator

- [x] 3.1 Implement `MutantGenerator` that accepts a list of mutators
- [x] 3.2 Implement generation pipeline that invokes each mutator on a parsed file
- [x] 3.3 Implement deduplication by `(file, location, replacement)`
- [x] 3.4 Add tests for generator composition and deduplication

## 4. Serialization

- [x] 4.1 Add serde `Serialize` and `Deserialize` derives to `Mutant`
- [x] 4.2 Add JSON round-trip tests for `Mutant`
- [x] 4.3 Add JSON round-trip tests for collections of mutants

## 5. CLI List Command

- [x] 5.1 Add a subcommand to list mutants for a given file or project
- [x] 5.2 Print listed mutants as JSON lines
- [x] 5.3 Add integration tests for the list subcommand

## 6. Documentation

- [x] 6.1 Document the `Mutant` and `MutantGenerator` APIs
- [x] 6.2 Document the id generation scheme and collision considerations
