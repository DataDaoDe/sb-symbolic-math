# Symbolic Math Specification

## 1. Status

This document defines the architectural and semantic contract for Symbolic
Math.

It is normative for:

- the Rust domain model;
- the verification boundary;
- theory and rule packages;
- symbolic operations;
- learner-step validation;
- public serialization;
- the TypeScript and WebAssembly boundary;
- the first implementation slice.

The behavior-driven acceptance catalog is maintained separately in
[`features.md`](./features.md).

## 2. Purpose

Symbolic Math is the verified mathematical reasoning platform for Socrates
Academy.

It combines:

- symbolic computation;
- explicit mathematical semantics;
- proof and certificate verification;
- pedagogical derivation planning;
- learner-work validation.

The system must support exact, inspectable mathematical work rather than
returning only opaque final answers.

Its long-term scope includes mathematics from arithmetic and school algebra
through calculus, linear algebra, abstract algebra, complex analysis, geometry,
topology, category theory, and other advanced domains.

## 3. Normative Language

The terms **MUST**, **MUST NOT**, **SHOULD**, **SHOULD NOT**, and **MAY** are
normative.

- **MUST** indicates a required invariant.
- **SHOULD** indicates a strong design requirement that may be violated only
  with a documented reason.
- **MAY** indicates an optional capability.

## 4. Product Goals

Symbolic Math MUST be able to:

1. preserve the distinction between notation and mathematical meaning;
2. elaborate notation into typed mathematical objects;
3. interpret objects within explicit contexts;
4. compute exact results where possible;
5. state assumptions, side conditions, and unresolved obligations;
6. distinguish different mathematical relations;
7. produce replayable derivations and checkable evidence;
8. validate learner-entered work without requiring one preferred method;
9. generate pedagogy from mathematical evidence;
10. grow through domain packages rather than a monolithic expression type;
11. expose a stable browser-facing API independent of generated Wasm bindings;
12. serialize public results without loss of mathematical meaning or precision.

## 5. Non-Goals

Symbolic Math is not intended to be:

- an arbitrary TeX interpreter;
- one universal untyped expression tree;
- one universal simplification algorithm;
- a black-box solver whose output is trusted without verification;
- a system in which failure to prove means false;
- a system in which every transformation preserves the same relation;
- a prose explanation engine whose strings serve as mathematical evidence;
- a public API coupled to generated Rust or `wasm-bindgen` classes;
- an immediate reimplementation of every existing computer algebra system or
  theorem prover.

Interoperation with external symbolic systems, proof assistants, SMT solvers,
and interchange formats is permitted and encouraged when their output can be
checked or clearly marked with an appropriate trust status.

## 6. System Architecture

The intended processing flow is:

```text
surface input
    ↓
tokenization and concrete syntax
    ↓
notation resolution and elaboration
    ↓
typed semantic objects in context
    ↓
domain-specific computation, rewriting, or search
    ↓
proof or certificate production
    ↓
verification
    ↓
verified result and remaining obligations
    ↓
pedagogical planning and explanation
    ↓
rendering and public serialization
```

The browser-facing dependency direction is:

```text
Application
    ↓
@socrates/math
    ↓
handwritten TypeScript facade
    ↓
versioned public DTOs
    ↓
internal WebAssembly adapter
    ↓
Rust application services
    ↓
domain engines and verification kernel
```

Dependencies MUST point inward toward semantic and verification layers.

The verification kernel MUST NOT depend on:

- WebAssembly;
- TypeScript;
- UI code;
- rendering;
- pedagogical explanation generation;
- high-level solver orchestration.

## 7. Trust Model

### 7.1 Trusted components

The trusted computing base SHOULD remain as small as practical.

It consists of:

- core term and context well-formedness checking;
- rule instantiation checking;
- proof-node checking;
- domain-specific certificate checkers;
- exact primitive arithmetic used by those checkers;
- theory manifests and explicitly declared axioms.

### 7.2 Untrusted proof producers

The following components MUST be treated as proof or candidate producers:

- parsers after syntax construction;
- elaboration heuristics;
- simplifiers;
- rewrite search;
- e-graphs;
- solvers;
- external computer algebra systems;
- external SMT solvers;
- strategy recognizers;
- learner-step classifiers;
- pedagogical planners.

A defect in one of these components MUST NOT allow an invalid mathematical
conclusion to be returned as verified.

### 7.3 Public trust status

Every public mathematical result MUST indicate one of these states:

```text
Verified
Conditional
Unverified
```

