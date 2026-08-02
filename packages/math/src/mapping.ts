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
  ProtocolManifestDto,
  ValidateMathExpressionResponseDto,
  NormalizeRealDomainResponseDto,
  CompareRealDomainsResponseDto,
  RealDomainDto,
  RealDomainMembershipResponseDto,
  RealSetDto,
  RealFunctionSourceDto,
  RealFunctionDto,
  ValidateRealFunctionResponseDto,
  CompareRealFunctionsResponseDto,
  ValidatedMathExpressionDto,
  UnitDto,
  ExactQuantityDto,
  EvaluateRealFunctionResponseDto,
  EvaluateRealFunctionTableResponseDto,
  AverageRateResponseDto,
  DifferenceQuotientResponseDto,
  ApplyDifferenceQuotientRuleResponseDto,
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
  ProtocolManifest,
  ValidateMathExpressionResult,
  NormalizeRealDomainResult,
  CompareRealDomainsResult,
  RealDomain,
  RealDomainMembershipResult,
  RealSet,
  RealFunctionSource,
  RealFunction,
  ValidateRealFunctionResult,
  CompareRealFunctionsResult,
  Unit,
  ExactQuantity,
  EvaluateRealFunctionResult,
  EvaluateRealFunctionTableResult,
  AverageRateResult,
  DifferenceQuotientResult,
  ApplyDifferenceQuotientRuleResult,
} from "./types.js";

export function toRealFunctionSourceDto(source: RealFunctionSource): RealFunctionSourceDto {
  return {
    schema: source.schema,
    version: source.version,
    input: { variable: source.input.variable, label: source.input.label ?? null, unit: source.input.unit ? toUnitDto(source.input.unit) : null },
    output: { label: source.output?.label ?? null, unit: source.output?.unit ? toUnitDto(source.output.unit) : null },
    definition: { kind: source.definition.kind, expression: source.definition.expression, input_format: source.definition.inputFormat },
    declared_domain: source.declaredDomain ? toRealDomainDto(source.declaredDomain) : null,
    declared_codomain: source.declaredCodomain ? toRealDomainDto(source.declaredCodomain) : null,
    parameters: source.parameters ?? [],
    assumptions: source.assumptions ?? [],
  };
}

export function mapValidateRealFunctionResponse(dto: ValidateRealFunctionResponseDto): ValidateRealFunctionResult {
  return { outcome: dto.outcome, function: dto.function ? mapRealFunction(dto.function) : null, diagnostics: dto.diagnostics };
}

export function mapCompareRealFunctionsResponse(dto: CompareRealFunctionsResponseDto): CompareRealFunctionsResult {
  const relation = dto.relation as CompareRealFunctionsResult["relation"];
  return { outcome: dto.outcome, relation, holds: dto.holds, conditions: dto.conditions, left: dto.left ? mapRealFunction(dto.left) : null, right: dto.right ? mapRealFunction(dto.right) : null, completeness: dto.completeness, diagnostics: dto.diagnostics };
}

export function toExactQuantityDto(quantity: ExactQuantity): ExactQuantityDto {
  return { value: quantity.value, unit: quantity.unit ? toUnitDto(quantity.unit) : null };
}

export function mapEvaluateRealFunctionResponse(dto: EvaluateRealFunctionResponseDto): EvaluateRealFunctionResult {
  return { outcome: dto.outcome, value: dto.value ? mapExactQuantity(dto.value) : null, function: dto.function ? mapRealFunction(dto.function) : null, completeness: dto.completeness, diagnostics: dto.diagnostics };
}

export function mapEvaluateRealFunctionTableResponse(dto: EvaluateRealFunctionTableResponseDto): EvaluateRealFunctionTableResult {
  return { outcome: dto.outcome, function: dto.function ? mapRealFunction(dto.function) : null, rows: dto.rows.map(row => ({ input: mapExactQuantity(row.input), outcome: row.outcome, output: row.output ? mapExactQuantity(row.output) : null, diagnostics: row.diagnostics })), completeness: dto.completeness, diagnostics: dto.diagnostics };
}

function mapDerivationStep(step: import("./dto.js").MathDerivationStepDto): import("./types.js").MathDerivationStep {
  return { rule: step.rule, reason: step.reason, target: step.target ? { kind: "whole" } : null, inputLatex: step.input_latex, outputLatex: step.output_latex };
}

