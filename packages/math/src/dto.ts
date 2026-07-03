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
}
