## Context

The project parses Lua source files with tree-sitter-lua. Binary expressions expose an operator child that can be matched by text/kind. Existing mutant generation infrastructure supports adding new mutators that traverse the AST and emit `Mutant` values.

## Goals / Non-Goals

**Goals:**
- Implement `ArithmeticOperatorMutator` that matches `binary_expression` nodes using arithmetic operators.
- Generate replacements for `+`, `-`, `*`, `/`, `%`, `//`, and `^` per the acceptance criteria.
- Record original operator, replacement operator, and byte-level source location for every mutant.

**Non-Goals:**
- Type-aware or value-aware mutation (e.g., avoiding division by zero at runtime).
- Mutation of compound assignment operators (`+=`, `-=`, etc.) or unary minus.
- Full equivalence detection.

## Decisions

- **Single mutator for all arithmetic operators**: Keeps the operator-to-replacements lookup localized and easy to extend. A dispatch table maps each operator to its candidate replacements.
- **Match by operator text**: tree-sitter-lua stores the operator token as a child of `binary_expression`. Matching the literal text (`+`, `-`, etc.) is the simplest portable approach.
- **Replace only the operator token**: A mutant stores the original operator range and replacement text; the serialized mutant applies a single-token edit. This reuses existing source-map utilities.
- **Optional runtime-error skipping deferred**: The acceptance criteria mark this as optional for v1. The initial implementation emits all replacements; a later filter can remove mutants that are statically known to error.

## Risks / Trade-offs

- **Equivalent mutants**: Some replacements (e.g., `x * 1` to `x / 1`) may not change behavior. This is accepted and documented as a known limitation.
- **Runtime errors**: Replacements such as division by zero are not statically prevented in v1; they will be caught when the mutant runs against tests.
