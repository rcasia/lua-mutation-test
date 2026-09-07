# ADR-004: Use ZeroVer (0-Based Versioning)

## Status

Accepted

## Context

We need a versioning scheme for `lua-mutation-test` while the project is in early
development and the API is unstable.

## Decision Drivers

- Communicate that the project is not yet stable.
- Avoid frequent major-version bumps during rapid early iteration.
- Align with the maintainer's preference and the project's playful identity.

## Considered Options

### Option 1: Semantic Versioning (SemVer) starting at 1.0.0

- **Pros**: Clear contract for consumers; widely understood.
- **Cons**: Implies stability prematurely; every breaking change requires a major bump.

### Option 2: ZeroVer

- **Pros**: Clearly signals initial development; no pressure to reach 1.0.0; matches
  the philosophy at [0ver.org](https://0ver.org/).
- **Cons**: Not everyone understands 0.x semantics; some tools treat 0.x as unstable.

### Option 3: Pre-releases (e.g., 1.0.0-alpha.1)

- **Pros**: Communicates instability while keeping SemVer structure.
- **Cons**: More complex to manage; not as straightforward as staying in 0.x.

## Decision

We will follow **ZeroVer**: the major version will remain **0** for the foreseeable
future, with minor and patch versions incremented as the project evolves.

## Rationale

ZeroVer is a simple, explicit signal that the project is in initial development and
that APIs may change. It also avoids the overhead of frequent major-version releases
while the design is still settling.

## Consequences

### Positive

- Clear communication of project maturity.
- Simpler release automation (no major-version bumps).
- Aligns with a well-known (if tongue-in-cheek) versioning philosophy.

### Negative

- Some consumers may be hesitant to adopt a 0.x tool.
- Version numbers do not convey breaking changes as strongly as SemVer major bumps.

## Implementation Notes

- Releases are automated with `semantic-release` configured to stay in the 0.x range.
- The initial tag is `v0.0.0`.

## References

- [ZeroVer](https://0ver.org/)
- ADR-006: Automate releases with semantic-release
