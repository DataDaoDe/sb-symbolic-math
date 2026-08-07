export type ExactValueDto =
  | { kind: "integer"; value: string }
  | { kind: "rational"; numerator: string; denominator: string };

export type RealDomainProvenanceDto = "declared" | "natural" | "contextual" | "restricted";
export type RealSetDto =
  | { kind: "empty" }
  | { kind: "all-real" }
  | { kind: "point"; value: ExactValueDto }
  | { kind: "interval"; lower: ExactValueDto; upper: ExactValueDto; lower_inclusive: boolean; upper_inclusive: boolean }
  | { kind: "ray"; direction: string; boundary: ExactValueDto; inclusive: boolean }
  | { kind: "union"; members: RealSetDto[] }
  | { kind: "exclude"; base: RealSetDto; points: ExactValueDto[] }
  | { kind: "set-builder"; source: string };
export interface RealDomainDto { schema: string; version: number; set: RealSetDto; provenance: RealDomainProvenanceDto }
export interface NormalizeRealDomainResponseDto { outcome: MathematicalOutcomeDto; domain: RealDomainDto | null; diagnostics: DiagnosticDto[] }
export interface CompareRealDomainsResponseDto { outcome: MathematicalOutcomeDto; relation: string; equal: boolean | null; left_normalized: RealDomainDto | null; right_normalized: RealDomainDto | null; diagnostics: DiagnosticDto[] }
export interface RealDomainMembershipResponseDto { outcome: MathematicalOutcomeDto; relation: string; contains: boolean | null; normalized_domain: RealDomainDto | null; diagnostics: DiagnosticDto[] }

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

export interface ProtocolCapabilityDto { id: string; version: number }
export interface ProtocolManifestDto { schema: string; version: number; capabilities: ProtocolCapabilityDto[] }
export interface TypedVariableDto { symbol: string; type_id: string }
export interface MathContextDto { theory_ids: string[]; variables: TypedVariableDto[]; assumptions: string[] }
export interface ValidatedMathExpressionDto { schema: string; version: number; source_latex: string; canonical_latex: string; theory: string; context: MathContextDto; value_type: string; free_variables: TypedVariableDto[]; semantic_fingerprint: string }
export interface ValidateMathExpressionResponseDto { outcome: MathematicalOutcomeDto; expression: ValidatedMathExpressionDto | null; diagnostics: DiagnosticDto[] }
export interface UnitDimensionDto { base: string; exponent: ExactValueDto }
export interface UnitDto { schema: string; version: number; dimensions: UnitDimensionDto[]; scale_to_canonical: ExactValueDto; symbol: string }
export interface ExactQuantityDto { value: ExactValueDto; unit: UnitDto | null }
export interface RealFunctionInputSourceDto { variable: string; label: string | null; unit: UnitDto | null }
export interface RealFunctionOutputSourceDto { label: string | null; unit: UnitDto | null }
export interface ExplicitFunctionDefinitionSourceDto { kind: string; expression: string; input_format: string }
export interface RealFunctionSourceDto { schema: string; version: number; input: RealFunctionInputSourceDto; output: RealFunctionOutputSourceDto; definition: ExplicitFunctionDefinitionSourceDto; declared_domain: RealDomainDto | null; declared_codomain: RealDomainDto | null; parameters: string[]; assumptions: string[] }
export interface RealFunctionDto { schema: string; version: number; input: TypedVariableDto; input_label: string | null; input_unit: UnitDto | null; output_type: string; output_label: string | null; output_unit: UnitDto | null; expression: ValidatedMathExpressionDto; natural_domain: RealDomainDto; declared_domain: RealDomainDto | null; effective_domain: RealDomainDto; declared_codomain: RealDomainDto | null; parameters: string[]; assumptions: string[]; semantic_fingerprint: string }
export interface ValidateRealFunctionResponseDto { outcome: MathematicalOutcomeDto; function: RealFunctionDto | null; diagnostics: DiagnosticDto[] }
export interface CompareRealFunctionsResponseDto { outcome: MathematicalOutcomeDto; relation: string; holds: boolean | null; conditions: string[]; left: RealFunctionDto | null; right: RealFunctionDto | null; completeness: string; diagnostics: DiagnosticDto[] }
export interface EvaluateRealFunctionResponseDto { outcome: MathematicalOutcomeDto; value: ExactQuantityDto | null; function: RealFunctionDto | null; completeness: string; diagnostics: DiagnosticDto[] }
export interface RealFunctionTableRowDto { input: ExactQuantityDto; outcome: MathematicalOutcomeDto; output: ExactQuantityDto | null; diagnostics: DiagnosticDto[] }
export interface EvaluateRealFunctionTableResponseDto { outcome: MathematicalOutcomeDto; function: RealFunctionDto | null; rows: RealFunctionTableRowDto[]; completeness: string; diagnostics: DiagnosticDto[] }
export interface AverageRateResponseDto { outcome: MathematicalOutcomeDto; relation: string; value: ExactQuantityDto | null; left_input: ExactQuantityDto; right_input: ExactQuantityDto; function: RealFunctionDto | null; completeness: string; diagnostics: DiagnosticDto[] }
export interface DifferenceQuotientResponseDto { outcome: MathematicalOutcomeDto; relation: string; increment_variable: string; conditions: string[]; initial: MathExpressionDto | null; result: MathExpressionDto | null; result_unit: UnitDto | null; applicable_rules: string[]; steps: MathDerivationStepDto[]; completeness: string; diagnostics: DiagnosticDto[] }
export interface ApplyDifferenceQuotientRuleResponseDto { outcome: MathematicalOutcomeDto; relation: string; rule: string; conditions: string[]; previous: MathExpressionDto | null; result: MathExpressionDto | null; step: MathDerivationStepDto | null; diagnostics: DiagnosticDto[] }

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
  submitted_normalized: ExactValueDto | null;
  expected_normalized: ExactValueDto | null;
  absolute_error: ExactValueDto | null;
  accepted_tolerance: ExactValueDto | null;
  diagnostics: DiagnosticDto[];
}