`Unverified` results MAY be exposed only by APIs that explicitly request
heuristic or candidate output.

The normal teaching and answer-checking APIs MUST return only `Verified` or
`Conditional` mathematical conclusions.

## 8. Fundamental Semantic Model

The fundamental semantic unit is a judgment in a context:

```text
Γ ⊢ A R B
```

where:

- `Γ` is a mathematical context;
- `A` and `B` are typed semantic objects;
- `R` is an explicit mathematical relation;
- the judgment may carry proof evidence and obligations.

Unary judgments are also permitted:

```text
Γ ⊢ P
Γ ⊢ A : T
Γ ⊢ Defined(A)
```

Examples:

```text
x : Real, x > 0 ⊢ sqrt(x²) = x
x : Rational ⊢ 3(x - 2) + 4 = 3x - 2
x : Rational ⊢ Solutions(x + 1 = 3, x) = {2}
```

No operation may rely on an undocumented ambient domain or hidden global
assumption.

## 9. Stable Semantic Identity

Semantic identity MUST NOT depend on display strings.

The system MUST use stable identifiers for:

```text
TheoryId
SymbolId
RuleId
RelationId
TypeId
ConceptId
NotationProfileId
CertificateFormatId
```

Examples:

```text
core.integer.add
core.rational.literal
algebra.ring.mul
logic.equality
equation.same_solution_set
category_theory.functor.map
```

Display names such as `+`, `map`, or `0` are notation attached to semantic
identities.

Stable identifiers MUST be versioned through their owning theory package.

## 10. Syntax Layer

### 10.1 Versioned input profile

The project supports a versioned Socrates Mathematical LaTeX Profile.

It does not accept arbitrary LaTeX.

The parser MUST receive:

- source text;
- notation profile identifier and version;
- optional locale;
- optional syntax mode;
- optional recovery policy.

### 10.2 Concrete syntax tree

Parsing MUST produce a concrete syntax representation before semantic
elaboration.

The concrete representation MUST preserve:

- token boundaries;
- source spans;
- explicit grouping;
- implicit multiplication sites;
- commands and delimiters;
- recoverable malformed regions;
- learner-entered structure where possible.

The system SHOULD preserve enough source information to highlight the exact
part of an expression involved in a transformation or error.

### 10.3 Parse outcomes

Parsing MUST distinguish:

```text
Parsed
ParsedWithRecovery
Incomplete
Rejected
```

A parse error MUST include:

- a stable diagnostic code;
- the source span where available;
- expected syntax where useful;
- the unsupported or malformed construct;
- recovery information if recovery occurred.

### 10.4 Syntax is not semantics

The parser MUST NOT decide the final meaning of overloaded notation.

For example, the parser may identify a delimited pair or an infix operator, but
the elaborator decides whether it denotes:

- an interval;
- an ordered pair;
- a greatest common divisor;
- scalar addition;
- vector addition;
- a coproduct;
- another imported meaning.

## 11. Elaboration

Elaboration converts concrete syntax into typed semantic objects.

It is responsible for:

- name and namespace resolution;
- notation resolution;
- operator overloading;
- implicit arguments;
- binder construction;
- type inference;
- coercion insertion;
- domain resolution;
- placeholder and hole tracking;
- generation of elaboration obligations.

Elaboration MUST receive an explicit context and theory environment.

### 11.1 Elaboration outcomes

Elaboration MUST distinguish:

```text
Elaborated
ElaboratedWithObligations
Ambiguous
Incomplete
Rejected
```

Required diagnostic categories include:

```text
UnknownSymbol
AmbiguousNotation
CannotInferType
TypeMismatch
MissingBinder
UnresolvedImplicitArgument
NoCoercion
AmbiguousCoercion
TheoryNotImported
UnsupportedSemanticForm
```

Elaboration heuristics are outside the trusted kernel. The elaborated result
MUST pass semantic well-formedness checking before it is accepted.

## 12. Core Semantic Terms

The semantic core MUST represent typed mathematical terms independently of
surface notation.

The term language MUST support at least:

- theory-qualified constants;
- local variables;
- exact literals;
- function application;
- typed binders;
- function types;
- type annotations;
- definitions and local `let` bindings;
- tuples or structured arguments;
- metavariables and holes;
- propositions or statement terms.

The core MUST NOT require one enum variant for every mathematical operation.
Operations such as addition, composition, continuity, or functor mapping SHOULD
normally be represented as applications of theory-qualified symbols.

Illustrative form:

