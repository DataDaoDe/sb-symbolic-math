import type {
  CompareEquationSolutionSetsResponseDto,
  CompareMathExpressionsResponseDto,
  CompareNumericAnswerResponseDto,
  ExactValueDto,
  NormalizeMathExpressionResponseDto,
  SolutionSetDto,
  SolveLinearEquationResponseDto,
  TransformMathExpressionResponseDto,
} from "./dto.js";
import type {
  CompareEquationSolutionSetsResult,
  CompareMathExpressionsResult,
  CompareNumericAnswerResult,
  ExactValue,
  NormalizeMathExpressionResult,
  SolutionSet,
  SolveLinearEquationResult,
  TransformMathExpressionResult,
} from "./types.js";

export function mapSolveLinearEquationResponse(
  dto: SolveLinearEquationResponseDto,
): SolveLinearEquationResult {
  return {
    outcome: dto.outcome,
    variable: dto.variable,
    solutionSet: dto.solution_set ? mapSolutionSet(dto.solution_set) : null,
    solutionSetLatex: dto.solution_set_latex,
    completeness: dto.completeness,
    diagnostics: dto.diagnostics,
  };
}

export function mapCompareEquationSolutionSetsResponse(
  dto: CompareEquationSolutionSetsResponseDto,
): CompareEquationSolutionSetsResult {
  return {
    outcome: dto.outcome,
    relation: "equation.same_solution_set",
    equal: dto.equal,
    leftSolutionSet: dto.left_solution_set
      ? mapSolutionSet(dto.left_solution_set)
      : null,
    rightSolutionSet: dto.right_solution_set
      ? mapSolutionSet(dto.right_solution_set)
      : null,
    leftSolutionSetLatex: dto.left_solution_set_latex,
    rightSolutionSetLatex: dto.right_solution_set_latex,
    diagnostics: dto.diagnostics,
  };
}

export function mapNormalizeMathExpressionResponse(
  dto: NormalizeMathExpressionResponseDto,
): NormalizeMathExpressionResult {
  return {
    outcome: dto.outcome,
    normalized: dto.normalized,
    diagnostics: dto.diagnostics,
  };
}

export function mapCompareMathExpressionsResponse(
  dto: CompareMathExpressionsResponseDto,
): CompareMathExpressionsResult {
  return {
    outcome: dto.outcome,
    relation: "expression.equivalent",
    equal: dto.equal,
    leftNormalized: dto.left_normalized,
    rightNormalized: dto.right_normalized,
    diagnostics: dto.diagnostics,
  };
}

export function mapCompareNumericAnswerResponse(
  dto: CompareNumericAnswerResponseDto,
): CompareNumericAnswerResult {
  return {
    outcome: dto.outcome,
    relation: "number.within_tolerance",
    equal: dto.equal,
    submittedValue: dto.submitted_value,
    expectedValue: dto.expected_value,
    absoluteError: dto.absolute_error,
    tolerance: dto.tolerance,
    diagnostics: dto.diagnostics,
  };
}

export function mapTransformMathExpressionResponse(
  dto: TransformMathExpressionResponseDto,
): TransformMathExpressionResult {
  return {
    outcome: dto.outcome,
    relation:
      dto.relation === "calculus.derivative"
        ? "calculus.derivative"
        : "calculus.antiderivative",
    result: dto.result,
    steps: dto.steps.map((step) => ({
      rule: step.rule,
      reason: step.reason,
      inputLatex: step.input_latex,
      outputLatex: step.output_latex,
    })),
    diagnostics: dto.diagnostics,
  };
}

function mapSolutionSet(dto: SolutionSetDto): SolutionSet {
  switch (dto.kind) {
    case "empty":
      return { kind: "empty" };
    case "unique":
      return { kind: "unique", value: mapExactValue(dto.value) };
    case "all-rationals":
      return { kind: "allRationals" };
  }
}

function mapExactValue(dto: ExactValueDto): ExactValue {
  switch (dto.kind) {
    case "integer":
      return { kind: "integer", value: dto.value };
    case "rational":
      return {
        kind: "rational",
        numerator: dto.numerator,
        denominator: dto.denominator,
      };
  }
}
