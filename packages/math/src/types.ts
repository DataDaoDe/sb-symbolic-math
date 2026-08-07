export type ExactValue =
  | { kind: "integer"; value: string }
  | { kind: "rational"; numerator: string; denominator: string };

export type RealDomainProvenance = "declared" | "natural" | "contextual" | "restricted";
export type RealSet =
  | { kind: "empty" }
  | { kind: "allReal" }
  | { kind: "point"; value: ExactValue }
  | { kind: "interval"; lower: ExactValue; upper: ExactValue; lowerInclusive: boolean; upperInclusive: boolean }
  | { kind: "ray"; direction: "below" | "above"; boundary: ExactValue; inclusive: boolean }
  | { kind: "union"; members: RealSet[] }
  | { kind: "exclude"; base: RealSet; points: ExactValue[] }
  | { kind: "setBuilder"; source: string };
export interface RealDomain { schema: "socrates.real-domain"; version: 1; set: RealSet; provenance: RealDomainProvenance }
export interface NormalizeRealDomainRequest { domain: RealDomain }
export interface NormalizeRealDomainResult { outcome: MathematicalOutcome; domain: RealDomain | null; diagnostics: MathDiagnostic[] }
export interface IntersectRealDomainsRequest { left: RealDomain; right: RealDomain }
export interface CompareRealDomainsRequest { left: RealDomain; right: RealDomain }
export interface CompareRealDomainsResult { outcome: MathematicalOutcome; relation: "domain.real.equal"; equal: boolean | null; leftNormalized: RealDomain | null; rightNormalized: RealDomain | null; diagnostics: MathDiagnostic[] }
export interface RealDomainContainsRequest { domain: RealDomain; value: ExactValue }
export interface RealDomainMembershipResult { outcome: MathematicalOutcome; relation: "domain.real.membership"; contains: boolean | null; normalizedDomain: RealDomain | null; diagnostics: MathDiagnostic[] }

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