```text
Apply(
  Symbol("algebra.ring.add"),
  [x, Literal(Integer("1"))]
)
```

The initial implementation MAY use a non-dependent typed term language, but the
serialized model and identifiers MUST NOT preclude later support for dependent
binders and richer type theory.

## 13. Semantic Object Graph

Semantic objects SHOULD use an immutable directed acyclic graph with structural
sharing.

Each object SHOULD have:

- a stable session-local identifier;
- a semantic hash;
- a type reference;
- its term node;
- optional origin mappings to source spans;
- optional display annotations excluded from semantic equality.

The semantic hash MUST be based only on mathematically relevant structure and
versioned semantic identities.

Source spans, UI selections, and explanation strings MUST NOT affect semantic
hashes.

## 14. Contexts

A context defines the environment in which semantic objects are interpreted.

A context MUST be immutable or persistently versioned.

It may contain:

```text
Context
├── imported theory packages
├── symbol declarations
├── local variable declarations
├── hypotheses
├── local definitions
├── notation environment
├── structure and instance environment
├── coercion graph
├── domain and branch conventions
└── provenance
```

### 14.1 Assumptions

Assumptions MUST be:

- typed;
- scoped;
- independently addressable;
- recorded in proof dependencies;
- serializable;
- accompanied by provenance.

Assumptions MUST NOT be represented only as free-form strings.

### 14.2 Context consistency

Where practical, the system SHOULD detect inconsistent contexts.

Operations performed under a known inconsistent context MUST report that
condition. They MUST NOT present arbitrary conclusions as pedagogically useful
proofs merely because classical explosion would permit them.

### 14.3 Context extension

Adding a declaration or hypothesis creates a new context.

Context extension MUST preserve a reference to the parent context or an
equivalent persistent representation suitable for derivation replay.

## 15. Mathematical Structures and Coercions

The system MUST represent mathematical structures explicitly.

Examples include:

- semigroups;
- groups;
- rings;
- fields;
- modules;
- vector spaces;
- algebras;
- metric spaces;
- topological spaces;
- categories.

Elements MUST be interpreted relative to an appropriate parent, carrier, or
type.

### 15.1 Canonical maps

Coercions MUST correspond to registered mathematical maps.

Examples:

```text
Integer → Rational
Rational → Real
Real → Complex
```

Every inserted coercion MUST be recoverable from provenance.

### 15.2 Coercion rules

- Ambiguous coercion paths MUST be rejected.
- Lossy conversions MUST require explicit operations.
- Coercion insertion MUST NOT silently change the meaning of an object.
- Coercion declarations MUST belong to versioned theory packages.
- The verifier MUST check inserted coercions for type correctness.

## 16. Theory Packages

Mathematical capabilities are added through theory packages.

A theory package may define:

```text
TheoryPackage
├── package identity and version
├── dependencies
├── semantic symbols
├── types and structures
├── notation declarations
├── definitions
├── axioms
├── theorems
├── relations
├── rewrite rules
├── normalization procedures
├── solvers and decision procedures
├── certificate formats and checkers
├── renderers
└── pedagogical metadata
```

### 16.1 Theory manifests

Each theory package MUST declare:

- a stable package identifier;
- a semantic version;
- dependency versions;
- exported symbol identifiers;
- exported rule identifiers;
- declared axioms;
- certificate checker versions;
- serialization compatibility information.

### 16.2 Axioms and trust

Axioms MUST be explicit and queryable.

A proof result MUST be able to report which axioms or foundational theory
packages it depends on.

### 16.3 Initial loading model

The first implementation SHOULD compile theory packages statically into the
Rust workspace.

Dynamic third-party loading is deferred until package signing, sandboxing,
compatibility, and trust policies are specified.

## 17. Relations

Relations are first-class semantic identities.

The core relation registry MUST support metadata describing:

- arity;
- expected operand types;
- reflexivity;
- symmetry;
- transitivity;
- congruence behavior;
- verification procedure;
- display form.

Initial relations include:

```text
syntax.same_tree
logic.equal
logic.equivalent
logic.implies
logic.implied_by
equation.same_solution_set
function.extensionally_equal_on
numeric.approximates_within
```

Future domain packages may define:

```text
algebra.isomorphic
linear_algebra.similar
topology.homeomorphic
topology.homotopy_equivalent
category_theory.equivalent
diagram.commutes
```

Public comparison APIs MUST require an explicit relation or a domain-specific
operation whose relation is unambiguous.

## 18. Mathematical Outcomes

