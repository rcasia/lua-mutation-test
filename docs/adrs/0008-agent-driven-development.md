# ADR-008: Agent-Driven Development

## Status

Accepted

## Context

As the project matured, development transitioned from manual authoring to being
primarily driven by AI coding agents. This changes how decisions, code, and
documentation are produced and reviewed.

## Decision Drivers

- Accelerate development by leveraging agentic tooling.
- Maintain consistency and traceability of agent-generated changes.
- Document the transition so future contributors understand the project's history.

## Decision

Starting from commit `2d74bf1` ("ci: rename default branch from master to main in
workflow"), this project is developed and maintained **completely by AI coding
agents**. Human oversight is exercised at the goal and review level, while
implementation, documentation, and release automation are agent-driven.

## Consequences

### Positive

- Rapid iteration and broad feature coverage.
- Consistent application of conventions (Conventional Commits, trunk-based dev, ADRs).
- Documentation stays in sync with code through explicit agent instructions.

### Negative

- Requires careful review of agent-generated output.
- May produce over-engineered or speculative code if goals are not precise.
- Agent coordination can cause race conditions or transient CI failures.

## Implementation Notes

- `AGENTS.md` contains the conventions and documentation-update rules agents must
  follow.
- ADRs capture significant technical decisions made by or with agents.
- Git history records the agent-driven commits from `2d74bf1` onward.

## References

- `AGENTS.md`
- Git history starting at `2d74bf1`
