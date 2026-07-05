import type {
  ApplicableRule,
  ApplyMathExpressionRuleResult,
  CompareEquationSolutionSetsResult,
  CompareMathExpressionsResult,
  CompareNumericAnswerResult,
  CompareSetExpressionsResult,
  EvaluateFiniteRelationPredicateResult,
  EvaluateSetCardinalityResult,
  EvaluateSetStatementResult,
  FiniteRelationProperty,
  MathEngine,
  MathExpressionInputFormat,
  NormalizeMathExpressionResult,
  NormalizeSetExpressionResult,
  RuleTarget,
  SetExpressionInputFormat,
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

export interface SetExpressionAnswerKey {
  type: "set_expression";
  prompt: string;
  inputFormat: SetExpressionInputFormat;
  acceptedExpressions: readonly string[];
}

export interface SetStatementAnswerKey {
  type: "set_statement";
  prompt: string;
  inputFormat: SetExpressionInputFormat;
  expectedTruth: boolean;
}

export interface SetCardinalityAnswerKey {
  type: "set_cardinality";
  prompt: string;
  inputFormat: SetExpressionInputFormat;
  expectedCardinality: number;
}

export interface FiniteRelationPredicateAnswerKey {
  type: "relation" | "function";
  prompt: string;
  inputFormat: SetExpressionInputFormat;
  relationExpression: string;
  domainExpression: string;
  codomainExpression: string;
  expectedTruth: boolean;
}

export interface FiniteRelationPropertyAnswerKey {
  type: "relation_property";
  prompt: string;
  inputFormat: SetExpressionInputFormat;
  relationExpression: string;
  setExpression: string;
  property: FiniteRelationProperty;
  expectedTruth: boolean;
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

export interface SetExpressionDetail {
  normalized: NormalizeSetExpressionResult;
  matchedExpression: string | null;
  lastComparison: CompareSetExpressionsResult | null;
}

export interface SetStatementDetail {
  evaluation: EvaluateSetStatementResult;
}

export interface SetCardinalityDetail {
  evaluation: EvaluateSetCardinalityResult;
}

export interface FiniteRelationPredicateDetail {
  evaluation: EvaluateFiniteRelationPredicateResult;
}

export interface ManualRuleStepRequest {
  expression: string;
  variable: string;
  inputFormat: MathExpressionInputFormat;
  selectedTarget: RuleTarget;
  rule: string;
}

export interface ManualRuleStepPreview {
  legalMoves: ApplicableRule[];
  selectedRule: ApplicableRule | null;
}

export interface ManualRuleStepDetail {
  preview: ManualRuleStepPreview;
  application: ApplyMathExpressionRuleResult;
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

export function previewManualRuleStep(
  engine: MathEngine,
  request: ManualRuleStepRequest,
): ManualRuleStepPreview {
  const legalMoves = engine.listApplicableMathExpressionRules({
    expression: request.expression,
    inputFormat: request.inputFormat,
    variable: request.variable,
    target: request.selectedTarget,
  }).rules;

  return {
    legalMoves,
    selectedRule:
      legalMoves.find(
        (move) => move.rule === request.rule && move.status === "applicable",
      ) ?? null,
  };
}

export function applyManualRuleStep(
  engine: MathEngine,
  request: ManualRuleStepRequest,
): ManualRuleStepDetail {
  const preview = previewManualRuleStep(engine, request);
  const application = engine.applyMathExpressionRule({
    expression: request.expression,
    inputFormat: request.inputFormat,
    variable: request.variable,
    rule: request.rule,
    target: request.selectedTarget,
  });

  return { preview, application };
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

export function gradeSetExpression(
  engine: MathEngine,
  answerKey: SetExpressionAnswerKey,
  response: StudentLatexResponse,
): GradeResult<SetExpressionDetail> {
  const normalized = engine.normalizeSetExpression({
    expression: response.latex,
    inputFormat: answerKey.inputFormat,
  });

  let lastComparison: CompareSetExpressionsResult | null = null;

  for (const acceptedExpression of answerKey.acceptedExpressions) {
    const comparison = engine.compareSetExpressions({
      leftExpression: acceptedExpression,
      rightExpression: response.latex,
      inputFormat: answerKey.inputFormat,
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

export function gradeSetStatement(
  engine: MathEngine,
  answerKey: SetStatementAnswerKey,
  response: StudentLatexResponse,
): GradeResult<SetStatementDetail> {
  const evaluation = engine.evaluateSetStatement({
    statement: response.latex,
    inputFormat: answerKey.inputFormat,
  });

  return {
    correct:
      evaluation.outcome === "proven" &&
      evaluation.truth === answerKey.expectedTruth,
    latencyMs: response.latencyMs,
    detail: { evaluation },
  };
}

export function gradeSetCardinality(
  engine: MathEngine,
  answerKey: SetCardinalityAnswerKey,
  response: StudentLatexResponse,
): GradeResult<SetCardinalityDetail> {
  const evaluation = engine.evaluateSetCardinality({
    expression: response.latex,
    inputFormat: answerKey.inputFormat,
  });

  return {
    correct:
      evaluation.outcome === "proven" &&
      evaluation.cardinality === answerKey.expectedCardinality,
    latencyMs: response.latencyMs,
    detail: { evaluation },
  };
}

export function gradeFiniteRelationPredicate(
  engine: MathEngine,
  answerKey: FiniteRelationPredicateAnswerKey,
): GradeResult<FiniteRelationPredicateDetail> {
  const evaluation =
    answerKey.type === "function"
      ? engine.evaluateFunctionFrom({
          relationExpression: answerKey.relationExpression,
          domainExpression: answerKey.domainExpression,
          codomainExpression: answerKey.codomainExpression,
          inputFormat: answerKey.inputFormat,
        })
      : engine.evaluateRelationFrom({
          relationExpression: answerKey.relationExpression,
          domainExpression: answerKey.domainExpression,
          codomainExpression: answerKey.codomainExpression,
          inputFormat: answerKey.inputFormat,
        });

  return {
    correct:
      evaluation.outcome === "proven" &&
      evaluation.truth === answerKey.expectedTruth,
    latencyMs: 0,
    detail: { evaluation },
  };
}

export function gradeFiniteRelationProperty(
  engine: MathEngine,
  answerKey: FiniteRelationPropertyAnswerKey,
): GradeResult<FiniteRelationPredicateDetail> {
  const evaluation = engine.evaluateRelationProperty({
    relationExpression: answerKey.relationExpression,
    setExpression: answerKey.setExpression,
    property: answerKey.property,
    inputFormat: answerKey.inputFormat,
  });

  return {
    correct:
      evaluation.outcome === "proven" &&
      evaluation.truth === answerKey.expectedTruth,
    latencyMs: 0,
    detail: { evaluation },
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