A semantic query MUST return one of the following outcome classes:

```text
Proven
Disproven
Conditional
Unknown
Undefined
```

### 18.1 Proven

`Proven` contains:

- the established judgment;
- verified evidence;
- assumptions used;
- theory and rule dependencies.

### 18.2 Disproven

`Disproven` contains verified refutation evidence, such as:

- a counterexample;
- a contradiction proof;
- a decision-procedure certificate.

### 18.3 Conditional

`Conditional` contains:

- the judgment established under conditions;
- unresolved or required conditions;
- evidence valid under those conditions.

### 18.4 Unknown

`Unknown` means the system did not establish or refute the query.

It MUST include a reason such as:

```text
UnsupportedDomain
InsufficientAssumptions
ResourceLimit
IncompleteProcedure
NoApplicableMethod
ExternalBackendUnavailable
```

`Unknown` MUST NOT be converted to `false`.

### 18.5 Undefined

`Undefined` means the requested mathematical interpretation or operation is not
defined in the supplied context.

Examples include:

- division by zero;
- evaluating outside a function's domain;
- applying an operation to an object of the wrong structure;
- selecting an unspecified branch where no unique value exists.

Operational software failures are not mathematical `Undefined` results. They
must use a separate error channel.

## 19. Proofs, Derivations, and Certificates

### 19.1 Proof node

A proof node MUST contain:

```text
ProofNode
├── conclusion
├── rule identifier
├── premise references
├── instantiated parameters
├── substitutions
├── discharged side conditions
├── remaining obligations
├── occurrence or focus information
└── provenance
```

### 19.2 Proof graph

Proofs and derivations SHOULD be represented as DAGs rather than duplicated
lists.

A proof graph MUST be:

- deterministic under a deterministic operation policy;
- replayable;
- serializable;
- independently checkable;
- capable of sharing repeated subproofs.

### 19.3 Derivation

A derivation is an ordered pedagogically or operationally meaningful path
through verified proof nodes.

A derivation MUST contain:

- initial state;
- final state;
- ordered steps;
- context;
- relation established by each step;
- proof references;
- version metadata.

A derivation is authoritative mathematical data.

Explanation strings are not authoritative evidence.

### 19.4 Certificates

Specialized algorithms MAY emit compact certificates rather than expanded proof
terms.

Examples include:

- polynomial identity certificates;
- row-operation certificates;
- Gröbner basis certificates;
- primality certificates;
- interval enclosures;
- SAT or SMT proofs.

Every certificate format MUST have:

- a stable identifier;
- a version;
- a checker;
- documented soundness assumptions;
- serialization rules.

A certificate is accepted only after its checker succeeds.

## 20. Occurrences and Focus Locations

Transformations MUST identify where a change occurred.

A focus location SHOULD be represented by a stable occurrence path into the
semantic object, supplemented by source-span mappings where available.

A transformation MUST distinguish:

- the semantic occurrence changed;
- the source region associated with it;
- copied or shared occurrences;
- binder scope where relevant.

Source spans alone MUST NOT serve as semantic identity.

## 21. Canonicalization, Normalization, Simplification, and Transformation

These operations are distinct.

### 21.1 Canonicalization

Canonicalization produces deterministic internal structure.

It SHOULD be:

- cheap;
- deterministic;
- semantics-preserving;
- minimally assumption-dependent;
- unsuitable as a substitute for an instructional derivation.

Examples:

- canonical rational sign placement;
- normalized integer literals;
- deterministic ordering where the theory permits it;
- alpha-normalization of binders.

### 21.2 Normalization

Normalization computes a theory-specific normal form.

Every normalizer MUST state:

- the theory in which it operates;
- the relation preserved;
- its completeness domain;
- required assumptions;
- resource limitations.

Examples:

```text
PolynomialNormalForm over Rational[x]
ReducedRationalFunction over Rational(x)
RowReducedEchelonForm over a field
```

### 21.3 Simplification

Simplification attempts to improve an object according to an explicit policy.

A simplification policy MUST specify or inherit:

- goal;
- allowed theories and rules;
- cost model;
- step granularity;
- structure-preservation preferences;
- assumption policy;
- resource budget;
- determinism requirement.

Example policies:

```text
ArithmeticInstructional
LinearEquationInstructional
PolynomialExpanded
PolynomialFactored
MinimizeOperationCount
PrepareForDifferentiation
MatchLearnerStrategy
```

There is no global, context-free definition of the simplest form.

### 21.4 Transformation

