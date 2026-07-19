export type ExactValue =
  | { kind: "integer"; value: string }
  | { kind: "rational"; numerator: string; denominator: string };

export type SolutionSet =
  | { kind: "empty" }
  | { kind: "unique"; value: ExactValue }
  | { kind: "allRationals" };

export type MathematicalOutcome =
  | "proven"
  | "disproven"
  | "conditional"
  | "unknown"
  | "undefined";

export interface MathDiagnostic {
  code: string;
  message: string;
}

export interface SolveLinearEquationRequest {
  equation: string;
  variable: string;
}

export interface SolveLinearEquationResult {
  outcome: MathematicalOutcome;
  variable: string;
  solutionSet: SolutionSet | null;
  solutionSetLatex: string | null;
  completeness: string | null;
  diagnostics: MathDiagnostic[];
}

export interface CompareEquationSolutionSetsRequest {
  leftEquation: string;
  rightEquation: string;
  variable: string;
}

export interface CompareEquationSolutionSetsResult {
  outcome: MathematicalOutcome;
  relation: "equation.same_solution_set";
  equal: boolean | null;
  leftSolutionSet: SolutionSet | null;
  rightSolutionSet: SolutionSet | null;
  leftSolutionSetLatex: string | null;
  rightSolutionSetLatex: string | null;
  diagnostics: MathDiagnostic[];
}

export type MathExpressionInputFormat = "latex";

export interface MathExpression {
  latex: string;
}

export type SetExpressionInputFormat = "latex";

export interface SetExpression {
  latex: string;
}

export interface SetStatement {
  latex: string;
}

export interface NormalizeMathExpressionRequest {
  expression: string;
  inputFormat: MathExpressionInputFormat;
  variable: string;
}

export interface NormalizeMathExpressionResult {
  outcome: MathematicalOutcome;
  normalized: MathExpression | null;
  diagnostics: MathDiagnostic[];
}

export interface CompareMathExpressionsRequest {
  leftExpression: string;
  rightExpression: string;
  inputFormat: MathExpressionInputFormat;
  variable: string;
}

export interface CompareMathExpressionsResult {
  outcome: MathematicalOutcome;
  relation: "expression.equivalent";
  equal: boolean | null;
  leftNormalized: MathExpression | null;
  rightNormalized: MathExpression | null;
  diagnostics: MathDiagnostic[];
}

export interface NormalizeSetExpressionRequest {
  expression: string;
  inputFormat: SetExpressionInputFormat;
}

export interface NormalizeSetExpressionResult {
  outcome: MathematicalOutcome;
  normalized: SetExpression | null;
  diagnostics: MathDiagnostic[];
}

export interface CompareSetExpressionsRequest {
  leftExpression: string;
  rightExpression: string;
  inputFormat: SetExpressionInputFormat;
}

export interface SetBinding {
  symbol: string;
  expression: string;
}

export interface CompareSetExpressionsInContextRequest {
  leftExpression: string;
  rightExpression: string;
  universeExpression: string;
  bindings: readonly SetBinding[];
  inputFormat: SetExpressionInputFormat;
}

export interface CompareSetExpressionsResult {
  outcome: MathematicalOutcome;
  relation: "set.extensional_equal" | "set.extensional_equal.in_context";
  equal: boolean | null;
  leftNormalized: SetExpression | null;
  rightNormalized: SetExpression | null;
  diagnostics: MathDiagnostic[];
}

export interface EvaluateSetStatementRequest {
  statement: string;
  inputFormat: SetExpressionInputFormat;
}

export interface EvaluateSetStatementResult {
  outcome: MathematicalOutcome;
  relation: "logic.truth";
  truth: boolean | null;
  normalized: SetStatement | null;
  diagnostics: MathDiagnostic[];
}

export interface EvaluateSetCardinalityRequest {
  expression: string;
  inputFormat: SetExpressionInputFormat;
}

export interface EvaluateSetCardinalityResult {
  outcome: MathematicalOutcome;
  relation: "set.cardinality";
  cardinality: number | null;
  cardinalityLatex: string | null;
  normalizedSet: SetExpression | null;
  diagnostics: MathDiagnostic[];
}

export interface EvaluateFiniteRelationPredicateRequest {
  relationExpression: string;
  domainExpression: string;
  codomainExpression: string;
  inputFormat: SetExpressionInputFormat;
}

export interface EvaluateFiniteRelationPredicateResult {
  outcome: MathematicalOutcome;
  relation:
    | "relation.from"
    | "function.from"
    | "relation.reflexive"
    | "relation.symmetric"
    | "relation.antisymmetric"
    | "relation.transitive";
  truth: boolean | null;
  normalizedRelation: SetExpression | null;
  normalizedDomain: SetExpression | null;
  normalizedCodomain: SetExpression | null;
  diagnostics: MathDiagnostic[];
}

export type FiniteRelationProperty =
  | "reflexive"
  | "symmetric"
  | "antisymmetric"
  | "transitive";

export interface EvaluateFiniteRelationPropertyRequest {
  relationExpression: string;
  setExpression: string;
  property: FiniteRelationProperty;
  inputFormat: SetExpressionInputFormat;
}

