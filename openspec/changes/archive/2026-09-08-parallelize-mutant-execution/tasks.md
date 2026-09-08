## 1. Concurrency approach and dependencies

- [x] 1.1 Evaluate `rayon`, `tokio`, and `std::thread` against the project needs
- [x] 1.2 Add the chosen concurrency dependency to `Cargo.toml`

## 2. Core worker pool implementation

- [x] 2.1 Create a worker pool abstraction for executing mutant jobs
- [x] 2.2 Route each generated mutant through the worker pool
- [x] 2.3 Allocate an isolated temporary directory per worker or per job

## 3. Configuration and CLI

- [x] 3.1 Add a `--workers <n>` CLI argument
- [x] 3.2 Default the worker count to the number of logical CPUs

## 4. Progress reporting and signal handling

- [x] 4.1 Emit progress updates during parallel execution
- [x] 4.2 Handle `SIGINT` for graceful shutdown

## 5. Tests and benchmarks

- [x] 5.1 Add unit tests for the worker pool
- [x] 5.2 Add an integration test verifying deterministic results
- [x] 5.3 Add a benchmark comparing sequential vs parallel execution
