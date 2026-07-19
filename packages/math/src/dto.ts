export type ExactValueDto =
  | { kind: "integer"; value: string }
  | { kind: "rational"; numerator: string; denominator: string };

export type SolutionSetDto =
  | { kind: "empty" }
  | { kind: "unique"; value: ExactValueDto }
  | { kind: "all-rationals" };

export type MathematicalOutcomeDto =
  | "proven"
  | "disproven"
  | "conditional"
  | "unknown"
  | "undefined";

export interface DiagnosticDto {
  code: string;
  message: string;
}

export interface SolveLinearEquationResponseDto {
  outcome: MathematicalOutcomeDto;
  variable: string;
  solution_set: SolutionSetDto | null;
  solution_set_latex: string | null;
  completeness: string | null;
  diagnostics: DiagnosticDto[];
}

export interface CompareEquationSolutionSetsResponseDto {
  outcome: MathematicalOutcomeDto;
  relation: string;
  equal: boolean | null;
  left_solution_set: SolutionSetDto | null;
  right_solution_set: SolutionSetDto | null;
  left_solution_set_latex: string | null;
  right_solution_set_latex: string | null;
  diagnostics: DiagnosticDto[];
}

export interface MathExpressionDto {
  latex: string;
}

export interface SetExpressionDto {
  latex: string;
}

export interface SetStatementDto {
  latex: string;
}

export interface NormalizeMathExpressionResponseDto {
  outcome: MathematicalOutcomeDto;
  normalized: MathExpressionDto | null;
  diagnostics: DiagnosticDto[];
}

export interface CompareMathExpressionsResponseDto {
  outcome: MathematicalOutcomeDto;
  relation: string;
  equal: boolean | null;
  left_normalized: MathExpressionDto | null;
  right_normalized: MathExpressionDto | null;
  diagnostics: DiagnosticDto[];
}

export interface NormalizeSetExpressionResponseDto {
  outcome: MathematicalOutcomeDto;
  normalized: SetExpressionDto | null;
  diagnostics: DiagnosticDto[];
}

export interface CompareSetExpressionsResponseDto {
  outcome: MathematicalOutcomeDto;
  relation: string;
  equal: boolean | null;
  left_normalized: SetExpressionDto | null;
  right_normalized: SetExpressionDto | null;
  diagnostics: DiagnosticDto[];
}

export interface SetBindingDto {
  symbol: string;
  expression: string;
}

export interface EvaluateSetStatementResponseDto {
  outcome: MathematicalOutcomeDto;
  relation: string;
  truth: boolean | null;
  normalized: SetStatementDto | null;
  diagnostics: DiagnosticDto[];
}

export interface EvaluateSetCardinalityResponseDto {
  outcome: MathematicalOutcomeDto;
  relation: string;
  cardinality: number | null;
  cardinality_latex: string | null;
  normalized_set: SetExpressionDto | null;
  diagnostics: DiagnosticDto[];
}

export interface EvaluateFiniteRelationPredicateResponseDto {
  outcome: MathematicalOutcomeDto;
  relation: string;
  truth: boolean | null;
  normalized_relation: SetExpressionDto | null;
  normalized_domain: SetExpressionDto | null;
  normalized_codomain: SetExpressionDto | null;
  diagnostics: DiagnosticDto[];
}

export interface CompareNumericAnswerResponseDto {
  outcome: MathematicalOutcomeDto;
  relation: string;
  equal: boolean | null;
  submitted_value: number | null;
  expected_value: number | null;
  absolute_error: number | null;
  tolerance: number;
  diagnostics: DiagnosticDto[];
}

export interface MathDerivationStepDto {
  rule: string;
  reason: string;
  target: RuleTargetDto | null;
  input_latex: string | null;
  output_latex: string | null;
}

export interface TransformMathExpressionResponseDto {
  outcome: MathematicalOutcomeDto;
  relation: string;
  result: MathExpressionDto | null;
  steps: MathDerivationStepDto[];
  diagnostics: DiagnosticDto[];
}

export type RuleTargetDto =
  | { kind: "whole" }
  | { kind: "polynomial-term"; degree: number };

export type RuleApplicabilityStatusDto =
  | "applicable"
  | "applicable-with-conditions"
  | "not-applicable"
  | "ambiguous-target"
  | "unsupported";

export interface ApplicableRuleDto {
  rule: string;
  status: RuleApplicabilityStatusDto;
  relation: string;
  target: RuleTargetDto | null;
  reason: string;
  required_conditions: string[];
  concepts: string[];
}

export interface ListApplicableRulesResponseDto {
  outcome: MathematicalOutcomeDto;
  rules: ApplicableRuleDto[];
  diagnostics: DiagnosticDto[];
}

export interface ApplyRuleResponseDto {
  outcome: MathematicalOutcomeDto;
  relation: string;
  previous: MathExpressionDto | null;
  result: MathExpressionDto | null;
  step: MathDerivationStepDto | null;
  diagnostics: DiagnosticDto[];
}

export interface WasmMathEngineBinding {
  solveLinearEquation(source: string, variable: string): string;
  compareEquationSolutionSets(
    leftSource: string,
    rightSource: string,
    variable: string,
  ): string;
  normalizeMathExpression?(
    source: string,
    inputFormat: string,
    variable: string,
  ): string;
  compareMathExpressions?(
    leftSource: string,
    rightSource: string,
    inputFormat: string,
    variable: string,
  ): string;
  normalizeSetExpression?(source: string, inputFormat: string): string;
  compareSetExpressions?(
    leftSource: string,
    rightSource: string,
    inputFormat: string,
  ): string;
  compareSetExpressionsInContext?(
    leftSource: string,
    rightSource: string,
    universeSource: string,
    bindingsJson: string,
    inputFormat: string,
  ): string;
  evaluateSetStatement?(source: string, inputFormat: string): string;
  evaluateSetCardinality?(source: string, inputFormat: string): string;
  evaluateRelationFrom?(
    relationSource: string,
    domainSource: string,
    codomainSource: string,
    inputFormat: string,
  ): string;
  evaluateFunctionFrom?(
    relationSource: string,
    domainSource: string,
    codomainSource: string,
    inputFormat: string,
  ): string;
  evaluateRelationProperty?(
    relationSource: string,
    setSource: string,
    property: string,
    inputFormat: string,
  ): string;
  evaluateRelationDomain?(relationSource: string, inputFormat: string): string;
  evaluateRelationRange?(relationSource: string, inputFormat: string): string;
  evaluateRelationInverse?(relationSource: string, inputFormat: string): string;
  compareNumericAnswer?(
    submittedSource: string,
    expectedSource: string,
    inputFormat: string,
    tolerance: number,
  ): string;
  differentiateMathExpression?(
    source: string,
    inputFormat: string,
    variable: string,
  ): string;
  integrateMathExpression?(
    source: string,
    inputFormat: string,
    variable: string,
  ): string;
  listApplicableMathExpressionRules?(
    source: string,
    inputFormat: string,
    variable: string,
    targetJson: string | null,
  ): string;
  applyMathExpressionRule?(
    source: string,
    inputFormat: string,
    variable: string,
    rule: string,
    targetJson: string | null,
  ): string;
}