A transformation requests a particular change of representation or structure.

Examples:

```text
Expand
Factor
Collect
CompleteSquare
Rationalize
Substitute
ChangeCoordinates
RewriteUsing(rule)
```

A transformation MUST return the relation it established and its verified
evidence.

## 22. Rewriting

Rewrite rules MUST contain:

- a stable rule identifier;
- a typed left pattern;
- a typed right pattern;
- side conditions;
- the relation established;
- theory ownership;
- verification behavior;
- optional pedagogical metadata.

Conditional rules MUST NOT be applied as unconditional rules.

For example:

```text
x / x → 1
```

requires an obligation proving `x ≠ 0`.

### 22.1 Rewrite search

Rewrite search and equality saturation MAY be used to discover candidate paths.

Extracted paths MUST be replayed through the verification kernel.

An e-graph or rewrite database MUST NOT itself be treated as proof.

## 23. Evaluation

Evaluation computes the interpretation of an object under an explicit
assignment and context.

### 23.1 Exact evaluation

Exact evaluation MUST preserve exact values.

Exact values MUST NOT silently cross the Wasm boundary as JavaScript `number`.

### 23.2 Expression evaluation

Expression evaluation may return:

```text
ExactValue
ResidualTerm
ConditionalValue
Undefined
Unknown
```

A residual result MUST report:

- the residual semantic term;
- substitutions performed;
- unresolved symbols;
- remaining conditions.

### 23.3 Statement evaluation

Statement evaluation under a complete assignment may return:

```text
True
False
Undefined
Unknown
```

Pointwise evaluation is not a proof of general equivalence.

### 23.4 Assignment validation

Assignments MUST be type checked against the context.

Unknown variables, incompatible values, and ambiguous coercions MUST be
reported explicitly.

## 24. Approximation

Approximation is separate from exact evaluation.

Approximation results SHOULD contain:

- approximate value;
- precision;
- rounding mode;
- error bound or enclosure;
- method identifier;
- domain conditions;
- verification status.

An approximation MUST NOT be serialized as though it were an exact value.

## 25. Comparison

The generic comparison operation has the conceptual form:

```text
compare(left, right, relation, context, options)
```

A comparison result MUST state:

- relation requested;
- outcome class;
- assumptions used;
- evidence or refutation;
- unresolved conditions;
- method used;
- completeness status where relevant.

Comparison modes MUST NOT be inferred only from the syntactic shape of the
inputs.

Examples:

- expression comparison may request ring equality;
- equation comparison may request equality of solution sets;
- function comparison may request extensional equality on a domain;
- category comparison may request equivalence rather than equality.

## 26. Solving

Solving operates on predicates or statements with respect to specified unknowns.

A solve request MUST include:

- statement or predicate;
- variables to solve for;
- ambient domains;
- context;
- strategy or policy;
- resource budget.

A solve result MUST include:

- candidate or verified solution representation;
- accepted solutions;
- excluded candidates;
- side conditions;
- derivation or certificate;
- completeness status.

Required completeness statuses:

```text
Complete
CompleteUnderConditions
SoundButPossiblyIncomplete
HeuristicCandidates
Unknown
```

### 26.1 Relation-changing solve steps

A solver MUST NOT assume every step preserves equivalence.

Each step may establish:

```text
Equivalent
Implies
ImpliedBy
Equisatisfiable
SameSolutionsFor(variable_set)
```

When a step can introduce extraneous candidates, the solver MUST validate final
candidates against the original statement.

When a step can discard candidates, the solver MUST record the exclusion
condition and justify completeness before returning `Complete`.

## 27. Learner-Step Validation

Learner-step validation compares a previous mathematical state with learner
input in a supplied context.

The validator MUST accept mathematically valid alternatives even when they do
not match the engine's preferred next step.

Required statuses:

```text
Valid
ValidWithSkippedSteps
ValidUnderAdditionalCondition
InvalidWithCounterexample
InvalidSideCondition
Unverified
Ambiguous
Malformed
```

A validation result SHOULD include:

- strongest established relation;
- recognized strategy or rule;
- changed occurrence;
- assumptions consumed;
- skipped transformations;
- proof evidence;
- violated side condition;
- counterexample when available;
- minimal repair when available;
- likely misconception identifiers;
- confidence of strategy recognition.

`Unverified` means the engine could not establish validity or invalidity. It
MUST NOT be presented as mathematically invalid.

## 28. Pedagogical Layer

The pedagogical layer consumes verified mathematics.

