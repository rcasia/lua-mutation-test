# Contributing to lua-mutation-test

Thanks for your interest in contributing! This project is a work in progress, and
all feedback, bug reports, and pull requests are welcome.

## Development setup

1. **Clone the repository**:

   ```bash
   git clone git@github.com:rcasia/lua-mutation-test.git
   cd lua-mutation-test
   ```

2. **Fetch the vendored tree-sitter Lua grammar**:

   ```bash
   ./scripts/fetch-tree-sitter-lua.sh
   ```

   The grammar is cloned into `tree-sitter-lua/` (gitignored).

3. **Build the project**:

   ```bash
   cargo build
   ```

4. **Run the tests**:

   ```bash
   cargo test
   ```

5. **Run the benchmarks** (optional):

   ```bash
   cargo bench
   ```

## Workflow

This project follows **trunk-based development**:

- Work on `main` or on a very short-lived branch rebased onto `main`.
- Keep commits small and focused.
- Apply **vertical slices**: implement features end-to-end through the minimum
  necessary layers rather than building horizontal layers in isolation.
- Pull/rebase `origin/main` before pushing.
- Ensure CI is green before and after your push.

## Commit messages

Use [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` — new feature
- `fix:` — bug fix
- `refactor:` — code change that neither fixes a bug nor adds a feature
- `perf:` — performance improvement
- `chore:` — maintenance tasks
- `ci:` — CI/CD changes
- `docs:` — documentation changes
- `test:` — adding or updating tests

## Pre-commit hooks

This project uses [pre-commit](https://pre-commit.com/) to run checks before each
commit. Install it with:

```bash
pip install pre-commit
pre-commit install
```

The configured hooks run file hygiene checks, `cargo fmt`, `cargo clippy`,
`cargo test`, and `mkdocs build`. Make sure you have the Rust toolchain and
MkDocs dependencies installed (`pip install -r docs/requirements.txt`) so all
hooks can run.

## Documentation

Documentation lives in `docs/` and is published with MkDocs. When you change
build steps, CLI usage, architecture, or reporting behavior, update the matching
pages in `docs/` and, if necessary, `mkdocs.yml`.

See `AGENTS.md` for the mapping between code changes and documentation updates.

## License

By contributing, you agree that your contributions will be licensed under the
[Apache License 2.0](LICENSE).
