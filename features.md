# Symbolic Math Features

This document is the behavior-driven feature catalog for Symbolic Math.

It translates the architectural and semantic contracts in
[`SPEC.md`](./SPEC.md) into product-facing acceptance behavior.

Features are written in executable-style Gherkin, but this document is also a
roadmap. Scenarios tagged `@slice-1` belong to the first complete implementation
target: single-variable linear equations over exact rational numbers.
Scenarios tagged `@future` describe required architectural growth and are not
part of the initial milestone unless separately promoted.

## Conventions

### Result language

Mathematical queries distinguish:

```text
Proven
Disproven
Conditional
Unknown
Undefined
```

Verification status distinguishes:

```text
Verified
Conditional
Unverified
```

A query that the engine cannot settle is `Unknown`; it is never silently
reported as false.

### Mathematical contexts

Every semantic operation is interpreted in an explicit context containing the
relevant:

- theories;
- declarations;
- assumptions;
- domains;
- structures;
- coercions;
- branch conventions.

### Evidence

A transformation label is not sufficient evidence.

Whenever a scenario says that a result is verified, the result must contain
proof data or a checked domain-specific certificate that can be replayed by the
verification layer.

### Exact values

Exact integers and rationals are represented without conversion to binary
floating point or JavaScript `number`.

---

# Part I: Syntax and Elaboration

@syntax @slice-1
## Feature: Parse Supported LaTeX Into Concrete Syntax

Socrates Academy needs learner and author input to retain its written structure
before mathematical meaning is assigned.

```gherkin
Feature: Parse supported LaTeX into concrete syntax

  Background:
    Given the notation profile "socrates-latex@0.1"

  Scenario: Parse a linear equation
    When I parse the source "3(x-2)+4 = 2x+9"
    Then parsing succeeds
    And the result is a concrete syntax tree
    And the top-level syntax form is an equality
    And the original source text is preserved
    And source spans are available for every parsed operand
    And the implicit multiplication between "3" and "(x-2)" is recorded

  Scenario: Preserve explicit learner grouping
    When I parse the source "3((x-2)+4)"
    Then parsing succeeds
    And both explicit parenthesis groups are represented
    And the parser does not erase the learner's grouping

  Scenario: Parse rational notation
    When I parse the source "\frac{1}{2}x + 3 = 4"
    Then parsing succeeds
    And "\frac{1}{2}" is represented as rational-literal syntax
    And its numerator and denominator have source spans

  Scenario: Parse multiplication notation
    When I parse the source "2x + 3\cdot y"
    Then parsing succeeds
    And juxtaposition is represented at "2x"
    And explicit multiplication is represented at "3\cdot y"
```

@syntax @slice-1
## Feature: Reject Unsupported LaTeX Clearly

```gherkin
Feature: Reject unsupported LaTeX clearly

  Background:
    Given the notation profile "socrates-latex@0.1"

  Scenario: Reject an unsupported command
    When I parse a statement containing an unsupported command
    Then parsing is rejected
    And the diagnostic code is stable and machine readable
    And the diagnostic identifies the unsupported command
    And the diagnostic includes a source span when available

  Scenario: Reject an unclosed delimiter
    When I parse the source "3(x-2 + 4"
    Then parsing is rejected or marked incomplete according to the parse policy
    And the diagnostic identifies the unmatched opening delimiter
    And the diagnostic includes the delimiter source span

  Scenario: Reject malformed rational notation
    When I parse the source "\frac{1}{}"
    Then parsing does not produce a valid rational literal
    And the diagnostic identifies the missing denominator
```

@syntax
## Feature: Recover From Incomplete Learner Input

```gherkin
Feature: Recover from incomplete learner input

  Scenario: Preserve a recoverable partial equation
    Given the notation profile "socrates-latex@0.1"
    And parsing is configured for interactive recovery
    When I parse the source "3x + = 7"
    Then the result is "ParsedWithRecovery" or "Incomplete"
    And a hole is present where an operand is missing
    And syntax after the missing operand remains available
    And the recovery diagnostic includes the missing operand location

  Scenario: Do not silently invent mathematical content
    Given parsing is configured for interactive recovery
    When a required operand is absent
    Then the parser may insert a syntax hole
    But the parser does not invent a numeric or symbolic operand
```

@elaboration @slice-1
## Feature: Elaborate Syntax Into Typed Mathematical Objects

```gherkin
Feature: Elaborate syntax into typed mathematical objects

  Background:
    Given the theories "core.logic@0.1", "core.rational@0.1",
      "algebra.ring@0.1", and "algebra.linear-equation@0.1" are imported
    And the context declares "x : Rational"

  Scenario: Elaborate a linear equation
    Given the parsed source "3(x-2)+4 = 2x+9"
    When I elaborate it as a statement
    Then elaboration succeeds
    And the result is a typed equality statement
    And both sides have type "Rational"
    And the equality relation has stable semantic identity
    And inserted operations use theory-qualified semantic symbols
    And the semantic object retains origin mappings to the source syntax

  Scenario: Elaborate exact rational arithmetic
    Given the parsed source "\frac{1}{2} + \frac{1}{3}"
    When I elaborate it as an expression
    Then elaboration succeeds
    And the result has type "Rational"
    And both rational literals remain exact

  Scenario: Reject a statement where an expression is required
    Given the parsed source "x = 2"
    When I elaborate it with expected kind "expression"
    Then elaboration is rejected
    And the diagnostic code is "TypeMismatch" or "UnsupportedSemanticForm"
```

@elaboration
## Feature: Resolve Notation Through Context

```gherkin
Feature: Resolve notation through context

  Scenario: Resolve overloaded addition
    Given a theory provides scalar addition on rationals
    And the context declares "x : Rational"
    When I elaborate "x + 1"
    Then "+" resolves to rational addition
    And the semantic identity of the operation is not the display string "+"

  @future
  Scenario: Resolve vector addition
    Given a theory provides vector addition on a vector space "V"
    And the context declares "u : V" and "v : V"
    When I elaborate "u + v"
    Then "+" resolves to the addition operation of "V"

  @future
  Scenario: Reject genuinely ambiguous notation
    Given two imported notations match the same syntax
    And neither meaning is preferred by type or context
    When I elaborate the syntax
    Then the result is "Ambiguous"
    And all viable interpretations are reported
    And no interpretation is silently selected
```

@elaboration
## Feature: Report Elaboration Failures Precisely

```gherkin
Feature: Report elaboration failures precisely

  Scenario: Report an unknown symbol
    Given the context does not declare "y"
    When I elaborate "x + y"
    Then elaboration is rejected
    And the diagnostic code is "UnknownSymbol"
    And the diagnostic identifies "y"
    And the diagnostic includes its source span

  Scenario: Report a missing theory
    Given the required operation belongs to a theory that is not imported
    When I elaborate the operation
    Then elaboration is rejected
    And the diagnostic code is "TheoryNotImported"
    And the missing theory or capability is identified

  Scenario: Report an unresolved type
    Given the notation is valid but its type cannot be inferred
    When I elaborate it
    Then the result is "Incomplete" or "Rejected"
    And the diagnostic code is "CannotInferType"
```