It MUST NOT alter the mathematical validity of a result.

It may select:

- step granularity;
- strategy;
- explanation depth;
- notation style;
- examples and counterexamples;
- hint amount;
- prerequisite concepts;
- vocabulary and localization;
- visual emphasis.

### 28.1 Rule and concept separation

Formal rule identifiers and pedagogical concept identifiers MUST be separate.

One formal rule may support multiple explanations, and one pedagogical concept
may be realized by several formal rules.

### 28.2 Explanation generation

Explanations MUST be generated from:

- verified derivation data;
- rule metadata;
- context;
- learner model;
- localization resources.

Stored prose MUST NOT be the sole representation of a derivation.

## 29. Rendering

Rendering is a presentation operation over syntax or semantic objects.

Required initial target:

- deterministic KaTeX-compatible LaTeX.

Future targets may include:

- MathML-like structured output;
- spoken mathematics;
- braille-oriented structure;
- debug semantic notation;
- diagram descriptions.

Rendering modes SHOULD include:

```text
OriginalLike
Canonical
Instructional
Compact
AccessibleSpeech
DebugSemantic
```

### 29.1 Round-trip contract

The standard semantic round-trip requirement is:

```text
elaborate(parse(render(object))) ≈ object
```

where the relation `≈` is explicitly defined by the rendering mode and theory.

The system MUST NOT claim concrete-syntax identity when it only guarantees
semantic equivalence.

## 30. Public Serialization

Public DTOs MUST be versioned.

A serialized verified result MUST contain enough information to preserve its
meaning and replayability, including:

- schema version;
- notation profile and version;
- context;
- semantic object graph;
- exact numeric representation;
- theory package identifiers and versions;
- rule identifiers;
- relation identifiers;
- proof or certificate data;
- assumptions and obligations;
- origin mappings where public;
- kernel and checker compatibility metadata.

### 30.1 Exact values

Exact values MUST use tagged representations.

Examples:

```json
{ "kind": "integer", "value": "-42" }
{ "kind": "rational", "numerator": "5", "denominator": "6" }
```

Large integers and rationals MUST NOT be encoded as JSON numbers.

### 30.2 Compatibility

Deserialization MUST reject or explicitly migrate incompatible:

- schema versions;
- semantic identities;
- rule versions;
- theory versions;
- certificate formats.

A derivation MUST NOT silently acquire new meaning after a package upgrade.

## 31. TypeScript API

The public package is `@socrates/math`.

The public API MUST expose handwritten facade objects and versioned DTOs.

Generated Wasm classes MUST remain private.

Illustrative API:

```ts
const engine = await createMathEngine({
  notationProfile: {
    id: "socrates-latex",
    version: "0.1",
  },
  theories: [
    "core.logic@0.1",
    "core.rational@0.1",
    "algebra.linear-equations@0.1",
  ],
});

const parsed = engine.parse(
  String.raw`3(x-2)+4 = 2x+9`,
);

const elaborated = engine.elaborate(parsed.syntax, {
  expectedKind: "statement",
  declarations: {
    x: "Rational",
  },
});

const solution = engine.solve(elaborated.object, {
  variables: ["x"],
  domains: { x: "Rational" },
  policy: "LinearEquationInstructional",
});

const validation = engine.validateStep({
  previous: elaborated.object,
  submittedLatex: String.raw`3x-2=2x+9`,
  context: elaborated.context,
});

const rendered = engine.render(solution.finalObject, {
  format: "latex",
  mode: "instructional",
});
```

### 31.1 API result discipline

Expected mathematical outcomes MUST be returned as tagged values rather than
exceptions.

Exceptions or rejected promises are reserved for operational failures such as:

- corrupted internal state;
- unavailable Wasm module;
- invalid serialized protocol;
- cancelled operation;
- internal resource failure.

## 32. Resource Budgets

All potentially expensive operations MUST accept or inherit a resource budget.

A budget may constrain:

- wall-clock duration;
- rewrite count;
- proof-search depth;
- e-graph nodes;
- term size;
- memory;
- external calls;
- numerical precision.

Budget exhaustion MUST return `Unknown` with reason `ResourceLimit`, not an
incorrect mathematical result.

Operations SHOULD support cancellation.

## 33. Determinism

Given the same:

- semantic inputs;
- context;
- theory package versions;
- operation policy;
- resource budget;
- deterministic mode;

the system MUST produce the same canonical result and public derivation order.

Heuristic operations MAY offer a nondeterministic mode, but that mode MUST be
explicit.

