# Symbolic Math

Symbolic Math is the verified mathematical reasoning platform for the Socrates Academy ecosystem.

Its purpose is to represent mathematical work in a form that can be:

* interpreted precisely;
* computed exactly;
* transformed deliberately;
* checked independently;
* explained pedagogically;
* replayed and serialized;
* compared with learner work.

Symbolic Math is not only a computer algebra system, theorem prover, or tutoring engine. It is the mathematical foundation that connects all three.

## Vision

The long-term goal is to support mathematical teaching and symbolic reasoning from arithmetic and school algebra through calculus, linear algebra, abstract algebra, complex analysis, geometry, topology, category theory, and other advanced domains.

Different domains require different algorithms and representations. The project therefore does not attempt to force all mathematics into one expression type or one universal simplifier. Instead, it provides:

* a typed mathematical language;
* explicit mathematical contexts and assumptions;
* an extensible theory and domain-package system;
* specialized symbolic computation engines;
* checkable proofs and certificates;
* a separate pedagogical reasoning layer.

The system should eventually be able to answer not only:

> What is the result?

but also:

> What mathematical object does this notation denote?
> Under which assumptions is the result valid?
> Which relation connects the input and output?
> How can the result be checked independently?
> How should this reasoning be presented to this learner?

## Core Model

The fundamental unit of mathematical work is a **judgment in context**:

```text
context ⊢ left_object relation right_object
```

For example:

```text
x : Real, x > 0 ⊢ sqrt(x²) = x
```

A mathematical context may contain:

* imported theories;
* symbol and variable declarations;
* types and mathematical structures;
* hypotheses and assumptions;
* definitions;
* notation;
* coercions and canonical maps;
* branch and domain conventions.

This makes the meaning and validity of every operation explicit.

## Architectural Principles

### Typed and contextual mathematics

Mathematical objects are interpreted relative to declared types, structures, domains, and assumptions. Notation such as `+`, `|x|`, `(a, b)`, or `f⁻¹` is resolved by an elaboration phase rather than assigned one fixed meaning by the parser.

### Proof-producing computation

Symbolic algorithms do not merely return answers. They produce structured evidence that records:

* what changed;
* where it changed;
* why the change was valid;
* which assumptions were used;
* which semantic relation was established;
* how the result can be checked.

### Small trusted core

Parsers, simplifiers, solvers, search procedures, and pedagogical planners are treated as proof producers, not unquestioned authorities. Their conclusions are accepted only when their proof, derivation, or domain-specific certificate can be checked by a smaller verification layer.

### Explicit relations

There is no single vague notion of equality. Depending on the operation, the library may reason about:

* syntactic equality;
* alpha-equivalence;
* equality in a mathematical structure;
* logical equivalence;
* implication;
* equality of solution sets;
* extensional equality on a domain;
* isomorphism or equivalence;
* approximation within an error bound.

Public APIs must state which relation is being requested.

### No universal simplifier

Canonicalization, normalization, simplification, transformation, evaluation, approximation, and pedagogical derivation are distinct operations.

A useful form depends on the goal. An expanded polynomial may be best for coefficient comparison, while a factored form may be best for solving or teaching. Simplification therefore operates under an explicit policy rather than pretending that every expression has one universally simplest form.

### Exactness by default

Integers, rationals, algebraic values, symbolic constants, and other exact objects must remain exact across Rust, WebAssembly, serialization, and TypeScript boundaries.

Approximate computation is explicit and should report its precision, error bound, or enclosure whenever possible.

### Pedagogy is derived from mathematics

A derivation is authoritative mathematical data, not a list of explanation strings. Learner-facing prose, hints, highlighting, diagrams, and step granularity are generated from verified derivations.

This allows the same mathematical evidence to support:

* concise expert solutions;
* detailed instructional solutions;
* learner-step validation;
* hints;
* misconception diagnosis;
* localized explanations;
* accessible spoken or structured rendering.

### Extensible mathematical theories

Advanced domains are added as theory packages rather than by continually enlarging a monolithic expression enum.

A theory package may define:

* symbols and semantic identities;
* mathematical structures and types;
* notation;
* definitions and theorems;
* rewrite rules;
* canonical forms;
* decision procedures and solvers;
* certificate checkers;
* rendering rules;
* pedagogical concepts and prerequisites.

## System Overview