export interface BaseTenPlaceDto { exponent: number; coefficient: number }
export interface BaseTenDecompositionResponseDto { outcome: MathematicalOutcomeDto; value: ExactValueDto | null; places: BaseTenPlaceDto[]; diagnostics: DiagnosticDto[] }
export interface CompareBaseTenResponseDto { outcome: MathematicalOutcomeDto; relation: string; equal: boolean | null; expected_normalized: ExactValueDto | null; submitted_normalized: ExactValueDto | null; expected_places: BaseTenPlaceDto[]; submitted_places: BaseTenPlaceDto[]; diagnostics: DiagnosticDto[] }

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
export interface ApplyLinearEquationRuleResponseDto {
  outcome: MathematicalOutcomeDto;
  relation: string;
  previous_latex: string | null;
  result_latex: string | null;
  step: MathDerivationStepDto | null;
  diagnostics: DiagnosticDto[];
}
export interface RunLinearEquationStrategyResponseDto {
  outcome: MathematicalOutcomeDto;
  relation: string;
  strategy: string;
  initial_latex: string;
  result_latex: string | null;
  steps: MathDerivationStepDto[];
  diagnostics: DiagnosticDto[];
}

export interface WasmMathEngineBinding {
  decomposeBaseTen?(valueJson: string, minimumExponent: number, maximumExponent: number): string;
  composeBaseTen?(placesJson: string): string;
  compareBaseTen?(expectedJson: string, submittedJson: string, minimumExponent: number, maximumExponent: number): string;
  validateRealFunction?(sourceJson: string): string;
  compareRealFunctions?(leftJson: string, rightJson: string, relation: string): string;
  evaluateRealFunction?(sourceJson: string, inputJson: string): string;
  evaluateRealFunctionTable?(sourceJson: string, inputsJson: string): string;
  averageRate?(sourceJson: string, leftJson: string, rightJson: string): string;
  deriveDifferenceQuotient?(sourceJson: string, incrementVariable: string): string;
  applyDifferenceQuotientRule?(sourceJson: string, incrementVariable: string, rule: string): string;
  normalizeRealDomain?(domainJson: string): string;
  intersectRealDomains?(leftJson: string, rightJson: string): string;
  compareRealDomains?(leftJson: string, rightJson: string): string;
  realDomainContains?(domainJson: string, valueJson: string): string;
  protocolManifest?(): string;
  validateMathExpression?(source: string, inputFormat: string, variable: string): string;
  applyLinearEquationRule?(source: string, variable: string, rule: string, operand?: string | null): string;
  runLinearEquationStrategy?(source: string, variable: string, strategy: string): string;
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
    gradingMode: string,
    absoluteTolerance: string,
    relativeTolerance: string | undefined,
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
