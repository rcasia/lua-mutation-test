# ADR-007: Apply Vertical Slices for Feature Development

## Status

Accepted

## Context

`lua-mutation-test` has several conceptual layers: CLI, Lua parser, mutant generator,
test runner, and reporter. We need a development strategy that lets us deliver value
incrementally while the architecture is still emerging.

## Decision Drivers

- Validate the end-to-end flow early.
- Avoid building fully general layers that may not be needed.
- Keep the project shippable and testable at all times.
- Make it easier to refactor internals because each slice has a working integration
  point.

## Considered Options

### Option 1: Horizontal layers

Build each layer completely before moving to the next (e.g., finish all parsing
utilities, then all mutation operators, then the runner, then reporting).

- **Pros**: Clean separation of concerns; each layer can be designed in depth.
- **Cons**: Long time before anything works end-to-end; risk of over-engineering
  layers; harder to validate assumptions.

### Option 2: Vertical slices

Implement each feature end-to-end, touching all layers minimally needed to make it
work.

- **Pros**: Fast feedback, working software after each slice, easier to pivot,
  surfaces integration issues early.
- **Cons**: Initial architecture may be less polished; requires periodic refactoring
  to keep slices coherent.

### Option 3: Hybrid approach

Start with a thin horizontal foundation, then build vertical slices on top.

- **Pros**: Balances early structure with incremental delivery.
- **Cons**: Can blur into horizontal development if the "foundation" grows too large.

## Decision

We will use **vertical slices** as the primary development approach. Each feature or
user-visible capability should be implemented end-to-end through the minimum necessary
layers, then refined and generalized based on real usage.

## Rationale

Vertical slices keep the project in a working state, reduce the risk of architectural
dead ends, and align with trunk-based development by enabling small, integrated
commits. They also make it easier for contributors to understand how a change affects
the whole system.

## Consequences

### Positive

- Working end-to-end features delivered quickly.
- Integration issues discovered early.
- Easier to demo and get feedback.
- Refactoring is guided by actual feature needs.

### Negative

- Early slices may require later refactoring as the architecture stabilizes.
- Requires discipline to avoid accumulating technical debt between slices.

## Implementation Notes

- Favor slice-sized GitHub issues that describe a user-visible outcome, not just a
  layer component.
- When adding a new mutation operator, include a minimal CLI path and a test that runs
  the operator against a sample file.
- When extending reporting, wire the new output format through the runner so it can be
  exercised end-to-end.
- Periodically refactor shared code that emerges from multiple slices into reusable
  modules.

## Related Decisions

- ADR-005: Trunk-based development

## References

- [Vertical Slice Architecture](https://en.wikipedia.org/wiki/Vertical_slice)
- [Feature Slicing](https://www.agilealliance.org/glossary/slicing/)