```text
Learner or author input
        ↓
Concrete syntax with source spans
        ↓
Notation resolution and elaboration
        ↓
Typed semantic mathematical objects
        ↓
Domain-specific computation and proof search
        ↓
Proof or certificate verification
        ↓
Verified result and remaining obligations
        ↓
Pedagogical planning and explanation
        ↓
LaTeX, structured UI, speech, or serialized output
```

The system preserves both the learner's original representation and the elaborated semantic object. The original form supports precise feedback and error highlighting; the semantic form supports computation and verification.

## Primary Capabilities

Symbolic Math is designed to provide:

* parsing of a versioned mathematical LaTeX profile;
* source-preserving concrete syntax trees;
* elaboration into typed semantic terms;
* explicit contexts, assumptions, and side conditions;
* exact evaluation and partial evaluation;
* certified numerical approximation;
* goal-directed normalization and transformation;
* comparison under explicitly selected mathematical relations;
* solving with candidate verification and completeness information;
* replayable proof and derivation graphs;
* validation of learner-entered intermediate steps;
* recognition of valid alternative strategies and skipped steps;
* counterexamples and minimal repairs for invalid work;
* deterministic mathematical rendering;
* versioned public serialization;
* a stable TypeScript API backed by Rust and WebAssembly.

Results must distinguish at least:

```text
Proven
Disproven
Conditional
Unknown
Undefined
```

Failure to prove a statement must never be reported as proof that it is false.

## First Vertical Slice

The first complete workflow is single-variable linear equations over exact rational numbers.

Example:

```latex
3(x - 2) + 4 = 2x + 9
```

The browser-facing library should be able to:

1. parse the input while preserving source spans;
2. elaborate it into a typed equality statement;
3. simplify it according to an instructional policy;
4. solve for `x` over the rationals;
5. produce a deterministic, replayable derivation;
6. verify every transformation independently;
7. render every state as KaTeX-compatible LaTeX;
8. validate learner-submitted intermediate equations;
9. accept mathematically valid alternative paths;
10. compare final answers by the requested solution-set relation;
11. evaluate expressions and statements under explicit assignments;
12. serialize and deserialize all public results without precision loss.

For the example above, the verified result is:

```latex
x = 11
```

The first slice is intentionally narrow, but it must exercise the same architectural boundaries required by later domains: syntax, elaboration, context, exact computation, proof production, verification, pedagogy, rendering, and serialization.

## Public Boundary

Applications consume `@socrates/math` through a handwritten, stable TypeScript facade.

```text
Application
    ↓
@socrates/math
    ↓
stable TypeScript facade and versioned DTOs
    ↓
internal WebAssembly adapter
    ↓
Rust application services
    ↓
mathematical engines and verification kernel
```

Generated WebAssembly binding classes are internal implementation details and must never become public application types.

## Long-Term Domain Growth

The platform should grow through coherent mathematical layers:

```text
logic and foundational terms
    ↓
natural numbers, integers, and rationals
    ↓
elementary and polynomial algebra
    ↓
functions, equations, and inequalities
    ↓
linear algebra and calculus
    ↓
abstract algebra and analysis
    ↓
geometry and topology
    ↓
category theory and other advanced domains
```

Each layer should reuse the common semantic language, contexts, judgments, proof infrastructure, and public protocols while providing its own appropriate algorithms.

## Non-Goals

Symbolic Math is not intended to be:

* an arbitrary LaTeX interpreter;
* one untyped expression tree containing every mathematical concept;
* a black-box solver that returns unverifiable answers;
* a system with one Boolean equality test for all mathematics;
* a system with one universal definition of “simplest”;
* a tutoring layer whose prose is treated as mathematical proof;
* a public API coupled to generated WebAssembly types;
* a replacement for specialized mathematical algorithms or external formal systems.

Interoperability with established computer algebra systems, theorem provers, SMT solvers, and mathematical interchange formats is expected where it improves coverage or trust.

## Project Documents

* [SPEC.md](./SPEC.md) defines the architecture, semantic contracts, trust boundaries, public protocols, and initial implementation requirements.
* [features.md](./features.md) defines the behavior-driven feature catalog and executable acceptance criteria.

## Guiding Standard

A feature is not complete merely because it returns the expected answer.

It is complete when the system can state:

* what the input means;
* under which context it is interpreted;
* what result was established;
* which relation was proved;
* which assumptions and obligations apply;
* how the result can be verified;
* how it can be presented without compromising the mathematics.
