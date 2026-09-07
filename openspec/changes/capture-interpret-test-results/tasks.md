## 1. Domain Model

- [ ] 1.1 Define `MutantResult` enum with `killed`, `survived`, `timed_out`, and `error` variants
- [ ] 1.2 Add captured stdout/stderr snippet fields to `MutantResult`
- [ ] 1.3 Define a runner failure signal type to distinguish runner errors from test failures

## 2. Result Interpreter

- [ ] 2.1 Implement interpreter function mapping exit code 0 to `survived`
- [ ] 2.2 Implement interpreter function mapping non-zero exit code to `killed`
- [ ] 2.3 Implement interpreter function mapping timeout to `timed_out`
- [ ] 2.4 Implement interpreter function mapping runner failure to `error`
- [ ] 2.5 Bound and store stdout/stderr snippets in the interpreted result

## 3. Tests

- [ ] 3.1 Add unit tests for each exit-code mapping
- [ ] 3.2 Add unit tests for timeout and error mappings
- [ ] 3.3 Add unit tests verifying output snippet capture
- [ ] 3.4 Add unit tests distinguishing runner failure from test failure

## 4. Documentation

- [ ] 4.1 Document the default exit-code mapping behavior
- [ ] 4.2 Note the future hook for framework-specific output parsing
