# Consumer Exercises Spec

This package is a downstream-consumer compatibility harness for exercise systems.
It intentionally imports `@socrates/math` through the public package API instead
of reaching into source files or Rust crates.

The specs model the kind of adapters an exercises package would write for answer
grading: normalize expression input, compare against accepted answers, solve
equations, compare equivalent solution sets, preserve diagnostics, and keep
enough structured detail for pedagogic feedback.

Covered exercise shapes:

- `math_expression`: accepts LaTeX input, normalizes it, and compares it with one
  or more stored correct expressions. Example: `3(x - 2) + 4` and `3x - 2`
  are equivalent expressions because they normalize to the same expression.
- `linear_equation`: compares a submitted equation with a saved answer by
  solution set, so equivalent equations can grade correctly.
- `math_equation`: compares equations by solution set. Example: `x + 1 = 3`
  and `2x = 4` are equivalent equations because both have solution set `x = 2`.
  This differs from `math_expression`, where there is no equals sign and the
  comparison is between expression values/forms such as `2(x + 1)` and `2x + 2`.
- `numeric_answer`: compares numeric LaTeX input against an expected numeric
  value with an absolute tolerance.
- `naive_set_theory`: currently a forward-looking consumer contract for the set
  theory course. It catalogs the exercise types the engine must eventually
  support: finite set normalization, membership, subset claims, set operations,
  cardinality, power sets, Cartesian products, bounded set-builder notation,
  Venn-region translation, identities, relations, functions, indexed families,
  and proof-step validation.
