## 1. Setup

- [ ] 1.1 Create the test discovery module.
- [ ] 1.2 Create the adapter trait/enum and module layout.
- [ ] 1.3 Add configuration fields for test glob patterns and framework selection.

## 2. Test Discovery

- [ ] 2.1 Implement default glob pattern matching (`*_spec.lua`, `*_test.lua`, `test_*.lua`).
- [ ] 2.2 Implement configurable glob pattern override.
- [ ] 2.3 Add unit tests for discovery with mixed file layouts.

## 3. Framework Adapters

- [ ] 3.1 Implement the `busted` adapter.
- [ ] 3.2 Implement the `luaunit` adapter.
- [ ] 3.3 Implement the generic shell-command adapter.
- [ ] 3.4 Add unit tests for each adapter invocation.

## 4. Baseline Run

- [ ] 4.1 Implement baseline execution against the original source.
- [ ] 4.2 Record and expose the baseline result.
- [ ] 4.3 Abort mutation testing when the baseline fails.
- [ ] 4.4 Add integration tests with fixture projects.
