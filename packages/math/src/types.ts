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

  compareNumericAnswer(
    request: CompareNumericAnswerRequest,
  ): CompareNumericAnswerResult;

  differentiateMathExpression(
    request: TransformMathExpressionRequest,
  ): TransformMathExpressionResult;

  integrateMathExpression(
    request: TransformMathExpressionRequest,
  ): TransformMathExpressionResult;
}
