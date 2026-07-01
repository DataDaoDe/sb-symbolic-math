# Symbolic Math Specification

## Purpose

Symbolic Math provides exact symbolic computation for Socrates Academy. The
kernel must be useful as a computer algebra system, but its defining capability
is pedagogic: it produces structured, verifiable mathematical derivations that
can support explanation, learner-step validation, and adaptive instruction.

## Initial Architecture

The intended dependency direction is:

```text
Frontend
-> @socrates/math
-> handwritten TypeScript facade
-> internal generated Wasm bindings
-> Rust application API
-> pure mathematical domain kernel
```

Generated WebAssembly binding types are never public API. They are adapter
details and may be replaced without breaking application code.

Initial package shape:

```text
crates/
  socrates-math-core
  socrates-math-engine
  socrates-math-latex
  socrates-math-wasm

packages/
  math
```

`packages/math-wasm` should be added only when packaging pressure proves it is
needed.

## Domain Boundaries

The system models these concepts separately:

- surface syntax;
- semantic expressions;
- statements, including equations and inequalities;
- mathematical context and assumptions;
- transformations;
- derivations;
- solving strategies;
- pedagogic explanations.

An equation is a statement, not an expression. For example, `x + 1 = 3` has
solution-set semantics; it is not merely a boolean-valued expression.

## Mathematical Objects

### Expression

An expression denotes a mathematical value.

Initial expression forms:

- exact integer;
- exact rational;
- symbol;
- sum;
- product;
- power with integer exponent where needed by the first slice.

### Statement

A statement denotes a mathematical proposition.

Initial statement forms:

- equality statement;
- future: inequality statement;
- future: compound proposition.

### Context

A context records the domain and assumptions under which a mathematical object
is interpreted.

For the first slice:

- numbers are exact rationals;
- variables range over rational or real scalar values, to be selected by the
  operation;
- division by an expression requires a nonzero assumption or proof.

## Evaluation

Evaluation converts a mathematical object into a value under an explicit
assignment and context.

- Expression evaluation returns an exact value when all required symbols are
  assigned.
- Statement evaluation returns a truth value when both sides can be evaluated.
- Partial evaluation may return a simplified object with unresolved symbols.
- Evaluation must never silently convert exact values to JavaScript `number`.

Example:

```text
evaluate expression x + 2 with x = 3 -> 5
evaluate statement x + 2 = 5 with x = 3 -> true
evaluate statement x + 2 = 6 with x = 3 -> false
```

Evaluation is pointwise: it answers whether a statement is true for a specific
assignment. It does not prove that two statements have the same solution set.
Solution-set comparison is a separate symbolic operation.

## Simplification

Simplification rewrites a mathematical object into a simpler equivalent object.
It must preserve enough structure to support pedagogy.

The system should distinguish:

- display-preserving structure used for learner-facing steps;
- canonical forms used for comparison and verification.

Simplification may produce a derivation. A silent canonicalization routine may
exist internally, but it must not erase pedagogically meaningful structure in
public derivations.

## Equality and Equivalence

The system must not use one vague equality operation for every purpose.

Required comparison modes:

- syntactic equality: same represented tree;
- normalized expression equality: same expression after canonicalization under
  context;
- statement equivalence: statements have the same truth conditions under
  context;
- equation solution-set equality: equations have the same solutions under
  context.

For learner answer checking, the default comparison for equations is
solution-set equality, not textual equality.

For saved-answer checking, equation comparison should generally use
solution-set equality. For checking a learner's entered expression, comparison
should generally use normalized expression equality. The caller must choose the
comparison mode explicitly in public APIs.

Example:

```text
x + 1 = 3
x = 2
```

These are not syntactically equal, but they have the same solution set over the
intended domain.

## Transformations

A transformation is a structured mathematical step.

Each transformation result must record:

- stable transformation identifier;
- input object;
- output object;
- focus location;
- matched operands;
- assumptions consumed;
- semantic relation preserved;
- structured justification;
- verifier data.

Initial semantic relations:

- `EqualExpression`;
- `EquivalentStatement`;
- `SameSolutionSet`;
- `Implies`;
- `Approximates`.

The first vertical slice should primarily use `EqualExpression`,
`EquivalentStatement`, and `SameSolutionSet`.

## Derivations

A derivation is replayable structured data. It is not a list of strings.

Each derivation contains:

- initial object;
- final object;
- ordered transformation steps;
- context;
- deterministic rendering data where needed;
- versioned serialization metadata.

Explanations are generated from derivations, not stored as the authoritative
mathematical evidence.

## Learner-Step Validation

Learner-step validation answers whether a submitted mathematical object is a
valid next state from a previous object under a context.

Validation should report:

- whether the step is valid;
- the strongest relation that was preserved;
- likely transformation if one is recognized;
- consumed assumptions;
- whether the step is expected, valid but skipped, or invalid;
- structured error details when invalid.

Validation must not require the learner to follow the engine's preferred next
step when another mathematically valid step is acceptable.

## LaTeX Profile

The project supports a versioned Socrates Mathematical LaTeX Profile, not
arbitrary LaTeX.

Initial profile:

- integers;
- rational literals using `\frac{a}{b}`;
- symbols;
- addition and subtraction;
- multiplication by juxtaposition or `\cdot`;
- parentheses;
- equality;
- powers only where needed by the first slice.

Parsing pipeline:

```text
LaTeX
-> tokens
-> surface syntax tree with source spans
-> semantic resolution
-> mathematical object
```

Rendering must produce deterministic KaTeX-compatible LaTeX.

Semantic round trip:

```text
parse(render(object)) is semantically equivalent to object
```

## TypeScript API Shape

The public package should expose coarse operations, not generated Wasm types.

Sketch:

```ts
const engine = await createMathEngine();

const statement = engine.parseStatement(String.raw`3(x-2)+4 = 2x+9`);

const simplified = engine.simplify(statement);

const comparison = engine.compareStatements(statement, simplified.statement, {
  mode: "solution-set",
});

const evaluation = engine.evaluateStatement(statement, {
  x: { kind: "integer", value: "11" },
});

const solution = engine.solve(statement, {
  variable: "x",
  explanation: "instructional",
});
```

Exact values cross the TypeScript boundary as tagged DTOs or strings, never as
implicit JavaScript numbers.

## First Implementation Target

The first implementation should prove one complete browser workflow:

1. parse a linear equation from LaTeX;
2. simplify both sides with derivation steps;
3. solve for one variable over exact rationals;
4. render every step as KaTeX-compatible LaTeX;
5. validate learner-submitted intermediate equations;
6. compare a learner final answer to the saved answer by solution-set equality;
7. evaluate expressions and statements under explicit assignments;
8. serialize and deserialize all public result DTOs.
