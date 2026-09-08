## ADDED Requirements

### Requirement: Detect obviously equivalent arithmetic-identity mutants
The system SHALL detect and skip mutants that replace an expression with an equivalent arithmetic identity (e.g., `x + 0`, `x * 1`).

#### Scenario: Addition identity
- **WHEN** the original expression is `x + 0` and a mutation produces `x - 0`
- **THEN** the system flags the mutant as equivalent and does not execute it

#### Scenario: Multiplication identity
- **WHEN** the original expression is `x * 1` and a mutation produces `x / 1`
- **THEN** the system flags the mutant as equivalent and does not execute it

#### Scenario: Subtraction identity
- **WHEN** the original expression is `x - 0`
- **THEN** any mutation that leaves the expression semantically identical SHALL be flagged as equivalent

### Requirement: Flag likely-equivalent mutants
The system SHALL flag mutants that are likely equivalent for reviewer attention.

#### Scenario: Equivalent mutant in report
- **WHEN** a mutant is classified as likely equivalent
- **THEN** the report includes the mutant as equivalent and provides the heuristic reason

### Requirement: Integration test fixtures
The system SHALL provide sample Lua projects under `tests/fixtures/` for integration tests.

#### Scenario: Fixture project present
- **WHEN** integration tests are run
- **THEN** the system loads sample Lua projects from `tests/fixtures/`

### Requirement: Integration test suite
The system SHALL include integration tests that run mutation testing against sample Lua projects.

#### Scenario: End-to-end integration test
- **WHEN** the integration test suite executes
- **THEN** it runs the mutation tool against the fixtures and asserts expected outcomes

### Requirement: Benchmark suite
The system SHALL provide benchmarks measuring mutation execution time.

#### Scenario: Benchmark execution
- **WHEN** the benchmark suite runs
- **THEN** it reports execution time for mutation testing on representative inputs

### Requirement: CI integration tests
The system SHALL run integration tests in a CI workflow.

#### Scenario: CI workflow on pull request
- **WHEN** a pull request is opened or updated
- **THEN** the CI pipeline executes the integration tests

### Requirement: Document equivalent-mutant cases
The system SHALL document known equivalent-mutant cases.

#### Scenario: Documentation present
- **WHEN** a user reads the mutation testing documentation
- **THEN** they find a section listing known equivalent-mutant patterns and heuristics
