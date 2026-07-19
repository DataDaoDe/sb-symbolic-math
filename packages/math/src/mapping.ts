import type {
  ApplicableRuleDto,
  ApplyRuleResponseDto,
  CompareEquationSolutionSetsResponseDto,
  CompareMathExpressionsResponseDto,
  CompareNumericAnswerResponseDto,
  CompareSetExpressionsResponseDto,
  EvaluateFiniteRelationPredicateResponseDto,
  EvaluateSetCardinalityResponseDto,
  EvaluateSetStatementResponseDto,
  ExactValueDto,
  ListApplicableRulesResponseDto,
  NormalizeMathExpressionResponseDto,
  NormalizeSetExpressionResponseDto,
  RuleApplicabilityStatusDto,
  RuleTargetDto,
  SolutionSetDto,
  SolveLinearEquationResponseDto,
  TransformMathExpressionResponseDto,
} from "./dto.js";
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
  ExactValue,
  ListApplicableMathExpressionRulesResult,
  NormalizeMathExpressionResult,
  NormalizeSetExpressionResult,
  RuleApplicabilityStatus,
  RuleTarget,
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

export function mapNormalizeSetExpressionResponse(
  dto: NormalizeSetExpressionResponseDto,
): NormalizeSetExpressionResult {
  return {
    outcome: dto.outcome,
    normalized: dto.normalized,
    diagnostics: dto.diagnostics,
  };
}

export function mapCompareSetExpressionsResponse(
  dto: CompareSetExpressionsResponseDto,
): CompareSetExpressionsResult {
  return {
    outcome: dto.outcome,
    relation:
      dto.relation === "set.extensional_equal.in_context"
        ? "set.extensional_equal.in_context"
        : "set.extensional_equal",
    equal: dto.equal,
    leftNormalized: dto.left_normalized,
    rightNormalized: dto.right_normalized,
    diagnostics: dto.diagnostics,
  };
}

export function mapEvaluateSetStatementResponse(
  dto: EvaluateSetStatementResponseDto,
): EvaluateSetStatementResult {
  return {
    outcome: dto.outcome,
    relation: "logic.truth",
    truth: dto.truth,
    normalized: dto.normalized,
    diagnostics: dto.diagnostics,
  };
}

export function mapEvaluateSetCardinalityResponse(
  dto: EvaluateSetCardinalityResponseDto,
): EvaluateSetCardinalityResult {
  return {
    outcome: dto.outcome,
    relation: "set.cardinality",
    cardinality: dto.cardinality,
    cardinalityLatex: dto.cardinality_latex,
    normalizedSet: dto.normalized_set,
    diagnostics: dto.diagnostics,
  };
}

export function mapEvaluateFiniteRelationPredicateResponse(
  dto: EvaluateFiniteRelationPredicateResponseDto,
): EvaluateFiniteRelationPredicateResult {
  return {
    outcome: dto.outcome,
    relation: mapFiniteRelationPredicateRelation(dto.relation),
    truth: dto.truth,
    normalizedRelation: dto.normalized_relation,
    normalizedDomain: dto.normalized_domain,
    normalizedCodomain: dto.normalized_codomain,
    diagnostics: dto.diagnostics,
  };
}

function mapFiniteRelationPredicateRelation(
  relation: string,
): EvaluateFiniteRelationPredicateResult["relation"] {
  switch (relation) {
    case "function.from":
      return "function.from";
    case "relation.reflexive":
      return "relation.reflexive";
    case "relation.symmetric":
      return "relation.symmetric";
    case "relation.antisymmetric":
      return "relation.antisymmetric";
    case "relation.transitive":
      return "relation.transitive";
    default:
      return "relation.from";
  }
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
      target: step.target ? mapRuleTarget(step.target) : null,
      inputLatex: step.input_latex,
      outputLatex: step.output_latex,
    })),
    diagnostics: dto.diagnostics,
  };
}

export function mapListApplicableRulesResponse(
  dto: ListApplicableRulesResponseDto,
): ListApplicableMathExpressionRulesResult {
  return {
    outcome: dto.outcome,
    rules: dto.rules.map(mapApplicableRule),
    diagnostics: dto.diagnostics,
  };
}

export function mapApplyRuleResponse(
  dto: ApplyRuleResponseDto,
): ApplyMathExpressionRuleResult {
  return {
    outcome: dto.outcome,
    relation: mapRuleApplicationRelation(dto.relation),
    previous: dto.previous,
    result: dto.result,
    step: dto.step
      ? {
          rule: dto.step.rule,
          reason: dto.step.reason,
          target: dto.step.target ? mapRuleTarget(dto.step.target) : null,
          inputLatex: dto.step.input_latex,
          outputLatex: dto.step.output_latex,
        }
      : null,
    diagnostics: dto.diagnostics,
  };
}

function mapRuleApplicationRelation(
  relation: string,
): ApplyMathExpressionRuleResult["relation"] {
  if (relation === "calculus.derivative") {
    return "calculus.derivative";
  }

  if (relation === "calculus.antiderivative") {
    return "calculus.antiderivative";
  }

  return "rule.application";
}

export function toRuleTargetDto(target: RuleTarget): RuleTargetDto {
  switch (target.kind) {
    case "whole":
      return { kind: "whole" };
    case "polynomialTerm":
      return { kind: "polynomial-term", degree: target.degree };
  }
}

function mapApplicableRule(dto: ApplicableRuleDto): ApplicableRule {
  return {
    rule: dto.rule,
    status: mapRuleApplicabilityStatus(dto.status),
    relation: dto.relation,
    target: dto.target ? mapRuleTarget(dto.target) : null,
    reason: dto.reason,
    requiredConditions: dto.required_conditions,
    concepts: dto.concepts,
  };
}

function mapRuleTarget(dto: RuleTargetDto): RuleTarget {
  switch (dto.kind) {
    case "whole":
      return { kind: "whole" };
    case "polynomial-term":
      return { kind: "polynomialTerm", degree: dto.degree };
  }
}

function mapRuleApplicabilityStatus(
  status: RuleApplicabilityStatusDto,
): RuleApplicabilityStatus {
  switch (status) {
    case "applicable":
      return "applicable";
    case "applicable-with-conditions":
      return "applicableWithConditions";
    case "not-applicable":
      return "notApplicable";
    case "ambiguous-target":
      return "ambiguousTarget";
    case "unsupported":
      return "unsupported";
  }
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
