## 1. Domain Model

- [ ] 1.1 Define `Mutant` struct with id, file path, operator, byte range, line/column, original text, and replacement text
- [ ] 1.2 Define a `Mutator` trait for operators to implement
- [ ] 1.3 Define a `CandidateMutant` or equivalent intermediate type used by operators

## 2. Stable IDs

- [ ] 2.1 Implement deterministic id generation from `(file_path, start_byte, end_byte, replacement)`
- [ ] 2.2 Add tests verifying identical mutants receive identical ids across regeneration runs

## 3. MutantGenerator

- [ ] 3.1 Implement `MutantGenerator` that accepts a list of mutators
- [ ] 3.2 Implement generation pipeline that invokes each mutator on a parsed file
- [ ] 3.3 Implement deduplication by `(file, location, replacement)`
- [ ] 3.4 Add tests for generator composition and deduplication

## 4. Serialization

- [ ] 4.1 Add serde `Serialize` and `Deserialize` derives to `Mutant`
- [ ] 4.2 Add JSON round-trip tests for `Mutant`
- [ ] 4.3 Add JSON round-trip tests for collections of mutants

## 5. CLI List Command

- [ ] 5.1 Add a subcommand to list mutants for a given file or project
- [ ] 5.2 Print listed mutants as JSON lines
- [ ] 5.3 Add integration tests for the list subcommand

## 6. Documentation

- [ ] 6.1 Document the `Mutant` and `MutantGenerator` APIs
- [ ] 6.2 Document the id generation scheme and collision considerations