---

# Part II: Semantic Objects, Contexts, and Theories

@semantics @slice-1
## Feature: Preserve Stable Semantic Identity

```gherkin
Feature: Preserve stable semantic identity

  Scenario: Distinguish semantic identity from notation
    Given two notations render the same rational addition operation
    When both are elaborated
    Then both refer to the same stable symbol identifier
    And changing display notation does not change semantic identity

  Scenario: Compute semantic hashes independently of source formatting
    Given the sources "x+1" and "x + 1"
    When both elaborate to the same typed term
    Then their semantic hashes are equal
    And their concrete syntax and source spans may remain different

  Scenario: Exclude explanation text from semantic identity
    Given one verified term has explanations in English
    And the same term has explanations in German
    Then both presentations have the same semantic identity
```

@context @slice-1
## Feature: Interpret Mathematics In Explicit Contexts

```gherkin
Feature: Interpret mathematics in explicit contexts

  Scenario: Declare the domain of a variable
    Given a context declaring "x : Rational"
    When I elaborate "x + 1"
    Then the term is interpreted in that context
    And the variable declaration is referenced by stable identity

  Scenario: Extend a context immutably
    Given a context "Γ"
    When I add the hypothesis "x != 0"
    Then a new context "Γ1" is created
    And "Γ" remains unchanged
    And "Γ1" records its relationship to "Γ"

  Scenario: Record assumptions used by a proof
    Given the context contains assumptions "A1" and "A2"
    When a verified transformation uses only "A2"
    Then the result lists "A2" as consumed
    And it does not claim to consume "A1"
```

@context
## Feature: Represent Conditional Mathematics

```gherkin
Feature: Represent conditional mathematics

  Scenario: Simplify a quotient under a nonzero assumption
    Given the context declares "x" in a field
    And the context proves "x != 0"
    When I transform "x / x" using cancellation
    Then the result is proven equal to "1"
    And the proof records the nonzero hypothesis used

  Scenario: Return a condition when cancellation is not justified
    Given the context declares "x" in a field
    And the context does not prove "x != 0"
    When I request cancellation of "x / x"
    Then the result is "Conditional" or the transformation is not applied
    And the required condition "x != 0" is reported
    And the library does not return an unconditional proof that "x / x = 1"
```

@context
## Feature: Handle Inconsistent Contexts Honestly

```gherkin
Feature: Handle inconsistent contexts honestly

  Scenario: Report a known inconsistent context
    Given the context contains verified hypotheses "x = 0" and "x != 0"
    When the context is checked
    Then the context consistency status is inconsistent
    And later pedagogical results report that status

  Scenario: Do not use explosion as ordinary tutoring evidence
    Given the context is known to be inconsistent
    When a learner asks whether an unrelated statement follows
    Then the system does not present an arbitrary conclusion as an ordinary
      pedagogical proof
    And it reports that the assumptions are inconsistent
```

@theory @slice-1
## Feature: Load Versioned Theory Packages

```gherkin
Feature: Load versioned theory packages

  Scenario: Load the first-slice theories
    When the engine loads the first vertical slice
    Then it loads versioned packages for logic, integers, rationals, rings,
      linear expressions, and linear equations
    And each exported symbol has a stable identifier
    And each exported rule has a stable identifier

  Scenario: Reject an incompatible dependency
    Given a theory package requires an incompatible version of another theory
    When the theory environment is constructed
    Then loading fails
    And the incompatible package versions are reported

  Scenario: Report proof dependencies
    Given a result is verified using rules from multiple theory packages
    When I inspect the result provenance
    Then all required theory package identifiers and versions are listed
```

@theory
## Feature: Keep Axioms Explicit

```gherkin
Feature: Keep axioms explicit

  Scenario: Report foundational dependencies
    Given a proof depends on declared ring axioms
    When I inspect the proof
    Then the relevant axioms or theorem dependencies are queryable
    And they are identified by stable semantic identifiers

  Scenario: Do not hide new axioms in a rewrite rule
    Given a proposed rule is not derivable from the imported theory
    When the rule is registered as a theorem-backed rewrite
    Then registration or verification fails
    Unless the rule is explicitly declared as an axiom by policy
```

@coercion @slice-1
## Feature: Insert Only Valid Canonical Coercions

```gherkin
Feature: Insert only valid canonical coercions

  Scenario: Insert an integer-to-rational coercion
    Given the context expects a rational value
    When I elaborate the integer literal "2"
    Then the term may include the canonical embedding from Integer to Rational
    And the coercion is recoverable from provenance
    And the verifier confirms it is well typed

  Scenario: Reject an ambiguous coercion
    Given two incomparable coercion paths can convert a term to the expected type
    When I elaborate the term
    Then elaboration is rejected as "AmbiguousCoercion"
    And both candidate paths are reported

  Scenario: Require explicit lossy conversion
    Given a conversion would lose exact information
    When elaboration considers the conversion
    Then it is not inserted implicitly
```

---

# Part III: Proofs, Rules, and Verification

@proof @slice-1
## Feature: Produce Structured Proof Nodes

```gherkin
Feature: Produce structured proof nodes

  Scenario: Record a distributive transformation
    Given the expression "3(x - 2)"
    When I distribute multiplication over subtraction
    Then the output is "3x - 6"
    And the result records a stable rule identifier
    And the result records the matched operands
    And the result records the changed occurrence
    And the result records the equality relation established
    And the result contains verifier data

  Scenario: Record premises and side conditions
    Given a transformation requires a premise
    When the transformation is produced
    Then the proof node references the premise
    And discharged side conditions are distinguished from remaining obligations
```

@proof @slice-1
## Feature: Verify Every Authoritative Transformation

```gherkin
Feature: Verify every authoritative transformation

  Scenario: Accept a valid proof node
    Given a proof node instantiating a valid distributive rule
    When the verification kernel checks it
    Then verification succeeds
    And the conclusion is marked "Verified"

  Scenario: Reject a malformed substitution
    Given a proof node claims to instantiate a rule
    But its substitution does not match the rule pattern
    When the verification kernel checks it
    Then verification fails
    And the claimed conclusion is not marked "Verified"

  Scenario: Reject a missing side condition
    Given a proof node uses division by a symbolic expression
    And no proof that the expression is nonzero is supplied
    When the verification kernel checks it
    Then verification fails or leaves a visible obligation
    And no unconditional conclusion is accepted

  Scenario: Reject a changed conclusion
    Given a valid proof node has been serialized
    And its conclusion is modified without updating its evidence
    When the verification kernel checks it
    Then verification fails
```

@proof
## Feature: Separate Derivations From Explanations

```gherkin
Feature: Separate derivations from explanations

  Scenario: Generate two explanations from one derivation
    Given a verified derivation
    When I request a concise explanation
    And I request an instructional explanation
    Then both explanations refer to the same proof data
    And the wording and granularity may differ
    And neither explanation string is used as proof evidence

  Scenario: Localize without changing mathematics
    Given a verified derivation
    When I render explanations in two languages
    Then the derivation and semantic result are unchanged
```

