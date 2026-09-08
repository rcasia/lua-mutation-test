# ADR-010: License Under Apache License 2.0

## Status

Accepted

## Context

`lua-mutation-test` was originally licensed under the Elastic License 2.0
([ADR-009](0009-elastic-license-2.0.md)). The copyright holder now wants to
adopt a permissive, OSI-approved open-source license to maximize adoption,
encourage contributions, and align with common practices in the Rust ecosystem.

## Decision Drivers

- Use a widely recognized, permissive open-source license.
- Be OSI-approved and recognized as free/open-source software.
- Provide explicit patent protection for users and contributors.
- Allow commercial use, modification, and distribution without restrictions.
- Remain compatible with the Rust/crates.io ecosystem.

## Considered Options

### Option 1: Apache License 2.0

- **Pros**: OSI-approved and widely used; provides a patent grant; permissive;
  compatible with commercial use; recognized by SPDX as `Apache-2.0`; standard
  choice for Rust projects.
- **Cons**: Does not reserve SaaS/hosted service rights; third parties can offer
  the Software as a service without restriction.

### Option 2: MIT License

- **Pros**: Very permissive, widely understood, simple, allows commercial use.
- **Cons**: Does not include an explicit patent grant; some organizations prefer
  Apache-2.0 for that reason.

### Option 3: Elastic License 2.0

- **Pros**: Already in use; permits use, modification, and distribution;
  explicitly prohibits providing the Software as a hosted or managed service.
- **Cons**: Not OSI-approved; limits adoption and contributions from projects
  that require an OSI-approved license.

### Option 4: Mozilla Public License 2.0 (MPL-2.0)

- **Pros**: OSI-approved; file-level copyleft; provides patent grant.
- **Cons**: Copyleft obligations are more complex than permissive licenses and
  may discourage adoption in proprietary codebases.

## Decision

We will license the project under the **Apache License 2.0**.

## Rationale

Apache License 2.0 is a permissive, OSI-approved license that encourages broad
adoption and contribution while providing an explicit patent grant. It aligns
with the licensing preferences of the Rust ecosystem and crates.io, and it
removes the hosted/managed service restriction that limited the project's
compatibility with open-source policies.

## Consequences

### Positive

- The project is recognized as OSI-approved open-source software.
- Commercial use, modification, and redistribution are permitted without
  restriction.
- Contributors and users receive an explicit patent grant.
- Alignment with the Rust/crates.io ecosystem improves adoption.

### Negative

- The copyright holder no longer reserves the exclusive right to offer the
  Software as a hosted or managed service.
- Third parties may compete by offering the Software as a service.

## Implementation Notes

- `LICENSE` contains the full Apache License 2.0 text.
- `Cargo.toml` uses `license = "Apache-2.0"`.
- `README.md` links to the license and summarizes the terms.
- [ADR-009](0009-elastic-license-2.0.md) is superseded by this ADR.

## References

- [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0)
- [SPDX Apache-2.0](https://spdx.org/licenses/Apache-2.0.html)
- [ADR-009: License Under Elastic License 2.0](0009-elastic-license-2.0.md)