export interface EvaluateFiniteRelationSetOperationRequest {
  relationExpression: string;
  inputFormat: SetExpressionInputFormat;
}

export interface CompareNumericAnswerRequest {
  submitted: string;
  expected: string;
  inputFormat: MathExpressionInputFormat;
  tolerance: number;
}

export interface CompareNumericAnswerResult {
  outcome: MathematicalOutcome;
  relation: "number.within_tolerance";
  equal: boolean | null;
  submittedValue: number | null;
  expectedValue: number | null;
  absoluteError: number | null;
  tolerance: number;
  diagnostics: MathDiagnostic[];
}

export interface MathDerivationStep {
  rule: string;
  reason: string;
  target: RuleTarget | null;
  inputLatex: string | null;
  outputLatex: string | null;
}

export interface TransformMathExpressionRequest {
  expression: string;
  inputFormat: MathExpressionInputFormat;
  variable: string;
}

export interface TransformMathExpressionResult {
  outcome: MathematicalOutcome;
  relation: "calculus.derivative" | "calculus.antiderivative";
  result: MathExpression | null;
  steps: MathDerivationStep[];
  diagnostics: MathDiagnostic[];
}

export type RuleTarget =
  | { kind: "whole" }
  | { kind: "polynomialTerm"; degree: number };

export type RuleApplicabilityStatus =
  | "applicable"
  | "applicableWithConditions"
  | "notApplicable"
  | "ambiguousTarget"
  | "unsupported";

export interface ApplicableRule {
  rule: string;
  status: RuleApplicabilityStatus;
  relation: string;
  target: RuleTarget | null;
  reason: string;
  requiredConditions: string[];
  concepts: string[];
}

export interface ListApplicableMathExpressionRulesRequest {
  expression: string;
  inputFormat: MathExpressionInputFormat;
  variable: string;
  target?: RuleTarget | null;
}

export interface ListApplicableMathExpressionRulesResult {
  outcome: MathematicalOutcome;
  rules: ApplicableRule[];
  diagnostics: MathDiagnostic[];
}

export interface ApplyMathExpressionRuleRequest {
  expression: string;
  inputFormat: MathExpressionInputFormat;
  variable: string;
  rule: string;
  target?: RuleTarget | null;
}

export interface ApplyMathExpressionRuleResult {
  outcome: MathematicalOutcome;
  relation: "calculus.derivative" | "calculus.antiderivative" | "rule.application";
  previous: MathExpression | null;
  result: MathExpression | null;
  step: MathDerivationStep | null;
  diagnostics: MathDiagnostic[];
}

export interface MathEngine {
  solveLinearEquation(
    request: SolveLinearEquationRequest,
  ): SolveLinearEquationResult;

  compareEquationSolutionSets(
    request: CompareEquationSolutionSetsRequest,
  ): CompareEquationSolutionSetsResult;

  normalizeMathExpression(
    request: NormalizeMathExpressionRequest,
  ): NormalizeMathExpressionResult;

  compareMathExpressions(
    request: CompareMathExpressionsRequest,
  ): CompareMathExpressionsResult;

  normalizeSetExpression(
    request: NormalizeSetExpressionRequest,
  ): NormalizeSetExpressionResult;

  compareSetExpressions(
    request: CompareSetExpressionsRequest,
  ): CompareSetExpressionsResult;

  compareSetExpressionsInContext(
    request: CompareSetExpressionsInContextRequest,
  ): CompareSetExpressionsResult;

  evaluateSetStatement(
    request: EvaluateSetStatementRequest,
  ): EvaluateSetStatementResult;

  evaluateSetCardinality(
    request: EvaluateSetCardinalityRequest,
  ): EvaluateSetCardinalityResult;

  evaluateRelationFrom(
    request: EvaluateFiniteRelationPredicateRequest,
  ): EvaluateFiniteRelationPredicateResult;

  evaluateFunctionFrom(
    request: EvaluateFiniteRelationPredicateRequest,
  ): EvaluateFiniteRelationPredicateResult;

  evaluateRelationProperty(
    request: EvaluateFiniteRelationPropertyRequest,
  ): EvaluateFiniteRelationPredicateResult;

  evaluateRelationDomain(
    request: EvaluateFiniteRelationSetOperationRequest,
  ): NormalizeSetExpressionResult;

  evaluateRelationRange(
    request: EvaluateFiniteRelationSetOperationRequest,
  ): NormalizeSetExpressionResult;

  evaluateRelationInverse(
    request: EvaluateFiniteRelationSetOperationRequest,
  ): NormalizeSetExpressionResult;

  compareNumericAnswer(
    request: CompareNumericAnswerRequest,
  ): CompareNumericAnswerResult;

  differentiateMathExpression(
    request: TransformMathExpressionRequest,
  ): TransformMathExpressionResult;

  integrateMathExpression(
    request: TransformMathExpressionRequest,
  ): TransformMathExpressionResult;

  listApplicableMathExpressionRules(
    request: ListApplicableMathExpressionRulesRequest,
  ): ListApplicableMathExpressionRulesResult;

  applyMathExpressionRule(
    request: ApplyMathExpressionRuleRequest,
  ): ApplyMathExpressionRuleResult;
}