@proof
## Feature: Replay Proof Graphs

```gherkin
Feature: Replay proof graphs

  Scenario: Replay a verified derivation
    Given a serialized verified derivation
    And all required theory and rule versions are available
    When I replay every proof node
    Then every node verifies
    And the reproduced final judgment matches the serialized conclusion

  Scenario: Reject replay with an incompatible rule version
    Given a derivation references a rule version with different semantics
    When I replay the derivation
    Then replay is rejected
    And the incompatible rule identifier and version are reported
```

@certificate @future
## Feature: Verify Domain-Specific Certificates

```gherkin
Feature: Verify domain-specific certificates

  Scenario: Accept a valid polynomial identity certificate
    Given a polynomial engine produces a certificate that two polynomials are equal
    When the certificate checker verifies it
    Then the equality result is marked "Verified"
    And the certificate format identifier and version are recorded

  Scenario: Reject a corrupted certificate
    Given a valid certificate is modified
    When the certificate checker verifies it
    Then verification fails
    And the claimed conclusion is not accepted

  Scenario: Keep certificate checkers versioned
    Given a serialized result references a certificate checker version
    When that version is unavailable or incompatible
    Then deserialization or replay does not silently trust the certificate
```

@rewrite
## Feature: Apply Conditional Rewrite Rules Soundly

```gherkin
Feature: Apply conditional rewrite rules soundly

  Scenario: Apply a rule whose side conditions are proven
    Given a conditional rewrite rule
    And every side condition is proven in the context
    When the rule is applied
    Then the transformation is verified
    And the used conditions are recorded

  Scenario: Return unresolved obligations
    Given a conditional rewrite rule
    And a side condition is neither proven nor disproven
    When the rule is considered
    Then the engine may return a conditional result
    And the unresolved condition is explicit

  Scenario: Refuse an unsatisfied rule
    Given a conditional rewrite rule
    And a required side condition is disproven
    When the rule is considered
    Then the rule is not applied
```

@egraph @future
## Feature: Use Equality Saturation Only As Search

```gherkin
Feature: Use equality saturation only as search

  Scenario: Verify an extracted rewrite path
    Given an equality-saturation engine discovers a candidate path
    When the path is extracted
    Then every step is replayed through the verification kernel
    And the result is authoritative only if replay succeeds

  Scenario: Reject an unsound equality path
    Given the search graph contains a path produced by incompatible conditional rules
    When the path is verified
    Then verification fails
    And the candidate equality is not marked proven
```

---

# Part IV: Exact Values and Evaluation

@numeric @slice-1
## Feature: Represent Exact Integers And Rationals

```gherkin
Feature: Represent exact integers and rationals

  Scenario: Normalize a rational value
    When I construct the rational value "10/15"
    Then its exact normalized value is "2/3"
    And the denominator is positive
    And no floating-point conversion occurs

  Scenario: Preserve an arbitrarily large integer
    Given an integer larger than the exact JavaScript number range
    When it crosses the public TypeScript boundary
    Then every digit is preserved
    And it is represented as tagged exact data rather than a JavaScript number

  Scenario: Reject a zero denominator
    When I construct a rational with denominator zero
    Then the operation returns "Undefined" or a construction diagnostic
    And no rational value is created
```

@evaluation @slice-1
## Feature: Evaluate Expressions Exactly

```gherkin
Feature: Evaluate expressions exactly

  Scenario: Evaluate an expression under an assignment
    Given the expression "x + 2"
    And the assignment "x = 3"
    When I evaluate the expression
    Then the result is the exact integer "5"
    And the result is verified
    And no JavaScript number conversion is required

  Scenario: Evaluate rational arithmetic exactly
    Given the expression "\frac{1}{2} + \frac{1}{3}"
    When I evaluate the expression
    Then the result is the exact rational "5/6"
    And the proof records exact rational arithmetic

  Scenario: Evaluate a negative rational exactly
    Given the expression "-\frac{4}{6}"
    When I evaluate the expression
    Then the result is the exact rational "-2/3"
```

@evaluation @slice-1
## Feature: Partially Evaluate Expressions

```gherkin
Feature: Partially evaluate expressions

  Scenario: Partially evaluate an expression
    Given the expression "x + y + 2"
    And the assignment "x = 3"
    When I partially evaluate the expression
    Then the residual expression is equivalent to "y + 5"
    And the performed substitution is reported
    And the unresolved symbol "y" is reported
    And the preserved relation is explicit

  Scenario: Refuse incomplete total evaluation
    Given the expression "x + y"
    And the assignment "x = 3"
    When I request total evaluation
    Then no exact total value is returned
    And the unresolved symbol "y" is reported
```

@evaluation @slice-1
## Feature: Evaluate Statements Pointwise

```gherkin
Feature: Evaluate statements pointwise

  Scenario: Evaluate a true equation
    Given the equation "x + 2 = 5"
    And the assignment "x = 3"
    When I evaluate the statement
    Then the pointwise truth result is true

  Scenario: Evaluate a false equation
    Given the equation "x + 2 = 6"
    And the assignment "x = 3"
    When I evaluate the statement
    Then the pointwise truth result is false

  Scenario: Distinguish evaluation from general proof
    Given the equation "x + 2 = 5"
    And the assignment "x = 3"
    When pointwise evaluation returns true
    Then the result does not claim the equation is true for every rational "x"
```

@evaluation
## Feature: Distinguish Undefined And Unknown Evaluation

```gherkin
Feature: Distinguish undefined and unknown evaluation

  Scenario: Report division by zero as undefined
    Given the expression "1 / x"
    And the assignment "x = 0"
    When I evaluate the expression
    Then the result is "Undefined"
    And the reason identifies division by zero

  @future
  Scenario: Report an unsupported special function as unknown
    Given an expression is mathematically defined
    But the active engine has no evaluation method for it
    When I evaluate the expression
    Then the result is "Unknown"
    And the reason is "UnsupportedDomain" or "NoApplicableMethod"
```

@approximation @future
## Feature: Approximate Explicitly And With Error Information

```gherkin
Feature: Approximate explicitly and with error information

  Scenario: Request a numerical approximation
    Given an exact mathematical value
    When I approximate it to a requested precision
    Then the result is marked approximate
    And its precision is reported
    And an error bound or enclosure is reported when supported
    And the exact value is not replaced in stored mathematics

  Scenario: Keep approximation separate from exact equality
    Given two approximate values overlap within their error bounds
    When I compare them
    Then the engine uses an approximation relation
    And it does not claim exact equality without proof
```

---

# Part V: Canonicalization, Normalization, and Transformation

@canonicalization @slice-1
## Feature: Canonicalize Deterministically

```gherkin
Feature: Canonicalize deterministically

  Scenario: Canonicalize equivalent rational literals
    Given the rational literals "2/4" and "1/2"
    When I canonicalize both
    Then both canonical values are identical
    And the canonicalization is verified

  Scenario: Canonicalization is idempotent
    Given a semantic object
    When I canonicalize it twice
    Then the second result is semantically identical to the first

  Scenario: Canonicalization does not erase original syntax
    Given a learner entered "\frac{2}{4}"
    When the semantic value is canonicalized to "1/2"
    Then the original syntax remains available for learner-facing feedback
```