export interface ProtocolCapability { id: string; version: number }
export interface ProtocolManifest { schema: "socrates.math.protocol-manifest"; version: number; capabilities: ProtocolCapability[] }
export interface TypedVariable { symbol: string; typeId: string }
export interface MathContext { theoryIds: string[]; variables: TypedVariable[]; assumptions: string[] }
export interface ValidatedMathExpression { schema: "socrates.math.validated-expression"; version: number; sourceLatex: string; canonicalLatex: string; theory: string; context: MathContext; valueType: string; freeVariables: TypedVariable[]; semanticFingerprint: string }
export interface ValidateMathExpressionRequest { expression: string; inputFormat: MathExpressionInputFormat; variable: string }
export interface ValidateMathExpressionResult { outcome: MathematicalOutcome; expression: ValidatedMathExpression | null; diagnostics: MathDiagnostic[] }
export type UnitBaseDimension = "length" | "mass" | "time" | "electric_current" | "temperature" | "amount" | "luminous_intensity" | "angle";
export interface Unit { schema: "socrates.unit"; version: 1; dimensions: Array<{ base: UnitBaseDimension; exponent: ExactValue }>; scaleToCanonical: ExactValue; symbol: string }
export interface ExactQuantity { value: ExactValue; unit: Unit | null }
export interface RealFunctionSource { schema: "socrates.real-function"; version: 1; input: { variable: string; label?: string; unit?: Unit }; output?: { label?: string; unit?: Unit }; definition: { kind: "explicit"; expression: string; inputFormat: "latex" }; declaredDomain?: RealDomain; declaredCodomain?: RealDomain; parameters?: string[]; assumptions?: string[] }
export interface RealFunction { schema: "socrates.real-function"; version: 1; input: TypedVariable; inputLabel: string | null; inputUnit: Unit | null; outputType: string; outputLabel: string | null; outputUnit: Unit | null; expression: ValidatedMathExpression; naturalDomain: RealDomain; declaredDomain: RealDomain | null; effectiveDomain: RealDomain; declaredCodomain: RealDomain | null; parameters: string[]; assumptions: string[]; semanticFingerprint: string }
export interface ValidateRealFunctionRequest { source: RealFunctionSource }
export interface ValidateRealFunctionResult { outcome: MathematicalOutcome; function: RealFunction | null; diagnostics: MathDiagnostic[] }
export type RealFunctionRelation = "function.equal" | "function.formula_equal_on_intersection" | "function.restriction_of" | "function.extension_of";
export interface CompareRealFunctionsRequest { left: RealFunctionSource; right: RealFunctionSource; relation: RealFunctionRelation }
export interface CompareRealFunctionsResult { outcome: MathematicalOutcome; relation: RealFunctionRelation; holds: boolean | null; conditions: string[]; left: RealFunction | null; right: RealFunction | null; completeness: string; diagnostics: MathDiagnostic[] }
export interface EvaluateRealFunctionRequest { source: RealFunctionSource; input: ExactQuantity }
export interface EvaluateRealFunctionResult { outcome: MathematicalOutcome; value: ExactQuantity | null; function: RealFunction | null; completeness: string; diagnostics: MathDiagnostic[] }
export interface EvaluateRealFunctionTableRequest { source: RealFunctionSource; inputs: readonly ExactQuantity[] }
export interface RealFunctionTableRow { input: ExactQuantity; outcome: MathematicalOutcome; output: ExactQuantity | null; diagnostics: MathDiagnostic[] }
export interface EvaluateRealFunctionTableResult { outcome: MathematicalOutcome; function: RealFunction | null; rows: RealFunctionTableRow[]; completeness: string; diagnostics: MathDiagnostic[] }
export interface AverageRateRequest { source: RealFunctionSource; leftInput: ExactQuantity; rightInput: ExactQuantity }
export interface AverageRateResult { outcome: MathematicalOutcome; relation: "function.average-rate"; value: ExactQuantity | null; leftInput: ExactQuantity; rightInput: ExactQuantity; function: RealFunction | null; completeness: string; diagnostics: MathDiagnostic[] }
export interface DifferenceQuotientRequest { source: RealFunctionSource; incrementVariable: string }
export interface DifferenceQuotientResult { outcome: MathematicalOutcome; relation: "function.difference-quotient"; incrementVariable: string; conditions: string[]; initial: MathExpression | null; result: MathExpression | null; resultUnit: Unit | null; applicableRules: string[]; steps: MathDerivationStep[]; completeness: string; diagnostics: MathDiagnostic[] }
export interface ApplyDifferenceQuotientRuleRequest extends DifferenceQuotientRequest { rule: string }
export interface ApplyDifferenceQuotientRuleResult { outcome: MathematicalOutcome; relation: "function.difference-quotient"; rule: string; conditions: string[]; previous: MathExpression | null; result: MathExpression | null; step: MathDerivationStep | null; diagnostics: MathDiagnostic[] }

export type RealFunctionGraphRefinement =
  | "initial"
  | "interactive"
  | "settled"
  | "export";

export type RealFunctionGraphSegmentStatus =
  | "certified_continuous"
  | "discontinuity_boundary"
  | "uncertain";

export interface SampleRealFunctionGraphRequest {
  source: RealFunctionSource;
  visibleInput: { min: number; max: number };
  viewportPixels: { width: number; height: number };
  visibleOutput: { min: number; max: number };
  targetScreenErrorPx: number;
  maxSamples: number;
  refinement: RealFunctionGraphRefinement;
}

export interface RealFunctionGraphSampleSegment {
  status: RealFunctionGraphSegmentStatus;
  points: readonly number[];
  inputEnclosures: readonly number[] | null;
  outputEnclosures: readonly number[] | null;
  errorBound: number | null;
  diagnostics: MathDiagnostic[];
}

export interface SampleRealFunctionGraphResult {
  outcome: MathematicalOutcome;
  function: RealFunction | null;
  segments: RealFunctionGraphSampleSegment[];
  completeness: string;
  diagnostics: MathDiagnostic[];
}

export type RealFunctionGraphFeatureKind =
  | "evaluated_point"
  | "x_intercept"
  | "y_intercept"
  | "excluded_point"
  | "hole"
  | "pole"
  | "intersection";

