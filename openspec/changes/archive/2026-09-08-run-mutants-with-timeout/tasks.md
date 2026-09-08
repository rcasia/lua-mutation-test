## 1. Setup

- [x] 1.1 Create the mutant runner module.
- [x] 1.2 Add configuration fields for timeout duration.
- [x] 1.3 Define the mutant outcome enum (`killed`, `survived`, `timed_out`, `errored`).

## 2. Process Isolation and Execution

- [x] 2.1 Spawn the configured test command as a separate process per mutant.
- [x] 2.2 Apply the mutant to a temporary source file before execution.
- [x] 2.3 Capture stdout, stderr, and exit status from the child process.
- [x] 2.4 Add unit tests for successful and failing mutant runs.

## 3. Timeout Handling

- [x] 3.1 Implement a 30-second default timeout.
- [x] 3.2 Implement configurable timeout parsing.
- [x] 3.3 Terminate the child process when the timeout is reached.
- [x] 3.4 Record the mutant result as `timed_out`.
- [x] 3.5 Add unit tests for timeout behavior.

## 4. Cleanup and Error Handling

- [x] 4.1 Delete temporary files after each run regardless of outcome.
- [x] 4.2 Handle process-spawning errors gracefully.
- [x] 4.3 Record spawn failures as `errored` and continue with the next mutant.
- [x] 4.4 Add unit tests for cleanup and error handling.

## 5. Integration

- [x] 5.1 Wire the mutant runner into the main mutation testing pipeline.
- [x] 5.2 Verify end-to-end execution on a sample Lua project.