@normalization @slice-1
## Feature: Normalize Linear Expressions

```gherkin
Feature: Normalize linear expressions

  Background:
    Given the context declares "x : Rational"

  Scenario: Recognize a linear expression
    Given the expression "3(x - 2) + 4"
    When I normalize it using rational linear-expression normalization
    Then the normal form represents coefficient "3" and constant "-2"
    And the normalization is verified
    And the normalizer reports its theory and completeness domain

  Scenario: Reject a nonlinear expression from the linear normalizer
    Given the expression "x^2 + 1"
    When I request linear-expression normalization
    Then the result is "Unknown" or "UnsupportedDomain"
    And the system does not misclassify the expression as linear
```

@simplification @slice-1
## Feature: Simplify According To An Explicit Policy

```gherkin
Feature: Simplify according to an explicit policy

  Scenario: Simplify arithmetic inside a linear expression
    Given the expression "3x - 6 + 4"
    When I simplify it using policy "LinearEquationInstructional"
    Then the result is "3x - 2"
    And the derivation records exact constant arithmetic
    And the derivation records the changed occurrence
    And the result is proven equal to the input

  Scenario: Preserve meaningful instructional structure
    Given the expression "3(x - 2) + 4"
    When I simplify it using policy "LinearEquationInstructional"
    Then one public step distributes multiplication over subtraction
    And a later public step combines exact constant terms
    And the public derivation does not collapse directly to the final form

  Scenario: Allow a compact policy
    Given the expression "3(x - 2) + 4"
    When I simplify it using a compact verified policy
    Then the system may return "3x - 2" in one public step
    But the underlying proof graph remains replayable
```

@simplification
## Feature: Avoid A Universal Simplest Form

```gherkin
Feature: Avoid a universal simplest form

  @future
  Scenario: Prefer expanded polynomial form
    Given the expression "(x + 1)^2"
    When I simplify with policy "PolynomialExpanded"
    Then the preferred result is "x^2 + 2x + 1"

  @future
  Scenario: Prefer factored polynomial form
    Given the expression "x^2 + 2x + 1"
    When I simplify with policy "PolynomialFactored"
    Then the preferred result is "(x + 1)^2"

  Scenario: Record the policy used
    Given any simplification result
    Then the result reports the simplification policy identifier and version
```

@transformation @slice-1
## Feature: Apply Requested Transformations

```gherkin
Feature: Apply requested transformations

  Scenario: Distribute a product
    Given the expression "3(x - 2)"
    When I request transformation "Distribute"
    Then the result is "3x - 6"
    And the equality is verified

  Scenario: Add the same rational to both sides
    Given the equation "x - 4 = 7"
    When I add "4" to both sides
    Then the result is "x = 11"
    And the transformation establishes the same solution set over rationals
    And the operation applied to both sides is recorded

  Scenario: Divide both sides by a nonzero rational
    Given the equation "2x = 1"
    When I divide both sides by "2"
    Then the result is "x = \frac{1}{2}"
    And nonzeroness of "2" is verified
    And the solution set is preserved

  Scenario: Reject division by zero
    Given an equation
    When I request division of both sides by "0"
    Then the transformation is rejected
    And the reason identifies the failed nonzero side condition
```

---

# Part VI: Comparison and Mathematical Relations

@comparison @slice-1
## Feature: Compare Syntax Explicitly

```gherkin
Feature: Compare syntax explicitly

  Scenario: Distinguish different trees
    Given the expressions "x + 1" and "1 + x"
    When I compare them using relation "syntax.same_tree"
    Then the result is "Disproven"
    And no algebraic commutativity rule is used

  Scenario: Recognize identical represented trees
    Given two copies of the represented expression "x + 1"
    When I compare them using relation "syntax.same_tree"
    Then the result is "Proven"
```

@comparison @slice-1
## Feature: Compare Expressions Under A Mathematical Theory

```gherkin
Feature: Compare expressions under a mathematical theory

  Background:
    Given the context declares "x : Rational"

  Scenario: Prove algebraic equality
    Given the expressions "3(x - 2) + 4" and "3x - 2"
    When I compare them using rational ring equality
    Then the result is "Proven"
    And the result includes verified evidence
    And the assumptions used are reported

  Scenario: Disprove unequal constant expressions
    Given the expressions "2 + 2" and "5"
    When I compare them using rational equality
    Then the result is "Disproven"
    And the refutation is verified

  Scenario: Return unknown when the method is incomplete
    Given a relation whose active procedure is incomplete
    And neither equality nor inequality is established
    When I compare two objects
    Then the result is "Unknown"
    And it is not reported as "Disproven"
```

@comparison @slice-1
## Feature: Compare Equations By Solution Set

```gherkin
Feature: Compare equations by solution set

  Background:
    Given the variable "x" ranges over the rationals

  Scenario: Accept equivalent solved and unsolved equations
    Given the equations "x + 1 = 3" and "x = 2"
    When I compare them using relation "equation.same_solution_set"
    Then the result is "Proven"
    And the variable set is "{x}"
    And both solution sets are reported as "{2}"
    And the comparison is complete

  Scenario: Reject equations with different solution sets
    Given the equations "x + 1 = 3" and "x = 3"
    When I compare them using relation "equation.same_solution_set"
    Then the result is "Disproven"
    And the differing solution sets are reported
    And a distinguishing assignment is reported when available

  Scenario: Require explicit variables and domains
    Given equations containing more than one free symbol
    When I request solution-set comparison without specifying solved variables
    Then the operation is rejected or returns an ambiguity diagnostic
    And no hidden choice of variables is made
```

@comparison
## Feature: Preserve Relation-Specific Meaning

```gherkin
Feature: Preserve relation-specific meaning

  Scenario: Do not confuse implication with equivalence
    Given statement "A" implies statement "B"
    But "B" does not imply "A"
    When I compare them using logical equivalence
    Then the result is not "Proven"
    And the available one-way implication may be reported separately

  @future
  Scenario: Compare functions on an explicit domain
    Given two function expressions
    And a domain "D"
    When I request extensional equality on "D"
    Then the result refers to equality of outputs for inputs in "D"
    And behavior outside "D" is not silently included

  @future
  Scenario: Compare structures by isomorphism
    Given two algebraic structures
    When I request isomorphism
    Then the system does not reduce the request to syntactic equality
```

---

# Part VII: Solving

@solve @slice-1
## Feature: Solve Single-Variable Linear Equations Over Rationals

```gherkin
Feature: Solve single-variable linear equations over rationals

  Background:
    Given the theories for rational linear equations are loaded
    And the context declares "x : Rational"

  Scenario: Solve an equation with an integer solution
    Given the equation "3(x - 2) + 4 = 2x + 9"
    When I solve for "x" over rationals
    Then the result is "Proven"
    And the solution is "x = 11"
    And the completeness status is "Complete"
    And the derivation is deterministic
    And every public step references verified proof data

  Scenario: Solve an equation with a rational solution
    Given the equation "2x = 1"
    When I solve for "x" over rationals
    Then the solution is "x = \frac{1}{2}"
    And the exact rational is not converted to floating point
    And the completeness status is "Complete"

  Scenario: Solve an equation with a negative rational solution
    Given the equation "3x = -2"
    When I solve for "x" over rationals
    Then the solution is "x = -\frac{2}{3}"
    And the completeness status is "Complete"
```

