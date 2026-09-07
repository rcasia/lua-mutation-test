## 1. Concurrency approach and dependencies

- [ ] 1.1 Evaluate `rayon`, `tokio`, and `std::thread` against the project needs
- [ ] 1.2 Add the chosen concurrency dependency to `Cargo.toml`

## 2. Core worker pool implementation

- [ ] 2.1 Create a worker pool abstraction for executing mutant jobs
- [ ] 2.2 Route each generated mutant through the worker pool
- [ ] 2.3 Allocate an isolated temporary directory per worker or per job

## 3. Configuration and CLI

- [ ] 3.1 Add a `--workers <n>` CLI argument
- [ ] 3.2 Default the worker count to the number of logical CPUs

## 4. Progress reporting and signal handling

- [ ] 4.1 Emit progress updates during parallel execution
- [ ] 4.2 Handle `SIGINT` for graceful shutdown

## 5. Tests and benchmarks

- [ ] 5.1 Add unit tests for the worker pool
- [ ] 5.2 Add an integration test verifying deterministic results
- [ ] 5.3 Add a benchmark comparing sequential vs parallel execution