export interface QueryRealFunctionGraphFeaturesRequest {
  source: RealFunctionSource;
  kinds: readonly RealFunctionGraphFeatureKind[];
  visibleInput?: { min: ExactValue; max: ExactValue };
  otherFunction?: RealFunctionSource;
}

export interface RealFunctionGraphFeature {
  id: string;
  kind: RealFunctionGraphFeatureKind;
  x: ExactValue;
  y: ExactValue | null;
  conditions: string[];
}

export interface QueryRealFunctionGraphFeaturesResult {
  outcome: MathematicalOutcome;
  function: RealFunction | null;
  features: RealFunctionGraphFeature[];
  completeness: string;
  diagnostics: MathDiagnostic[];
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

export type AuthoredLinearSolutionSet =
  | { kind: "empty" }
  | { kind: "all_reals" }
  | { kind: "point"; value: string }
  | { kind: "ray"; direction: "below" | "above"; boundary: string; inclusive: boolean }
  | { kind: "interval"; lower: string; upper: string; lower_inclusive: boolean; upper_inclusive: boolean };

export interface CompareAuthoredLinearSolutionSetsRequest {
  left: AuthoredLinearSolutionSet;
  right: AuthoredLinearSolutionSet;
}

export interface CompareAuthoredLinearSolutionSetsResult {
  outcome: MathematicalOutcome;
  relation: "linear_solution_set.equal";
  equal: boolean | null;
  diagnostics: MathDiagnostic[];
}

export interface ValidatePolynomialDerivationRequest {
  initialExpression: string;
  submittedSteps: readonly string[];
  goalExpression: string;
  inputFormat: "latex";
  variable: string;
}

export interface PolynomialDerivationStepResult {
  input: string;
  output: string;
  outcome: MathematicalOutcome;
  equivalent: boolean | null;
  inputNormalized: MathExpression | null;
  outputNormalized: MathExpression | null;
  diagnostics: MathDiagnostic[];
}

export interface ValidatePolynomialDerivationResult {
  outcome: MathematicalOutcome;
  relation: "derivation.polynomial_identity";
  valid: boolean | null;
  steps: PolynomialDerivationStepResult[];
  reachesGoal: boolean | null;
  diagnostics: MathDiagnostic[];
}
export type LinearEquationRule =
  | "algebra.linear-equation.simplify-both-sides"
  | "algebra.equation.add-both-sides"
  | "algebra.equation.subtract-both-sides"
  | "algebra.equation.multiply-both-sides"
  | "algebra.equation.divide-both-sides";
export interface ApplyLinearEquationRuleRequest { equation: string; variable: string; rule: LinearEquationRule; operand: string | null }
export interface ApplyLinearEquationRuleResult { outcome: MathematicalOutcome; relation: "rule.application"; previousLatex: string | null; resultLatex: string | null; step: MathDerivationStep | null; diagnostics: MathDiagnostic[] }
export type LinearEquationStrategy = "algebra.linear-equation.solve";
export interface RunLinearEquationStrategyRequest { equation: string; variable: string; strategy: LinearEquationStrategy }
export interface RunLinearEquationStrategyResult { outcome: MathematicalOutcome; relation: "strategy.linear-equation"; strategy: LinearEquationStrategy; initialLatex: string; resultLatex: string | null; steps: MathDerivationStep[]; diagnostics: MathDiagnostic[] }

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
  inputFormat: MathExpressionInputFormat | "plain";
  grading:
    | { mode: "exact" }
    | {
        mode: "approximate";
        absoluteTolerance: string;
        relativeTolerance: string | null;
      };
}

export interface CompareNumericAnswerResult {
  outcome: MathematicalOutcome;
  relation: "number.exact_equal" | "number.within_tolerance";
  equal: boolean | null;
  submittedNormalized: ExactValue | null;
  expectedNormalized: ExactValue | null;
  absoluteError: ExactValue | null;
  acceptedTolerance: ExactValue | null;
  diagnostics: MathDiagnostic[];
}