@solve @slice-1
## Feature: Handle Every Linear-Equation Solution Class

```gherkin
Feature: Handle every linear-equation solution class

  Background:
    Given the variable "x" ranges over the rationals

  Scenario: Return one solution
    Given the equation "5x + 2 = 2x + 11"
    When I solve for "x"
    Then the solution set is "{3}"
    And the completeness status is "Complete"

  Scenario: Return no solutions
    Given the equation "2x + 1 = 2x + 3"
    When I solve for "x"
    Then the solution set is empty
    And the completeness status is "Complete"
    And the derivation reduces the equation to a verified contradiction

  Scenario: Return all rational values
    Given the equation "2(x + 1) = 2x + 2"
    When I solve for "x"
    Then the solution set is all rational values
    And the completeness status is "Complete"
    And the derivation reduces the equation to a verified identity
```

@solve @slice-1
## Feature: Produce A Verified Instructional Derivation

```gherkin
Feature: Produce a verified instructional derivation

  Scenario: Solve by simplifying and isolating the variable
    Given the equation "3(x - 2) + 4 = 2x + 9"
    When I solve using policy "LinearEquationInstructional"
    Then the public derivation includes an equivalent form "3x - 2 = 2x + 9"
    And the derivation next isolates the variable
    And the final result is "x = 11"
    And every step states the relation preserved
    And every step can be independently replayed

  Scenario: Keep the original problem available
    Given a solve result
    Then the result retains the original semantic statement
    And accepted final solutions can be checked against that original statement
```

@solve
## Feature: Track Relation Changes During Solving

```gherkin
Feature: Track relation changes during solving

  @future
  Scenario: Record a potentially extraneous operation
    Given a solving step squares both sides of an equation
    When the step is produced
    Then the step records implication rather than automatic equivalence
    And candidate solutions are checked against the original equation

  @future
  Scenario: Record a potentially solution-losing operation
    Given a solving step divides by an expression that may be zero
    When the step is considered
    Then the zero case is split or preserved as an obligation
    And completeness is not claimed unless every case is handled
```

@solve
## Feature: Report Unsupported Solving Honestly

```gherkin
Feature: Report unsupported solving honestly

  Scenario: Reject a nonlinear equation in the first slice
    Given the equation "x^2 = 2"
    When I solve using only the first-slice linear solver
    Then the result is "Unknown"
    And the reason is "UnsupportedDomain"
    And the system does not return an incomplete candidate as a complete solution

  Scenario: Report resource exhaustion
    Given a supported operation exceeds its resource budget
    When the budget is exhausted
    Then the result is "Unknown"
    And the reason is "ResourceLimit"
```

---

# Part VIII: Learner-Step Validation

@validation @slice-1
## Feature: Accept A Valid Learner Step

```gherkin
Feature: Accept a valid learner step

  Scenario: Accept distribution
    Given the previous equation "3(x - 2) + 4 = 2x + 9"
    When a learner submits "3x - 6 + 4 = 2x + 9"
    Then the validation status is "Valid"
    And the recognized transformation is distribution
    And the strongest preserved relation is the same solution set over rationals
    And the changed occurrence is identified
    And the proof is verified

  Scenario: Accept adding the same value to both sides
    Given the previous equation "x - 4 = 7"
    When a learner submits "x = 11"
    Then the validation status is "Valid"
    And the balancing operation is recognized
    And solution-set preservation is verified
```

@validation @slice-1
## Feature: Accept Valid Skipped Steps

```gherkin
Feature: Accept valid skipped steps

  Scenario: Accept multiple simplifications in one learner step
    Given the previous equation "3(x - 2) + 4 = 2x + 9"
    When a learner submits "3x - 2 = 2x + 9"
    Then the validation status is "ValidWithSkippedSteps"
    And the skipped verified transformations are recoverable
    And the learner is not required to match the engine's preferred granularity

  Scenario: Accept a different valid strategy
    Given a linear equation can be solved by more than one sequence
    When the learner submits a state on a valid alternative path
    Then the step is accepted
    And it is not rejected merely because it differs from the suggested next step
```

@validation @slice-1
## Feature: Reject A Proven Invalid Learner Step

```gherkin
Feature: Reject a proven invalid learner step

  Scenario: Detect a changed solution set
    Given the previous equation "3(x - 2) + 4 = 2x + 9"
    When a learner submits "3x - 2 = 2x + 8"
    Then the validation status is "InvalidWithCounterexample"
    And the system proves that the solution set changed
    And the counterexample "x = 11" is reported
    And the changed right-hand constant is highlighted

  Scenario: Detect an invalid division step
    Given a learner divides by an expression
    And the expression may equal zero
    When the submitted step discards the zero case
    Then the validation status is "InvalidSideCondition" or
      "ValidUnderAdditionalCondition"
    And the missing nonzero condition is reported
```

@validation
## Feature: Distinguish Invalid From Unverified

```gherkin
Feature: Distinguish invalid from unverified

  Scenario: Return unverified for an unsupported but possibly valid step
    Given a learner submits a step outside every active verification procedure
    And no counterexample or refutation is found
    When I validate the step
    Then the validation status is "Unverified"
    And the step is not labeled mathematically invalid

  Scenario: Return ambiguous for multiple plausible parses
    Given the learner submission has multiple viable semantic interpretations
    When I validate the step
    Then the validation status is "Ambiguous"
    And the interpretations are reported
```

@validation
## Feature: Diagnose Learner Errors Structurally

```gherkin
Feature: Diagnose learner errors structurally

  Scenario: Report the smallest changed occurrence
    Given a submitted step differs from the previous state in one subexpression
    When validation identifies the change
    Then the smallest relevant semantic occurrence is reported
    And its source span is reported when available

  Scenario: Suggest a minimal repair
    Given an invalid linear-equation step differs by one constant
    When a verified minimal repair is available
    Then the result may suggest that repair
    And the repair is represented as structured mathematical data

  Scenario: Associate a misconception identifier
    Given an invalid step matches a known misconception pattern
    When validation reports the error
    Then a stable misconception identifier may be included
    And the misconception label does not replace the mathematical refutation
```

@validation
## Feature: Keep Strategy Recognition Separate From Validity

```gherkin
Feature: Keep strategy recognition separate from validity

  Scenario: Validate without recognizing a named strategy
    Given a submitted step is proven valid
    But no known pedagogical strategy matches it
    When I validate the step
    Then the validation status is still valid
    And strategy recognition may be absent or low confidence

  Scenario: Do not accept a step solely because it resembles a strategy
    Given a submitted step resembles distribution
    But the resulting equation is not equivalent
    When I validate the step
    Then the step is invalid or unverified according to evidence
    And the strategy resemblance does not override mathematics
```

