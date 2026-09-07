# ADR-009: License Under PolyForm Noncommercial 1.0.0

## Status

Accepted

## Context

We need to choose a license for `lua-mutation-test`. The project is open for
noncommercial use, modification, and distribution, but the copyright holder wants to
reserve all commercialization rights.

## Decision Drivers

- Allow personal, educational, and research use.
- Allow modification and redistribution.
- Reserve all commercial use, sale, and SaaS/cloud offering rights to the copyright
  holder.
- Use a standard, lawyer-drafted license rather than a custom one.

## Considered Options

### Option 1: MIT License

- **Pros**: Very permissive, widely understood, allows commercial use.
- **Cons**: Does not reserve commercialization rights.

### Option 2: PolyForm Noncommercial License 1.0.0

- **Pros**: Standard, noncommercial-only license; permits use, modification, and
  distribution; reserves commercial rights to the licensor.
- **Cons**: Less widely known than MIT; some users may be unfamiliar with it.

### Option 3: Commons Clause with MIT/Apache

- **Pros**: Adds a commercial restriction to a well-known license.
- **Cons**: Not a standalone license; considered controversial and incompatible with
  open-source definitions.

### Option 4: Custom license

- **Pros**: Tailored to exact requirements.
- **Cons**: Requires legal review; may be unclear or unenforceable.

## Decision

We will license the project under the **PolyForm Noncommercial License 1.0.0**.

## Rationale

PolyForm Noncommercial is a standard, well-drafted license that explicitly permits
noncommercial use, modification, and distribution while reserving all commercial rights
to the copyright holder. It avoids the ambiguity and legal risk of a custom license.

## Consequences

### Positive

- Clear, standard license terms.
- Commercialization rights remain with the copyright holder.
- Recognized by SPDX as `PolyForm-Noncommercial-1.0.0`.

### Negative

- Commercial users must contact the copyright holder for a license.
- Some open-source communities may not consider it "open source".

## Implementation Notes

- `LICENSE` contains the full PolyForm Noncommercial text with a required copyright
  notice.
- `Cargo.toml` uses `license = "PolyForm-Noncommercial-1.0.0"`.
- `README.md` links to the license and summarizes the terms.

## References

- [PolyForm Noncommercial License 1.0.0](https://polyformproject.org/licenses/noncommercial/1.0.0)
- [SPDX PolyForm-Noncommercial-1.0.0](https://spdx.org/licenses/PolyForm-Noncommercial-1.0.0.html)