export function mapAverageRateResponse(dto: AverageRateResponseDto): AverageRateResult {
  return { outcome: dto.outcome, relation: "function.average-rate", value: dto.value ? mapExactQuantity(dto.value) : null, leftInput: mapExactQuantity(dto.left_input), rightInput: mapExactQuantity(dto.right_input), function: dto.function ? mapRealFunction(dto.function) : null, completeness: dto.completeness, diagnostics: dto.diagnostics };
}

export function mapDifferenceQuotientResponse(dto: DifferenceQuotientResponseDto): DifferenceQuotientResult {
  return { outcome: dto.outcome, relation: "function.difference-quotient", incrementVariable: dto.increment_variable, conditions: dto.conditions, initial: dto.initial, result: dto.result, resultUnit: dto.result_unit ? mapUnit(dto.result_unit) : null, applicableRules: dto.applicable_rules, steps: dto.steps.map(mapDerivationStep), completeness: dto.completeness, diagnostics: dto.diagnostics };
}

export function mapApplyDifferenceQuotientRuleResponse(dto: ApplyDifferenceQuotientRuleResponseDto): ApplyDifferenceQuotientRuleResult {
  return { outcome: dto.outcome, relation: "function.difference-quotient", rule: dto.rule, conditions: dto.conditions, previous: dto.previous, result: dto.result, step: dto.step ? mapDerivationStep(dto.step) : null, diagnostics: dto.diagnostics };
}

function toUnitDto(unit: Unit): UnitDto {
  return { schema: unit.schema, version: unit.version, dimensions: unit.dimensions, scale_to_canonical: unit.scaleToCanonical, symbol: unit.symbol };
}

function mapUnit(dto: UnitDto): Unit {
  return { schema: "socrates.unit", version: 1, dimensions: dto.dimensions.map(dimension => ({ base: dimension.base as Unit["dimensions"][number]["base"], exponent: dimension.exponent })), scaleToCanonical: dto.scale_to_canonical, symbol: dto.symbol };
}

function mapExactQuantity(dto: ExactQuantityDto): ExactQuantity {
  return { value: dto.value, unit: dto.unit ? mapUnit(dto.unit) : null };
}

function mapRealFunction(dto: RealFunctionDto): RealFunction {
  return {
    schema: "socrates.real-function",
    version: 1,
    input: { symbol: dto.input.symbol, typeId: dto.input.type_id },
    inputLabel: dto.input_label,
    inputUnit: dto.input_unit ? mapUnit(dto.input_unit) : null,
    outputType: dto.output_type,
    outputLabel: dto.output_label,
    outputUnit: dto.output_unit ? mapUnit(dto.output_unit) : null,
    expression: mapValidatedExpression(dto.expression),
    naturalDomain: mapRealDomain(dto.natural_domain),
    declaredDomain: dto.declared_domain ? mapRealDomain(dto.declared_domain) : null,
    effectiveDomain: mapRealDomain(dto.effective_domain),
    declaredCodomain: dto.declared_codomain ? mapRealDomain(dto.declared_codomain) : null,
    parameters: dto.parameters,
    assumptions: dto.assumptions,
    semanticFingerprint: dto.semantic_fingerprint,
  };
}

function mapValidatedExpression(dto: ValidatedMathExpressionDto) {
  return {
    schema: "socrates.math.validated-expression" as const,
    version: dto.version,
    sourceLatex: dto.source_latex,
    canonicalLatex: dto.canonical_latex,
    theory: dto.theory,
    context: { theoryIds: dto.context.theory_ids, variables: dto.context.variables.map(variable => ({ symbol: variable.symbol, typeId: variable.type_id })), assumptions: dto.context.assumptions },
    valueType: dto.value_type,
    freeVariables: dto.free_variables.map(variable => ({ symbol: variable.symbol, typeId: variable.type_id })),
    semanticFingerprint: dto.semantic_fingerprint,
  };
}

export function mapNormalizeRealDomainResponse(dto: NormalizeRealDomainResponseDto): NormalizeRealDomainResult {
  return { outcome: dto.outcome, domain: dto.domain ? mapRealDomain(dto.domain) : null, diagnostics: dto.diagnostics };
}

