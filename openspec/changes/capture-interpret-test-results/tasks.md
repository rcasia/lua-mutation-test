## 1. Domain Model

- [x] 1.1 Define `MutantResult` enum with `killed`, `survived`, `timed_out`, and `error` variants
- [x] 1.2 Add captured stdout/stderr snippet fields to `MutantResult`
- [x] 1.3 Define a runner failure signal type to distinguish runner errors from test failures

## 2. Result Interpreter

- [x] 2.1 Implement interpreter function mapping exit code 0 to `survived`
- [x] 2.2 Implement interpreter function mapping non-zero exit code to `killed`
- [x] 2.3 Implement interpreter function mapping timeout to `timed_out`
- [x] 2.4 Implement interpreter function mapping runner failure to `error`
- [x] 2.5 Bound and store stdout/stderr snippets in the interpreted result

## 3. Tests

- [x] 3.1 Add unit tests for each exit-code mapping
- [x] 3.2 Add unit tests for timeout and error mappings
- [x] 3.3 Add unit tests verifying output snippet capture
- [x] 3.4 Add unit tests distinguishing runner failure from test failure

## 4. Documentation

- [x] 4.1 Document the default exit-code mapping behavior
- [x] 4.2 Note the future hook for framework-specific output parsing
