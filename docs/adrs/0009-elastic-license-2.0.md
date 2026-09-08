# ADR-009: License Under Elastic License 2.0

## Status

Superseded by [ADR-010: License under Apache License 2.0](0010-apache-license-2.0.md)

## Context

We need to choose a license for `lua-mutation-test`. The copyright holder wants a
standard license that allows ordinary use, modification, and distribution —
including workplace use — while preventing third parties from offering the Software
as a commercial hosted or managed service.

## Decision Drivers

- Allow personal, educational, research, and workplace use.
- Allow modification and redistribution.
- Reserve the right to offer the Software as a hosted or managed service.
- Use a widely recognized, standard license.

## Considered Options

### Option 1: MIT License

- **Pros**: Very permissive, widely understood, allows commercial use.
- **Cons**: Does not reserve SaaS/hosted service rights.

### Option 2: Elastic License 2.0

- **Pros**: Standard license used by Elasticsearch and others; permits use,
  modification, and distribution including workplace use; explicitly prohibits
  providing the Software as a hosted or managed service.
- **Cons**: Not OSI-approved; some users may need to review the terms.

### Option 3: Server Side Public License (SSPL)

- **Pros**: Standard license; strongly protects against managed service offerings.
- **Cons**: More controversial; requires open-sourcing derivative works when offering
  as a service.

### Option 4: PolyForm Noncommercial 1.0.0

- **Pros**: Standard, noncommercial-only license.
- **Cons**: Prohibits ordinary commercial workplace use, which we want to allow.

## Decision

We will license the project under the **Elastic License 2.0**.

## Rationale

Elastic License 2.0 is a standard, lawyer-drafted license that permits the use
cases we want to encourage (including workplace use) while clearly reserving the
right to offer the Software as a hosted or managed service. It is simpler and more
widely recognized than SSPL for this use case.

## Consequences

### Positive

- Workplace and internal commercial use are permitted.
- Modification and redistribution are permitted.
- Third parties cannot offer the Software as a managed service without permission.
- Recognized by SPDX as `Elastic-2.0`.

### Negative

- Not an OSI-approved open-source license.
- Some users may need to review the hosted/managed service restriction.

## Implementation Notes

- `LICENSE` contains the full Elastic License 2.0 text.
- `Cargo.toml` uses `license = "Elastic-2.0"`.
- `README.md` links to the license and summarizes the terms.

## References

- [Elastic License 2.0](https://www.elastic.co/licensing/elastic-license)
- [SPDX Elastic-2.0](https://spdx.org/licenses/Elastic-2.0.html)
