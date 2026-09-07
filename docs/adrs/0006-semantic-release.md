# ADR-006: Automate Releases with semantic-release

## Status

Accepted

## Context

We want to release new versions of `lua-mutation-test` automatically based on
conventional commits, without manual version bumping or changelog writing.

## Decision Drivers

- Automate version calculation and GitHub releases.
- Enforce consistent commit message conventions.
- Reduce release friction.

## Considered Options

### Option 1: semantic-release

- **Pros**: Mature, widely used, supports conventional commits, generates changelogs,
  creates GitHub releases.
- **Cons**: Opinionated about versioning (defaults to 1.0.0); requires Node.js in CI.

### Option 2: Manual releases

- **Pros**: Full control over version and release notes.
- **Cons**: Error-prone, inconsistent, time-consuming.

### Option 3: Cargo release tooling (e.g., `cargo-release`, `release-plz`)

- **Pros**: Rust-native, integrates with Cargo.
- **Cons**: Less ecosystem maturity than semantic-release; not as widely used in this
  project's reference projects.

## Decision

We will use **semantic-release** driven by **conventional commits**, configured for
ZeroVer releases.

## Rationale

semantic-release is proven, well-documented, and matches the release workflow used in
`rcasia/neotest-java`. By configuring the commit analyzer for ZeroVer, we can stay in
the 0.x range while still benefiting from automated changelog generation and GitHub
releases.

## Consequences

### Positive

- Automated version calculation and releases.
- Generated changelogs and GitHub release notes.
- Encourages consistent conventional commits.

### Negative

- Requires Node.js in CI.
- Needs careful configuration to stay in 0.x (see ADR-004).

## Implementation Notes

- `.releaserc.json` configures the plugins and ZeroVer release rules.
- `.github/workflows/release.yml` runs tests and then semantic-release on a schedule.
- The initial tag `v0.0.0` ensures semantic-release starts from 0.x.

## References

- [semantic-release](https://semantic-release.org/)
- ADR-004: Use ZeroVer (0-based versioning)