## 34. Rust Workspace

The target modular shape is:

```text
crates/
  socrates-math-syntax
  socrates-math-elab
  socrates-math-core
  socrates-math-kernel
  socrates-math-theory
  socrates-math-rewrite
  socrates-math-numeric
  socrates-math-algebra
  socrates-math-solve
  socrates-math-pedagogy
  socrates-math-render
  socrates-math-protocol
  socrates-math-wasm

packages/
  math
```

Not every crate must be created immediately.

The initial repository MAY use fewer physical crates, but it MUST preserve the
logical dependency boundaries so they can be separated without redesigning the
domain model.

### 34.1 Dependency rules

- `core` MUST NOT depend on parsing, rendering, pedagogy, or Wasm.
- `kernel` MUST depend only on core semantics, exact primitives, and checker
  interfaces.
- domain engines MAY depend on core, theories, rewrite infrastructure, and
  kernel interfaces.
- pedagogy MAY depend on verified derivations but MUST NOT be required by the
  kernel.
- rendering MAY depend on semantic objects and derivations but MUST NOT define
  their meaning.
- Wasm MUST adapt public protocols and MUST NOT contain mathematical logic that
  is unavailable to native Rust callers.

## 35. First Vertical Slice

The first complete implementation target is single-variable linear equations
over exact rational numbers.

### 35.1 Supported notation profile

Version `socrates-latex@0.1` MUST support:

- integer literals;
- rational literals using `\frac{a}{b}`;
- identifiers;
- unary negation;
- addition and subtraction;
- multiplication by juxtaposition;
- multiplication using `\cdot`;
- parentheses;
- equality;
- whitespace;
- source spans.

Powers MAY be parsed if needed by surrounding infrastructure, but solving
nonlinear equations is outside this slice.

### 35.2 Initial theories

The slice MUST provide at least:

```text
core.logic@0.1
core.integer@0.1
core.rational@0.1
algebra.ring@0.1
algebra.linear-expression@0.1
algebra.linear-equation@0.1
```

### 35.3 Initial semantic objects

The slice MUST support:

- exact integers;
- normalized exact rationals;
- rational-valued variables;
- ring addition and multiplication;
- additive inverse;
- equality statements;
- finite rational solution sets;
- empty solution sets;
- universal rational solution sets.

### 35.4 Linear expression semantics

The implementation MUST recognize expressions equivalent to:

```text
a*x + b
```

where `a` and `b` are rational values.

Recognition MUST be based on verified normalization rather than brittle syntax
matching.

### 35.5 Linear equation solving

For:

```text
a*x + b = c*x + d
```

the solver MUST handle all cases:

1. `a - c ≠ 0`  
   Return the unique rational solution:

   ```text
   x = (d - b) / (a - c)
   ```

2. `a - c = 0` and `b = d`  
   Return all rational values.

3. `a - c = 0` and `b ≠ d`  
   Return the empty solution set.

The solver MUST return `Complete` for these supported cases.

### 35.6 Initial verified transformations

The slice MUST support verified forms of:

- exact integer and rational arithmetic;
- additive and multiplicative identity removal;
- additive inverse normalization;
- associativity;
- commutativity where explicitly permitted;
- distributivity;
- constant-term collection;
- linear-term collection;
- adding the same expression to both sides;
- subtracting the same expression from both sides;
- multiplying both sides by a nonzero rational;
- dividing both sides by a nonzero rational;
- symmetry and transitivity of equality;
- congruence.

Pedagogical derivations MAY combine several kernel-level proof nodes into one
learner-facing step, but the underlying evidence must remain replayable.

### 35.7 First workflow

For:

```latex
3(x - 2) + 4 = 2x + 9
```

the system MUST be able to:

1. parse the source with spans;
2. elaborate `x` as a rational-valued variable;
3. produce a well-typed equality statement;
4. simplify to an appropriate instructional form;
5. solve completely for `x`;
6. return the verified solution `x = 11`;
7. render every public state deterministically;
8. validate valid intermediate learner work;
9. detect invalid steps with verified evidence or a counterexample;
10. accept valid skipped steps;
11. compare final equations by solution-set equality over the rationals;
12. evaluate expressions and statements under explicit assignments;
13. serialize and deserialize the complete public result.

## 36. First-Slice Learner Validation

The validator MUST support at least:

```text
3(x - 2) + 4 = 2x + 9
→ 3x - 6 + 4 = 2x + 9
```

as a valid distribution step.

It MUST support:

