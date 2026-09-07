# ADR-005: Use Trunk-Based Development

## Status

Accepted

## Context

We need a branching strategy for `lua-mutation-test`. The team is small and wants to
minimize integration overhead while keeping the main branch always deployable.

## Decision Drivers

- Small team; long-lived branches create unnecessary overhead.
- Desire for continuous integration and fast feedback.
- Need to keep CI green on the main branch.

## Considered Options

### Option 1: Trunk-based development

- **Pros**: Simple, fast integration, no merge hell, CI always reflects the latest
  state.
- **Cons**: Requires discipline to keep commits small and the trunk green.

### Option 2: Git Flow

- **Pros**: Structured release process with develop and feature branches.
- **Cons**: Too heavy for a small project; release branches add overhead.

### Option 3: GitHub Flow (short-lived PR branches)

- **Pros**: PR-based reviews, simple branch model.
- **Cons**: Still involves branch management and PR overhead.

## Decision

We will use **trunk-based development**. Work is done directly on or rebased into the
`main` branch. Commits are small and focused, and the trunk must stay green.

## Rationale

For a small, fast-moving project, trunk-based development minimizes process overhead
and ensures that the latest code is always integrated and tested.

## Consequences

### Positive

- Simplified workflow.
- Fast feedback from CI.
- Reduced merge conflicts.

### Negative

- Requires discipline to avoid pushing broken code.
- Less formal review gate than pull requests.

## Implementation Notes

- Default branch is `main`.
- CI runs on every push to `main`.
- Contributors should pull/rebase before pushing and ensure tests pass.

## References

- [Trunk-Based Development](https://trunkbaseddevelopment.com/)
