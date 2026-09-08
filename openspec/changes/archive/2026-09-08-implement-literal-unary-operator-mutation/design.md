## Context

Lua literals appear as dedicated AST nodes (`true`, `false`, `nil`, `numeral`, `string`) and unary operators appear as `unary_expression` nodes with an operator child. The existing mutator pipeline expects each mutator to traverse the AST and emit `Mutant` values.

## Goals / Non-Goals

**Goals:**
- Implement `LiteralMutator` for boolean, number, string, and `nil` literals.
- Implement `UnaryOperatorMutator` for `-`, `not`, and `#`.
- Record replacement text and source location for every mutant.
- Document known equivalent mutants.

**Non-Goals:**
- String append/prepend variants in v1 (optional in acceptance criteria).
- Removing or replacing `nil` in contexts where it would create invalid syntax.
- Type-aware value analysis.

## Decisions

- **Single `LiteralMutator` with node-kind dispatch**: Boolean, number, string, and `nil` are all literals, so one mutator can branch on node kind. A small dispatch table maps each kind to its replacement function.
- **Number mutation by text transformation**: Increment/decrement the parsed number, flip its sign, or replace it with `0`/`1`. This avoids serializing full AST changes.
- **String mutation to empty string**: Replace the string literal token text with `""`. Append/prepend variants are deferred to a later iteration.
- **Unary mutation by operand replacement**: For `unary_expression` nodes with `-`, `not`, or `#`, emit a mutant that replaces the whole expression with the operand. This is the minimal syntactically valid edit.
- **Nil replacement limited to `false`**: Replacing `nil` with `false` is safe in most expression contexts. Removing `nil` entirely is skipped in v1 to avoid invalid syntax.

## Risks / Trade-offs

- **Equivalent mutants**: Literal mutation is especially prone to equivalence (e.g., `true` in a context already forced to boolean). Cases will be documented rather than filtered in v1.
- **Invalid syntax from nil removal**: Removing a `nil` literal can break constructs like `return nil`. Keeping removal out of v1 avoids this risk.