```text
3(x - 2) + 4 = 2x + 9
→ 3x - 2 = 2x + 9
```

as valid with skipped simplification steps.

It MUST reject or disprove:

```text
3(x - 2) + 4 = 2x + 9
→ 3x - 2 = 2x + 8
```

because the equations do not have the same solution set over the rationals.

The invalid result SHOULD include the counterexample `x = 11`, which satisfies
the previous equation but not the submitted equation.

## 37. Testing Requirements

Mathematical correctness is the primary quality requirement.

The project MUST include:

- unit tests;
- rule-checker tests;
- proof replay tests;
- property-based tests;
- metamorphic tests;
- parser fuzzing;
- deserialization fuzzing;
- negative certificate tests;
- deterministic rendering tests;
- serialization compatibility tests;
- regression tests;
- performance benchmarks.

### 37.1 Required properties

Initial property tests MUST include:

- parse/render semantic round-trip;
- canonicalization idempotence;
- proof replay reproduces the conclusion;
- simplification preserves its declared relation;
- solver solutions satisfy the original equation;
- complete linear solve results omit no rational solutions;
- exact serialization round-trip;
- learner-step validation never labels `Unknown` as invalid;
- malformed certificates are rejected.

### 37.2 Differential tests

The project SHOULD compare supported computations against mature systems during
development.

Differential agreement is useful evidence but MUST NOT replace internal
verification.

## 38. Diagnostics

Diagnostics MUST use stable machine-readable codes.

A diagnostic SHOULD include:

- code;
- severity;
- summary;
- source spans;
- related spans;
- semantic occurrence;
- structured parameters;
- suggested repairs;
- localization key.

Mathematical diagnostics and operational errors MUST be distinguishable.

## 39. Security and Robustness

Because the library accepts learner input in browser environments, it MUST
defend against:

- deeply nested syntax;
- parser denial of service;
- rewrite explosion;
- e-graph explosion;
- proof-search explosion;
- enormous integers and rationals;
- malicious serialized DTOs;
- unsupported Wasm memory growth;
- unbounded rendering recursion.

All recursive or expanding subsystems MUST enforce configurable limits.

## 40. Accessibility and Localization

The semantic and derivation models MUST remain independent of natural language.

Explanation text MUST be generated through localization resources.

The architecture SHOULD permit:

- spoken mathematical output;
- structural navigation;
- alternative notation conventions;
- locale-specific decimal and list rendering;
- learner-facing terminology mappings.

Accessibility presentation MUST not alter the underlying semantic object.

## 41. Interoperability

The project SHOULD provide adapters for external systems when useful.

Possible targets include:

- mathematical JSON interchange;
- OpenMath-like semantic identifiers;
- proof assistant exports;
- SMT proof imports;
- external CAS candidate generation;
- MathML-compatible presentation.

Imported results MUST carry trust and provenance metadata.

Unverified imported results MUST not be silently promoted to verified results.

## 42. Evolution Strategy

Development should proceed through coherent capability layers:

1. exact rational arithmetic and linear equations;
2. polynomial algebra and inequalities;
3. functions, domains, radicals, exponentials, logarithms, and trigonometry;
4. linear algebra and differential calculus;
5. integral calculus, limits, sequences, and series;
6. abstract algebraic structures and homomorphisms;
7. real and complex analysis;
8. geometry and semantic diagrams;
9. topology;
10. category theory and universal-property reasoning.

Each layer MUST reuse the common contracts for:

- typed terms;
- contexts;
- relations;
- outcomes;
- proof verification;
- derivations;
- serialization;
- pedagogy.

## 43. Deferred Decisions

The following decisions are intentionally deferred:

- the final foundational type theory;
- dynamic third-party theory loading;
- the complete proof language for advanced domains;
- the external proof-assistant interoperability target;
- distributed or server-side computation;
- persistent global mathematical knowledge storage;
- user-defined executable algorithms;
- general automated theorem proving.

Deferred decisions MUST NOT be accidentally fixed by narrow first-slice public
types.

## 44. Completion Standard

A mathematical feature is complete only when:

1. its input has an explicit syntax and semantic interpretation;
2. required context and assumptions are represented;
3. outputs use explicit mathematical relations;
4. exactness or approximation status is preserved;
5. evidence is produced and checked;
6. unknown and undefined cases are represented honestly;
7. results are serializable and replayable;
8. learner-facing presentation is derived from verified data;
9. resource limits and failure modes are tested;
10. the public API remains independent of implementation-specific bindings.