---

# Part IX: Pedagogical Derivations and Explanations

@pedagogy @slice-1
## Feature: Select Step Granularity

```gherkin
Feature: Select step granularity

  Scenario: Produce an instructional derivation
    Given a verified solution proof
    When I request instructional granularity
    Then arithmetic, distribution, and equation-balancing steps are shown
    And every public step corresponds to verified proof data

  Scenario: Produce a concise derivation
    Given the same verified solution proof
    When I request concise granularity
    Then several low-level proof nodes may be grouped
    And the final mathematical result remains identical

  Scenario: Do not fabricate omitted reasoning
    Given a public step groups several proof nodes
    Then the grouped nodes remain available for expansion
```

@pedagogy
## Feature: Generate Hints From Verified Search

```gherkin
Feature: Generate hints from verified search

  Scenario: Provide the next action without revealing the solution
    Given a verified solution path is available
    When I request a minimal hint
    Then the hint identifies a valid next mathematical action
    And it does not require displaying the final answer

  Scenario: Verify a hint before presenting it
    Given a candidate hint proposes a transformation
    When the transformation cannot be verified
    Then the hint is not presented as mathematically valid
```

@pedagogy
## Feature: Track Pedagogical Prerequisites

```gherkin
Feature: Track pedagogical prerequisites

  Scenario: Report concepts used by a derivation
    Given a derivation uses distributivity and additive inverses
    When I inspect its pedagogical metadata
    Then the associated concept identifiers are reported

  Scenario: Select an explanation compatible with learner knowledge
    Given the learner model marks one strategy as not yet introduced
    And an alternative verified strategy is available
    When I request an instructional solution
    Then the pedagogical planner may select the compatible strategy
    And mathematical correctness remains unchanged
```

---

# Part X: Rendering

@rendering @slice-1
## Feature: Render Mathematical Objects Deterministically

```gherkin
Feature: Render mathematical objects deterministically

  Scenario: Render a parsed equation
    Given the elaborated equation "3(x-2)+4 = 2x+9"
    When I render it as KaTeX-compatible LaTeX in canonical mode
    Then the output is deterministic
    And the output is accepted by the supported KaTeX profile

  Scenario: Render an exact rational
    Given the exact rational value "5/6"
    When I render it as instructional LaTeX
    Then the output is "\frac{5}{6}"

  Scenario: Render the same object repeatedly
    Given a semantic object and fixed rendering options
    When I render it multiple times
    Then every output is identical
```

@rendering @slice-1
## Feature: Preserve Semantic Round Trips

```gherkin
Feature: Preserve semantic round trips

  Scenario: Round-trip a linear equation
    Given an elaborated linear equation
    When I render it in canonical LaTeX
    And parse and elaborate the rendered output in the same context
    Then the new object is semantically equivalent to the original
    And the relation used for the round trip is reported

  Scenario: Do not claim source identity from semantic round trip
    Given rendering changes whitespace or explicit grouping
    When semantic round trip succeeds
    Then the system does not claim the concrete syntax trees are identical
```

@rendering
## Feature: Offer Distinct Rendering Modes

```gherkin
Feature: Offer distinct rendering modes

  Scenario: Render close to learner input
    Given an object retains source-origin information
    When I render in "OriginalLike" mode
    Then the output preserves learner grouping where possible

  Scenario: Render canonical notation
    Given the same object
    When I render in "Canonical" mode
    Then the output follows deterministic canonical notation

  Scenario: Render an instructional step
    Given a derivation step with a focused occurrence
    When I render in "Instructional" mode
    Then the changed occurrence can be identified for highlighting

  @future
  Scenario: Render accessible speech
    Given a semantic mathematical object
    When I render in "AccessibleSpeech" mode
    Then the output is derived from semantic structure
    And it is not generated solely by reading raw LaTeX characters
```

---

# Part XI: Serialization and Public API

@serialization @slice-1
## Feature: Serialize Public Mathematical Results

```gherkin
Feature: Serialize public mathematical results

  Scenario: Serialize and deserialize a solve result
    Given a verified solve result for "3(x - 2) + 4 = 2x + 9"
    When I serialize the result to the public protocol
    And deserialize it with compatible versions
    Then the mathematical conclusion is unchanged
    And every exact number is preserved without precision loss
    And every derivation step remains replayable
    And every semantic identifier is preserved

  Scenario: Serialize exact rationals as tagged data
    Given the exact rational "5/6"
    When I serialize it
    Then the numerator is the string "5"
    And the denominator is the string "6"
    And no JSON number represents the exact rational

  Scenario: Preserve assumptions and obligations
    Given a conditional verified result
    When I serialize and deserialize it
    Then every used assumption is preserved
    And every unresolved obligation is preserved
```

@serialization
## Feature: Reject Incompatible Serialized Mathematics

```gherkin
Feature: Reject incompatible serialized mathematics

  Scenario: Reject an incompatible schema version
    Given a serialized result uses an unsupported schema version
    When I deserialize it
    Then deserialization is rejected or requires an explicit migration
    And the incompatibility is reported

  Scenario: Reject an unavailable theory version
    Given a proof depends on a theory package version that is unavailable
    When I deserialize it for verification
    Then the proof is not silently trusted
    And the missing package is reported

  Scenario: Reject unknown certificate formats
    Given a result contains an unknown certificate format identifier
    When I deserialize it
    Then the certificate remains unverified
    And the result is not promoted to verified
```

@api @slice-1
## Feature: Keep Generated Wasm Types Out Of Public TypeScript

```gherkin
Feature: Keep generated Wasm types out of public TypeScript

  Scenario: Public operations return stable DTOs
    Given a browser application uses "@socrates/math"
    When it parses, elaborates, simplifies, solves, evaluates, compares,
      validates, renders, or serializes mathematics
    Then it receives handwritten facade objects and versioned DTOs
    And it does not depend on generated wasm-bindgen classes

  Scenario: Change the internal binding strategy
    Given the internal WebAssembly adapter changes
    When the public DTO contract is unchanged
    Then existing application code does not require generated-type changes
```

@api @slice-1
## Feature: Separate Mathematical Outcomes From Operational Failures

```gherkin
Feature: Separate mathematical outcomes from operational failures

  Scenario: Return unknown as an ordinary mathematical result
    Given a mathematical query exceeds its supported domain
    When I invoke the operation
    Then the API returns a tagged "Unknown" result
    And it does not throw an exception merely because the query is unresolved

  Scenario: Reject a corrupted protocol operationally
    Given a public DTO is structurally corrupted
    When I submit it to the engine
    Then the API reports a protocol or deserialization error
    And it does not report the mathematics as "Undefined"
```

---

# Part XII: Determinism, Budgets, and Robustness

@determinism @slice-1
## Feature: Produce Deterministic First-Slice Results

```gherkin
Feature: Produce deterministic first-slice results

  Scenario: Repeat a solve operation
    Given identical semantic input, context, theory versions, policy, and budget
    When I solve the same linear equation multiple times in deterministic mode
    Then the final result is identical
    And the public derivation order is identical
    And the rendered output is identical

  Scenario: Preserve determinism across serialization
    Given a deterministic solve result is serialized and replayed
    Then the replayed proof graph has the same public ordering and conclusion
```