export interface BaseTenPlace { exponent: number; coefficient: number }
export interface DecomposeBaseTenRequest { value: ExactValue; minimumExponent: number; maximumExponent: number }
export interface ComposeBaseTenRequest { places: readonly BaseTenPlace[] }
export interface CompareBaseTenRequest extends DecomposeBaseTenRequest { submitted: readonly BaseTenPlace[] }
export interface BaseTenDecompositionResult { outcome: MathematicalOutcome; value: ExactValue | null; places: BaseTenPlace[]; diagnostics: MathDiagnostic[] }
export interface CompareBaseTenResult { outcome: MathematicalOutcome; relation: "number.base-ten-place-value"; equal: boolean | null; expectedNormalized: ExactValue | null; submittedNormalized: ExactValue | null; expectedPlaces: BaseTenPlace[]; submittedPlaces: BaseTenPlace[]; diagnostics: MathDiagnostic[] }

export interface ExactPointInput {
  x: string;
  y: string;
}

export type ExactPointComparison =
  | "equal"
  | "coordinates_swapped"
  | "x_sign_error"
  | "y_sign_error"
  | "both_sign_errors"
  | "scale_mismatch"
  | "x_mismatch"
  | "y_mismatch"
  | "mismatch";

export interface CompareExactPointAnswerRequest {
  submitted: ExactPointInput;
  expected: ExactPointInput;
  gridStep: string;
  inputFormat: MathExpressionInputFormat | "plain";
}

export interface CompareExactPointAnswerResult {
  outcome: MathematicalOutcome;
  relation: "point.exact_equal";
  equal: boolean | null;
  comparison: ExactPointComparison | null;
  submittedNormalized: { x: ExactValue; y: ExactValue } | null;
  expectedNormalized: { x: ExactValue; y: ExactValue } | null;
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
  decomposeBaseTen(request: DecomposeBaseTenRequest): BaseTenDecompositionResult;
  composeBaseTen(request: ComposeBaseTenRequest): BaseTenDecompositionResult;
  compareBaseTen(request: CompareBaseTenRequest): CompareBaseTenResult;
  validateRealFunction(request: ValidateRealFunctionRequest): ValidateRealFunctionResult;
  compareRealFunctions(request: CompareRealFunctionsRequest): CompareRealFunctionsResult;
  evaluateRealFunction(request: EvaluateRealFunctionRequest): EvaluateRealFunctionResult;
  evaluateRealFunctionTable(request: EvaluateRealFunctionTableRequest): EvaluateRealFunctionTableResult;
  averageRate(request: AverageRateRequest): AverageRateResult;
  deriveDifferenceQuotient(request: DifferenceQuotientRequest): DifferenceQuotientResult;
  applyDifferenceQuotientRule(request: ApplyDifferenceQuotientRuleRequest): ApplyDifferenceQuotientRuleResult;
  sampleRealFunctionGraph(request: SampleRealFunctionGraphRequest): SampleRealFunctionGraphResult;
  queryRealFunctionGraphFeatures(request: QueryRealFunctionGraphFeaturesRequest): QueryRealFunctionGraphFeaturesResult;
  normalizeRealDomain(request: NormalizeRealDomainRequest): NormalizeRealDomainResult;
  intersectRealDomains(request: IntersectRealDomainsRequest): NormalizeRealDomainResult;
  compareRealDomains(request: CompareRealDomainsRequest): CompareRealDomainsResult;
  realDomainContains(request: RealDomainContainsRequest): RealDomainMembershipResult;
  protocolManifest(): ProtocolManifest;
  validateMathExpression(request: ValidateMathExpressionRequest): ValidateMathExpressionResult;
  applyLinearEquationRule(request: ApplyLinearEquationRuleRequest): ApplyLinearEquationRuleResult;
  runLinearEquationStrategy(request: RunLinearEquationStrategyRequest): RunLinearEquationStrategyResult;
  validatePolynomialDerivation(
    request: ValidatePolynomialDerivationRequest,
  ): ValidatePolynomialDerivationResult;

  compareAuthoredLinearSolutionSets(
    request: CompareAuthoredLinearSolutionSetsRequest,
  ): CompareAuthoredLinearSolutionSetsResult;

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

  compareExactPointAnswer(
    request: CompareExactPointAnswerRequest,
  ): CompareExactPointAnswerResult;

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
