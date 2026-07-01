# Symbolic Math

Symbolic Math is the proof-producing mathematics kernel for the Socrates
Academy ecosystem.

The project is not intended to be only a faster general-purpose computer
algebra system. Its primary purpose is to represent mathematical work in a
form that can be computed, rendered, explained, replayed, and validated for
pedagogic use.

## Product Goal

Build a high-performance symbolic mathematics library that can:

- parse a constrained mathematical LaTeX profile;
- represent expressions, statements, assumptions, and derivations explicitly;
- simplify expressions and statements with recorded reasons;
- evaluate expressions and statements exactly when enough values are known;
- determine whether two mathematical objects are equivalent under assumptions;
- solve useful classes of equations;
- validate learner-entered steps against prior mathematical state;
- render deterministic KaTeX-compatible LaTeX;
- expose a stable TypeScript API backed by a Rust and WebAssembly kernel.

## First Vertical Slice

The first complete workflow is single-variable linear equations over exact
rational numbers.

Example:

```latex
3(x - 2) + 4 = 2x + 9
```

The browser-facing API must be able to parse the equation, solve it, return a
structured derivation, validate learner steps, serialize the result, and render
each mathematical object as KaTeX-compatible LaTeX.

## Architectural Principle

Every mathematical transformation should be able to answer:

- what changed;
- where it changed;
- why it was valid;
- which assumptions were required;
- what semantic relation was preserved;
- how the step can be independently verified.

Explanations are generated from derivation data. They are not the derivation.

## Project Documents

- [SPEC.md](./SPEC.md) defines the initial architecture and domain contract.
- [features.md](./features.md) stores the BDD feature catalog.
