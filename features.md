# Symbolic Math Features

This document stores the behavior-driven feature catalog for Symbolic Math.
Features are written in product-facing language first and are expected to map
to executable tests as implementation begins.

## Feature: Parse Supported LaTeX Into Mathematical Objects

Socrates Academy needs learner and author input to become structured
mathematical data.

Scenario: Parse a linear equation

```gherkin
Given the Socrates Mathematical LaTeX Profile version "0.1"
When I parse the statement "3(x-2)+4 = 2x+9"
Then the result is an equality statement
And the left side is the expression "3(x - 2) + 4"
And the right side is the expression "2x + 9"
And source spans are available for learner-facing feedback
```

Scenario: Reject unsupported LaTeX clearly

```gherkin
Given the Socrates Mathematical LaTeX Profile version "0.1"
When I parse a statement containing an unsupported command
Then parsing fails
And the error identifies the unsupported syntax
And the error includes a source span when available
```

## Feature: Render Mathematical Objects As KaTeX-Compatible LaTeX

Socrates Academy needs deterministic rendering for lessons, feedback, and
stored answer display.

Scenario: Render a parsed equation

```gherkin
Given the equation "3(x-2)+4 = 2x+9"
When I render the equation
Then the output is deterministic KaTeX-compatible LaTeX
And parsing the rendered output produces a semantically equivalent equation
```

## Feature: Simplify Expressions With Recorded Reasons

Socrates Academy needs simplification both for computation and for teaching.

Scenario: Simplify arithmetic inside an expression

```gherkin
Given the expression "3x - 6 + 4"
When I simplify the expression
Then the result is "3x - 2"
And the derivation records a "CombineLikeTerms" transformation
And the transformation preserves expression equality
```

Scenario: Preserve meaningful intermediate structure

```gherkin
Given the expression "3(x - 2) + 4"
When I request instructional simplification
Then one derivation step distributes multiplication over subtraction
And a later derivation step combines constant terms
And the derivation does not collapse directly to the final expression without steps
```

## Feature: Simplify Equations With Solution-Set Preservation

Socrates Academy needs equations to simplify without changing their solutions.

Scenario: Simplify both sides of an equation

```gherkin
Given the equation "3(x - 2) + 4 = 2x + 9"
When I simplify the equation
Then the result is "3x - 2 = 2x + 9"
And each step records the focus location that changed
And each step preserves the same solution set
```

## Feature: Compare Expressions For Equality

Socrates Academy needs to recognize equivalent answers even when they look
different.

Scenario: Compare algebraically equal expressions

```gherkin
Given the expression "3(x - 2) + 4"
And the expression "3x - 2"
When I compare the expressions by normalized expression equality
Then the comparison is equal
And the comparison includes the assumptions used
```

Scenario: Distinguish syntactic equality from semantic equality

```gherkin
Given the expression "x + 1"
And the expression "1 + x"
When I compare the expressions by syntactic equality
Then the comparison is not equal
When I compare the expressions by normalized expression equality
Then the comparison is equal
```

## Feature: Compare Equations By Solution Set

Socrates Academy needs to check learner answers against saved answers without
requiring identical formatting or identical algebraic form.

Scenario: Accept equivalent solved and unsolved equations

```gherkin
Given the equation "x + 1 = 3"
And the equation "x = 2"
When I compare the equations by solution-set equality over rationals
Then the comparison is equal
And the comparison reports that both equations have the solution set "{2}"
```

Scenario: Reject equations with different solution sets

```gherkin
Given the equation "x + 1 = 3"
And the equation "x = 3"
When I compare the equations by solution-set equality over rationals
Then the comparison is not equal
And the comparison reports the differing solution sets when they can be computed
```

## Feature: Evaluate Expressions Exactly

Socrates Academy needs exact evaluation for checking answers and generating
feedback.

Scenario: Evaluate an expression under an assignment

```gherkin
Given the expression "x + 2"
And the assignment "x = 3"
When I evaluate the expression
Then the result is the exact integer "5"
And no JavaScript number conversion is required
```

Scenario: Evaluate a rational expression exactly

```gherkin
Given the expression "\frac{1}{2} + \frac{1}{3}"
When I evaluate the expression
Then the result is the exact rational "5/6"
```

Scenario: Partially evaluate an expression

```gherkin
Given the expression "x + y + 2"
And the assignment "x = 3"
When I partially evaluate the expression
Then the result is "y + 5"
And the unresolved symbol "y" is reported
```

## Feature: Evaluate Statements Exactly

Socrates Academy needs equations and future propositions to evaluate to truth
values under explicit assignments.

Scenario: Evaluate a true equation

```gherkin
Given the equation "x + 2 = 5"
And the assignment "x = 3"
When I evaluate the equation
Then the result is true
```

Scenario: Evaluate a false equation

```gherkin
Given the equation "x + 2 = 6"
And the assignment "x = 3"
When I evaluate the equation
Then the result is false
```

Scenario: Refuse incomplete total evaluation

```gherkin
Given the equation "x + y = 6"
And the assignment "x = 3"
When I evaluate the equation as a total evaluation
Then evaluation fails
And the unresolved symbol "y" is reported
```

## Feature: Solve Single-Variable Linear Equations

Socrates Academy needs exact solving with a derivation that can be shown to a
learner.

Scenario: Solve a linear equation over rationals

```gherkin
Given the equation "3(x - 2) + 4 = 2x + 9"
When I solve for "x" over rationals
Then the solution is "x = 11"
And the derivation is deterministic
And every step records the transformation used
And every step can be independently verified
```

Scenario: Solve an equation with rational solution

```gherkin
Given the equation "2x = 1"
When I solve for "x" over rationals
Then the solution is "x = \frac{1}{2}"
And the exact rational is not converted to a floating point value
```

## Feature: Validate Learner Steps

Socrates Academy needs to distinguish invalid work from valid alternate work.

Scenario: Accept a valid learner simplification step

```gherkin
Given the previous equation "3(x - 2) + 4 = 2x + 9"
When a learner submits "3x - 6 + 4 = 2x + 9"
Then the step is valid
And the recognized transformation is "Distribute"
And the preserved relation is "SameSolutionSet"
```

Scenario: Accept a valid skipped step

```gherkin
Given the previous equation "3(x - 2) + 4 = 2x + 9"
When a learner submits "3x - 2 = 2x + 9"
Then the step is valid
And the validation reports that multiple simplification transformations were skipped
```

Scenario: Reject an invalid learner step

```gherkin
Given the previous equation "3(x - 2) + 4 = 2x + 9"
When a learner submits "3x - 2 = 2x + 8"
Then the step is invalid
And the validation explains that the solution set changed
```

## Feature: Serialize Public Mathematical Results

Socrates Academy needs derivations, answers, and validation results to be stored
and replayed.

Scenario: Serialize and deserialize a solve result

```gherkin
Given a solve result for "3(x - 2) + 4 = 2x + 9"
When I serialize the result to a public DTO
And I deserialize the DTO
Then the mathematical result is unchanged
And every exact number is represented without precision loss
And every derivation step remains replayable
```

## Feature: Keep Generated Wasm Types Out Of Public TypeScript

Socrates Academy applications need a stable API even if the Wasm binding
strategy changes.

Scenario: Public API returns stable DTOs and facade objects

```gherkin
Given a browser application using "@socrates/math"
When it parses, simplifies, solves, evaluates, or validates mathematics
Then it only receives public TypeScript facade objects and versioned DTOs
And it never depends on generated wasm-bindgen classes
```
