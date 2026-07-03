import type {
  CompareEquationSolutionSetsResult,
  CompareMathExpressionsResult,
  CompareNumericAnswerResult,
  MathEngine,
  MathExpressionInputFormat,
  NormalizeMathExpressionResult,
} from "@socrates/math";

export interface MathExpressionAnswerKey {
  type: "math_expression";
  prompt: string;
  variable: string;
  inputFormat: MathExpressionInputFormat;
  acceptedExpressions: readonly string[];
}

export interface MathEquationAnswerKey {
  type: "math_equation";
  prompt: string;
  variable: string;
  acceptedEquation: string;
}

export interface NumericAnswerKey {
  type: "numeric_answer";
  prompt: string;
  inputFormat: MathExpressionInputFormat;
  expected: string;
  tolerance: number;
}

export interface StudentLatexResponse {
  latex: string;
  latencyMs: number;
}

export interface GradeResult<TDetail> {
  correct: boolean;
  latencyMs: number;
  detail: TDetail;
}

export interface MathExpressionDetail {
  normalized: NormalizeMathExpressionResult;
  matchedExpression: string | null;
  lastComparison: CompareMathExpressionsResult | null;
}

export interface MathEquationDetail {
  comparison: CompareEquationSolutionSetsResult;
}

export interface NumericAnswerDetail {
  comparison: CompareNumericAnswerResult;
}

export function gradeMathExpression(
  engine: MathEngine,
  answerKey: MathExpressionAnswerKey,
  response: StudentLatexResponse,
): GradeResult<MathExpressionDetail> {
  const normalized = engine.normalizeMathExpression({
    expression: response.latex,
    inputFormat: answerKey.inputFormat,
    variable: answerKey.variable,
  });

  let lastComparison: CompareMathExpressionsResult | null = null;

  for (const acceptedExpression of answerKey.acceptedExpressions) {
    const comparison = engine.compareMathExpressions({
      leftExpression: acceptedExpression,
      rightExpression: response.latex,
      inputFormat: answerKey.inputFormat,
      variable: answerKey.variable,
    });

    lastComparison = comparison;

    if (comparison.outcome === "proven" && comparison.equal === true) {
      return {
        correct: true,
        latencyMs: response.latencyMs,
        detail: {
          normalized,
          matchedExpression: acceptedExpression,
          lastComparison,
        },
      };
    }
  }

  return {
    correct: false,
    latencyMs: response.latencyMs,
    detail: {
      normalized,
      matchedExpression: null,
      lastComparison,
    },
  };
}

export function gradeMathEquation(
  engine: MathEngine,
  answerKey: MathEquationAnswerKey,
  response: StudentLatexResponse,
): GradeResult<MathEquationDetail> {
  const comparison = engine.compareEquationSolutionSets({
    leftEquation: answerKey.acceptedEquation,
    rightEquation: response.latex,
    variable: answerKey.variable,
  });

  return {
    correct: comparison.outcome === "proven" && comparison.equal === true,
    latencyMs: response.latencyMs,
    detail: { comparison },
  };
}

export function gradeNumericAnswer(
  engine: MathEngine,
  answerKey: NumericAnswerKey,
  response: StudentLatexResponse,
): GradeResult<NumericAnswerDetail> {
  const comparison = engine.compareNumericAnswer({
    submitted: response.latex,
    expected: answerKey.expected,
    inputFormat: answerKey.inputFormat,
    tolerance: answerKey.tolerance,
  });

  return {
    correct: comparison.outcome === "proven" && comparison.equal === true,
    latencyMs: response.latencyMs,
    detail: { comparison },
  };
}