@budget
## Feature: Enforce Resource Budgets

```gherkin
Feature: Enforce resource budgets

  Scenario: Stop an operation at its rewrite limit
    Given an operation has a maximum rewrite count
    When the limit is reached before the query is settled
    Then the result is "Unknown"
    And the reason is "ResourceLimit"
    And no unverified conclusion is presented as proven

  Scenario: Cancel a long-running operation
    Given a symbolic operation is running
    When cancellation is requested
    Then the operation stops
    And the API reports cancellation separately from a mathematical result

  Scenario: Bound deeply nested input
    Given input exceeds the configured syntax nesting limit
    When I parse it
    Then parsing stops safely
    And a resource diagnostic is returned
```

@security
## Feature: Defend Against Malicious Or Pathological Input

```gherkin
Feature: Defend against malicious or pathological input

  Scenario: Reject oversized exact literals by policy
    Given an integer literal exceeds the configured digit budget
    When I parse or evaluate it
    Then the operation stops safely
    And the configured resource limit is reported

  Scenario: Bound proof deserialization
    Given serialized proof data exceeds configured graph limits
    When I deserialize it
    Then deserialization stops safely
    And no partial proof is trusted

  Scenario: Bound rendering recursion
    Given a semantic object exceeds rendering depth limits
    When I render it
    Then rendering stops safely with a diagnostic
```

@diagnostics @slice-1
## Feature: Return Structured Diagnostics

```gherkin
Feature: Return structured diagnostics

  Scenario: Return a parse diagnostic
    When parsing fails
    Then the diagnostic has a stable code
    And it has a severity
    And it includes source spans where available
    And it includes structured parameters suitable for localization

  Scenario: Return a mathematical side-condition diagnostic
    Given a transformation requires a nonzero value
    But nonzeroness is unavailable
    When the transformation is requested
    Then the diagnostic identifies the side condition
    And it is distinguishable from an operational engine failure

  Scenario: Localize a diagnostic without changing its code
    Given a diagnostic with a stable code
    When it is rendered in another language
    Then its message changes language
    But its code and structured parameters remain unchanged
```

---

# Part XIII: Testing Contracts

@testing @slice-1
## Feature: Preserve Core Mathematical Properties

```gherkin
Feature: Preserve core mathematical properties

  Scenario: Canonicalization is idempotent
    Given any supported first-slice semantic object
    When it is canonicalized twice
    Then the second result equals the first canonical result

  Scenario: Simplification preserves its declared relation
    Given any supported first-slice simplification
    When its proof is replayed
    Then the declared equality or solution-set relation is verified

  Scenario: Solver outputs satisfy the original equation
    Given any unique solution returned by the first-slice solver
    When the solution is substituted into the original equation
    Then pointwise evaluation returns true

  Scenario: Complete linear results cover every case
    Given any supported rational linear equation
    When it is solved
    Then the result is exactly one of:
      unique rational solution,
      empty solution set,
      all rational values
    And the completeness status is "Complete"
```

@testing
## Feature: Reject Invalid Evidence

```gherkin
Feature: Reject invalid evidence

  Scenario: Mutate a proof premise
    Given a verified proof
    When one required premise is removed
    Then proof replay fails

  Scenario: Mutate a rule identifier
    Given a verified proof node
    When its rule identifier is replaced by an incompatible rule
    Then proof replay fails

  Scenario: Mutate an exact value
    Given a serialized verified result
    When an exact numeric value in its conclusion is changed
    Then replay or verification fails
```

@testing
## Feature: Fuzz Public Input Boundaries Safely

```gherkin
Feature: Fuzz public input boundaries safely

  Scenario: Fuzz the LaTeX parser
    Given arbitrary input within the fuzzing harness
    When the parser processes it
    Then it either returns a structured parse result or a structured diagnostic
    And it does not panic or access invalid memory

  Scenario: Fuzz public DTO deserialization
    Given arbitrary serialized data
    When the protocol layer processes it
    Then it either returns a valid bounded DTO or a structured rejection
    And no unverified mathematical object becomes trusted
```

---

# Part XIV: Future Domain Capability Contracts

The following features establish long-term behavioral direction. They are not
part of the first vertical slice.

@future @polynomial
## Feature: Compute With Multivariate Polynomials

```gherkin
Feature: Compute with multivariate polynomials

  Scenario: Normalize a polynomial over rationals
    Given a multivariate polynomial expression over rational coefficients
    When I request polynomial normal form
    Then the result is canonical for the declared monomial order
    And equality to the input is certified

  Scenario: Compare polynomials by normal form
    Given two polynomial expressions
    When their verified polynomial normal forms are identical
    Then polynomial equality is proven

  Scenario: Report the coefficient domain and monomial order
    Given any polynomial normalization result
    Then the coefficient domain is explicit
    And the monomial order is explicit
```

@future @inequality
## Feature: Solve Equations And Inequalities With Domain Conditions

```gherkin
Feature: Solve equations and inequalities with domain conditions

  Scenario: Preserve excluded denominator values
    Given a rational equation
    When denominators are cleared
    Then excluded zero-denominator values are recorded
    And final candidates are checked against the original equation

  Scenario: Reverse an inequality when multiplying by a negative value
    Given an ordered-field inequality
    When both sides are multiplied by a proven negative value
    Then the inequality direction is reversed
    And the sign proof is recorded

  Scenario: Return a conditional inequality transformation
    Given the multiplier's sign is unknown
    When multiplication is requested
    Then the result is conditional or split into sign cases
```

@future @functions
## Feature: Treat Functions As First-Class Mathematical Objects

```gherkin
Feature: Treat functions as first-class mathematical objects

  Scenario: Distinguish an expression from the function it defines
    Given the expression "x / x"
    When it is interpreted as a function on rationals
    Then its domain excludes zero
    And it is not identified with the constant-one function on all rationals

  Scenario: Compare functions on a specified domain
    Given two functions and domain "D"
    When I request extensional equality on "D"
    Then the proof quantifies over inputs in "D"
    And domain obligations are explicit
```

@future @calculus
## Feature: Differentiate With Conditions And Proof

```gherkin
Feature: Differentiate with conditions and proof

  Scenario: Differentiate a polynomial
    Given a polynomial function over the reals
    When I differentiate it
    Then the derivative is exact
    And the derivation applies verified differentiation rules

  Scenario: Report differentiability conditions
    Given a piecewise or partially defined function
    When I differentiate it
    Then the result includes its validity domain
    And unresolved differentiability conditions are explicit
```

@future @analysis
## Feature: Reason About Limits And Convergence

```gherkin
Feature: Reason about limits and convergence

  Scenario: Prove a supported limit
    Given a limit problem within a complete decision procedure
    When I compute the limit
    Then the result is proven
    And the domain and approach direction are explicit

  Scenario: Return unknown for an unresolved limit
    Given no active method proves or disproves the limit
    When I compute it
    Then the result is "Unknown"
    And no heuristic guess is presented as verified

  Scenario: Report convergence regions
    Given a power series
    When I analyze convergence
    Then the region or radius of convergence is included
    And boundary cases are treated explicitly
```