export function mapCompareRealDomainsResponse(dto: CompareRealDomainsResponseDto): CompareRealDomainsResult {
  return { outcome: dto.outcome, relation: "domain.real.equal", equal: dto.equal, leftNormalized: dto.left_normalized ? mapRealDomain(dto.left_normalized) : null, rightNormalized: dto.right_normalized ? mapRealDomain(dto.right_normalized) : null, diagnostics: dto.diagnostics };
}

export function mapRealDomainMembershipResponse(dto: RealDomainMembershipResponseDto): RealDomainMembershipResult {
  return { outcome: dto.outcome, relation: "domain.real.membership", contains: dto.contains, normalizedDomain: dto.normalized_domain ? mapRealDomain(dto.normalized_domain) : null, diagnostics: dto.diagnostics };
}

export function toRealDomainDto(domain: RealDomain): RealDomainDto {
  return { schema: domain.schema, version: domain.version, provenance: domain.provenance, set: toRealSetDto(domain.set) };
}

function mapRealDomain(dto: RealDomainDto): RealDomain {
  return { schema: "socrates.real-domain", version: 1, provenance: dto.provenance, set: mapRealSet(dto.set) };
}

function mapRealSet(dto: RealSetDto): RealSet {
  switch (dto.kind) {
    case "empty": return { kind: "empty" };
    case "all-real": return { kind: "allReal" };
    case "point": return { kind: "point", value: dto.value };
    case "interval": return { kind: "interval", lower: dto.lower, upper: dto.upper, lowerInclusive: dto.lower_inclusive, upperInclusive: dto.upper_inclusive };
    case "ray": return { kind: "ray", direction: dto.direction === "below" ? "below" : "above", boundary: dto.boundary, inclusive: dto.inclusive };
    case "union": return { kind: "union", members: dto.members.map(mapRealSet) };
    case "exclude": return { kind: "exclude", base: mapRealSet(dto.base), points: dto.points };
    case "set-builder": return { kind: "setBuilder", source: dto.source };
  }
}

function toRealSetDto(set: RealSet): RealSetDto {
  switch (set.kind) {
    case "empty": return { kind: "empty" };
    case "allReal": return { kind: "all-real" };
    case "point": return { kind: "point", value: set.value };
    case "interval": return { kind: "interval", lower: set.lower, upper: set.upper, lower_inclusive: set.lowerInclusive, upper_inclusive: set.upperInclusive };
    case "ray": return { kind: "ray", direction: set.direction, boundary: set.boundary, inclusive: set.inclusive };
    case "union": return { kind: "union", members: set.members.map(toRealSetDto) };
    case "exclude": return { kind: "exclude", base: toRealSetDto(set.base), points: set.points };
    case "setBuilder": return { kind: "set-builder", source: set.source };
  }
}

export function mapProtocolManifest(dto: ProtocolManifestDto): ProtocolManifest {
  return { schema: "socrates.math.protocol-manifest", version: dto.version, capabilities: dto.capabilities };
}

export function mapValidateMathExpressionResponse(dto: ValidateMathExpressionResponseDto): ValidateMathExpressionResult {
  return {
    outcome: dto.outcome,
    expression: dto.expression ? {
      schema: "socrates.math.validated-expression",
      version: dto.expression.version,
      sourceLatex: dto.expression.source_latex,
      canonicalLatex: dto.expression.canonical_latex,
      theory: dto.expression.theory,
      context: {
        theoryIds: dto.expression.context.theory_ids,
        variables: dto.expression.context.variables.map(variable => ({ symbol: variable.symbol, typeId: variable.type_id })),
        assumptions: dto.expression.context.assumptions,
      },
      valueType: dto.expression.value_type,
      freeVariables: dto.expression.free_variables.map(variable => ({ symbol: variable.symbol, typeId: variable.type_id })),
      semanticFingerprint: dto.expression.semantic_fingerprint,
    } : null,
    diagnostics: dto.diagnostics,
  };
}

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
    relation: dto.relation === "number.exact_equal"
      ? "number.exact_equal"
      : "number.within_tolerance",
    equal: dto.equal,
    submittedNormalized: dto.submitted_normalized ? mapExactValue(dto.submitted_normalized) : null,
    expectedNormalized: dto.expected_normalized ? mapExactValue(dto.expected_normalized) : null,
    absoluteError: dto.absolute_error ? mapExactValue(dto.absolute_error) : null,
    acceptedTolerance: dto.accepted_tolerance ? mapExactValue(dto.accepted_tolerance) : null,
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
