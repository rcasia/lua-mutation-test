## Why

Before the mutation testing tool can execute a test suite, it must know which Lua files are tests and how to run them. Hard-coding a single test command limits adoption across projects that use `busted`, `luaunit`, or custom scripts. Detecting test files and providing framework adapters makes the tool usable out of the box for the most common Lua testing workflows.

## What Changes

- Add a test-discovery component that scans the project for Lua test files.
- Support default glob patterns: `*_spec.lua`, `*_test.lua`, and `test_*.lua`.
- Allow users to override default patterns via configuration.
- Implement framework adapters:
  - `busted` adapter that invokes the `busted` command.
  - `luaunit` adapter that invokes `lua` with the luaunit test file.
  - Generic shell-command adapter for custom test runners.
- Run the discovered tests against the original, unmutated source to establish a baseline.
- Store the baseline result and make it available to the mutant runner.

## Capabilities

### New Capabilities
- `lua-test-discovery`: Discover Lua test files and run them through configurable framework adapters to establish a baseline.

### Modified Capabilities
<!-- No existing capabilities require requirement-level changes. -->

## Impact

- New modules for discovery, adapter abstraction, and baseline execution.
- Configuration surface expanded to accept test file globs and framework selection.
- Dependency on the existing CLI configuration and test-result capture components.