@future @complex
## Feature: Respect Complex Branch Conventions

```gherkin
Feature: Respect complex branch conventions

  Scenario: Record the logarithm branch
    Given a complex logarithm expression
    When it is elaborated or transformed
    Then the active branch convention is part of the context

  Scenario: Refuse a branch-dependent identity without context
    Given a proposed complex identity depends on branch choices
    And no branch convention resolves it
    When I compare the expressions
    Then the result is conditional, ambiguous, or unknown
    And the identity is not accepted unconditionally
```

@future @linear-algebra
## Feature: Compute With Linear Maps And Matrices

```gherkin
Feature: Compute with linear maps and matrices

  Scenario: Type-check matrix multiplication
    Given matrices "A" and "B"
    When I form "AB"
    Then multiplication is accepted only when dimensions align
    And the resulting dimensions are inferred

  Scenario: Verify row reduction
    Given a matrix
    When I compute row-reduced echelon form
    Then each elementary row operation is recorded or certified
    And row equivalence is verified

  Scenario: Distinguish matrices from linear maps
    Given a linear map and a choice of bases
    When I obtain its matrix representation
    Then the bases are recorded
    And changing bases does not change the underlying linear map
```

@future @abstract-algebra
## Feature: Reason Through Mathematical Structures

```gherkin
Feature: Reason through mathematical structures

  Scenario: Apply a theorem by required structure
    Given a theorem requires a commutative ring
    And the context supplies a verified commutative-ring instance
    When the theorem is applied
    Then the instance dependency is recorded

  Scenario: Reject a theorem with insufficient structure
    Given a theorem requires a field
    But the context supplies only a ring
    When the theorem is applied
    Then application is rejected
    And the missing structure is reported

  Scenario: Verify a homomorphism
    Given a candidate map between groups
    When I verify that it preserves identity and multiplication
    Then the result is a verified group homomorphism
```

@future @geometry
## Feature: Connect Geometry, Constraints, And Diagrams

```gherkin
Feature: Connect geometry, constraints, and diagrams

  Scenario: Represent a geometric construction semantically
    Given points, lines, and incidence constraints
    When a diagram is rendered
    Then the picture is derived from semantic objects
    And moving the presentation does not alter the geometric facts

  Scenario: Detect a degenerate configuration
    Given a theorem requires three noncollinear points
    When the points are collinear
    Then the theorem is not applied
    And the failed nondegeneracy condition is reported

  Scenario: Change coordinate systems
    Given a geometric object in one coordinate system
    When coordinates are transformed
    Then the underlying geometric object is preserved
    And the coordinate transformation is recorded
```

@future @topology
## Feature: Reason About Topological Structure

```gherkin
Feature: Reason about topological structure

  Scenario: Verify continuity compositionally
    Given verified continuous maps "f" and "g"
    And their domains and codomains compose
    When I form "g ∘ f"
    Then continuity of the composition is proven
    And the typing of the composition is verified

  Scenario: Distinguish equality from homeomorphism
    Given two spaces are homeomorphic but not definitionally identical
    When I compare them by equality
    Then equality is not inferred from homeomorphism
    When I compare them by homeomorphism
    Then the homeomorphism evidence is used
```

@future @category-theory
## Feature: Type-Check Category-Theoretic Composition

```gherkin
Feature: Type-check category-theoretic composition

  Scenario: Compose compatible morphisms
    Given "f : A → B" and "g : B → C"
    When I form "g ∘ f"
    Then the composite has type "A → C"
    And composition is represented by the category's semantic operation

  Scenario: Reject incompatible composition
    Given "f : A → B" and "h : C → D"
    And "B" is not "C"
    When I form "h ∘ f"
    Then elaboration is rejected
    And the domain-codomain mismatch is reported

  Scenario: Normalize associativity and identities
    Given composable morphisms
    When I compare "(h ∘ g) ∘ f" with "h ∘ (g ∘ f)"
    Then equality is proven using category axioms
    And rendering may preserve readable association
```

@future @category-theory
## Feature: Verify Diagram Commutativity

```gherkin
Feature: Verify diagram commutativity

  Scenario: Prove two paths equal
    Given a typed diagram
    And two paths with the same source and target
    When known commuting cells imply the paths are equal
    Then diagram commutativity is proven
    And the path-equality proof is replayable

  Scenario: Reject paths with different endpoints
    Given two diagram paths with different sources or targets
    When I request path equality
    Then the request is rejected as ill typed

  Scenario: Return unknown when commutativity is not established
    Given no available theorem proves or disproves path equality
    When I check the diagram
    Then the result is "Unknown"
```

@future @category-theory
## Feature: Instantiate Universal Properties

```gherkin
Feature: Instantiate universal properties

  Scenario: Use a product universal property
    Given a declared product object with projections
    And compatible morphisms into the factors
    When I request the mediating morphism
    Then the system constructs or identifies the unique candidate
    And verifies the projection equations
    And reports the uniqueness proof

  Scenario: Keep uniqueness explicit
    Given a universal-property result
    Then existence and uniqueness evidence are separately inspectable
```

---

# Part XV: First Vertical Slice Completion

@slice-1
## Feature: Complete The Browser Linear-Equation Workflow

```gherkin
Feature: Complete the browser linear-equation workflow

  Scenario: Execute the complete verified workflow
    Given a browser application using "@socrates/math"
    And the source "3(x - 2) + 4 = 2x + 9"
    When the application parses the source
    And elaborates it with "x : Rational"
    And solves it using "LinearEquationInstructional"
    Then the final verified solution is "x = 11"
    And the completeness status is "Complete"
    And every public step is renderable as KaTeX-compatible LaTeX
    And the derivation can be serialized and replayed
    And generated Wasm classes do not appear in public application types

  Scenario: Validate learner work in the same workflow
    Given the previous equation "3(x - 2) + 4 = 2x + 9"
    When the learner submits "3x - 2 = 2x + 9"
    Then the step is accepted as valid with skipped steps
    When the learner instead submits "3x - 2 = 2x + 8"
    Then the step is rejected with a verified counterexample

  Scenario: Preserve exactness throughout the workflow
    Given any exact integer or rational produced by the workflow
    When it crosses Rust, WebAssembly, serialization, and TypeScript boundaries
    Then its value is preserved exactly
    And it is never implicitly converted to JavaScript floating point
```

## Completion Rule

A `@slice-1` feature is complete only when:

1. the accepted syntax is specified;
2. elaboration produces a well-typed semantic object;
3. all required context is explicit;
4. the operation states its mathematical relation;
5. the result distinguishes proven, disproven, conditional, unknown, and
   undefined outcomes where relevant;
6. authoritative conclusions contain checked evidence;
7. exact values remain exact;
8. rendering is deterministic;
9. public DTOs serialize and deserialize safely;
10. learner-facing explanations are derived from verified mathematics;
11. resource failures cannot become incorrect conclusions;
12. generated WebAssembly types remain private.
