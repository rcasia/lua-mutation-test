## 1. Define configuration schema

- [x] 1.1 Define the configuration struct with fields for test command, framework, timeout, glob patterns, operators, parallelism, and output formats.
- [x] 1.2 Add a version field to the configuration schema.
- [x] 1.3 Document the schema in the project documentation.

## 2. Implement configuration parsing

- [x] 2.1 Implement TOML config parsing using the `toml` crate.
- [x] 2.2 Implement JSON config parsing fallback.
- [x] 2.3 Provide clear error messages for parse and validation failures.
- [x] 2.4 Add unit tests for valid and invalid configuration files.

## 3. Implement filtering logic

- [x] 3.1 Implement file include/exclude filtering using glob patterns.
- [x] 3.2 Implement operator include/exclude filtering by identifier.
- [x] 3.3 Implement function include/exclude filtering by name pattern.
- [x] 3.4 Add unit tests covering include, exclude, and precedence behavior.

## 4. Integrate configuration with CLI

- [x] 4.1 Add the `--config` CLI flag.
- [x] 4.2 Implement CLI flag precedence over configuration file values.
- [x] 4.3 Apply merged configuration to the run command.

## 5. Integrate filters with generator

- [x] 5.1 Pass file filters to the source discovery stage.
- [x] 5.2 Pass operator filters to the mutant generator.
- [x] 5.3 Pass function filters to the AST traversal stage.

## 6. Documentation and tests

- [x] 6.1 Add example configuration files to README and docs.
- [x] 6.2 Add integration tests for end-to-end config loading and filtering.
