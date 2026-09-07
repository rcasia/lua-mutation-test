## ADDED Requirements

### Requirement: MutationScore contains category counts and overall percentage
The system SHALL define a `MutationScore` struct that holds counts for `killed`, `survived`, `timed_out`, `error`, and `skipped` mutants, and computes an overall percentage.

#### Scenario: Score with only killed and survived mutants
- **WHEN** results contain 8 killed and 2 survived mutants with no errors or timeouts
- **THEN** the overall score is 80%

### Requirement: Overall score excludes errors and timeouts by default
The system SHALL calculate the overall mutation score as `killed / (total - errors - timeouts)` by default.

#### Scenario: Score with errors and timeouts present
- **WHEN** results contain 6 killed, 2 survived, 1 timed_out, and 1 error
- **THEN** the overall score is 75% (6 / 8)

### Requirement: Mutants are categorized
The system SHALL categorize each mutant result as exactly one of `killed`, `survived`, `timed_out`, `error`, or `skipped`.

#### Scenario: Categorize a timeout result
- **WHEN** a result is `timed_out`
- **THEN** it contributes to the `timed_out` count and not to `killed` or `survived`

### Requirement: Per-file scores are provided
The system SHALL provide a `MutationScore` for each source file.

#### Scenario: Aggregate results by file
- **WHEN** results include mutants from multiple files
- **THEN** the system returns a separate score for each file

### Requirement: Per-operator scores are provided
The system SHALL provide a `MutationScore` for each mutation operator.

#### Scenario: Aggregate results by operator
- **WHEN** results include mutants produced by different operators
- **THEN** the system returns a separate score for each operator

### Requirement: Per-function scores are provided when location metadata exists
The system SHALL provide a `MutationScore` for each function when function location metadata is available.

#### Scenario: Mutants inside functions
- **WHEN** results include mutants located inside named functions
- **THEN** the system returns a separate score for each function

#### Scenario: Mutants outside functions
- **WHEN** results include mutants outside any named function
- **THEN** the system groups them under a fallback entry

### Requirement: Score data is exposed for reporting
The system SHALL expose score data as a plain data structure that reporting modules can consume without formatting logic.

#### Scenario: Reporter consumes score data
- **WHEN** the scoring module returns score data
- **THEN** a reporter can read overall, per-file, per-operator, and per-function scores
