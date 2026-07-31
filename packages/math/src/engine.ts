import type {
  ApplyRuleResponseDto,
  ApplyLinearEquationRuleResponseDto,
  RunLinearEquationStrategyResponseDto,
  CompareEquationSolutionSetsResponseDto,
  CompareMathExpressionsResponseDto,
  CompareNumericAnswerResponseDto,
  CompareSetExpressionsResponseDto,
  EvaluateFiniteRelationPredicateResponseDto,
  EvaluateSetCardinalityResponseDto,
  EvaluateSetStatementResponseDto,
  ListApplicableRulesResponseDto,
  NormalizeMathExpressionResponseDto,
  NormalizeSetExpressionResponseDto,
  SolveLinearEquationResponseDto,
  TransformMathExpressionResponseDto,
  WasmMathEngineBinding,
} from "./dto.js";
import {
  mapApplyRuleResponse,
  mapCompareEquationSolutionSetsResponse,
  mapCompareMathExpressionsResponse,
  mapCompareNumericAnswerResponse,
  mapCompareSetExpressionsResponse,
  mapEvaluateFiniteRelationPredicateResponse,
  mapEvaluateSetCardinalityResponse,
  mapEvaluateSetStatementResponse,
  mapListApplicableRulesResponse,
  mapNormalizeMathExpressionResponse,
  mapNormalizeSetExpressionResponse,
  mapSolveLinearEquationResponse,
  mapTransformMathExpressionResponse,
  toRuleTargetDto,
} from "./mapping.js";
import type { MathEngine } from "./types.js";
import type { AuthoredLinearSolutionSet, CompareAuthoredLinearSolutionSetsRequest, CompareAuthoredLinearSolutionSetsResult, ValidatePolynomialDerivationRequest, ValidatePolynomialDerivationResult } from "./types.js";

export interface CreateMathEngineOptions {
  wasmEngine: WasmMathEngineBinding;
}

export async function createMathEngine(
  options: CreateMathEngineOptions,
): Promise<MathEngine> {
  const wasmEngine = options.wasmEngine;

  return {
    runLinearEquationStrategy(request) {
      if (!wasmEngine.runLinearEquationStrategy) return { outcome: "unknown", relation: "strategy.linear-equation", strategy: request.strategy, initialLatex: request.equation, resultLatex: null, steps: [], diagnostics: [{ code: "Engine.UnsupportedOperation", message: "The loaded engine cannot run linear-equation strategies." }] };
      const dto = parseJson<RunLinearEquationStrategyResponseDto>(wasmEngine.runLinearEquationStrategy(request.equation, request.variable, request.strategy));
      return { outcome: dto.outcome, relation: "strategy.linear-equation", strategy: request.strategy, initialLatex: dto.initial_latex, resultLatex: dto.result_latex, steps: dto.steps.map(step => ({ rule: step.rule, reason: step.reason, target: step.target ? { kind: "whole" as const } : null, inputLatex: step.input_latex, outputLatex: step.output_latex })), diagnostics: dto.diagnostics };
    },
    applyLinearEquationRule(request) {
      if (!wasmEngine.applyLinearEquationRule) return { outcome: "unknown", relation: "rule.application", previousLatex: request.equation, resultLatex: null, step: null, diagnostics: [{ code: "Engine.UnsupportedOperation", message: "The loaded engine cannot apply linear equation rules." }] };
      const dto = parseJson<ApplyLinearEquationRuleResponseDto>(wasmEngine.applyLinearEquationRule(request.equation, request.variable, request.rule, request.operand));
      return { outcome: dto.outcome, relation: "rule.application", previousLatex: dto.previous_latex, resultLatex: dto.result_latex, step: dto.step ? { rule: dto.step.rule, reason: dto.step.reason, target: dto.step.target ? { kind: "whole" } : null, inputLatex: dto.step.input_latex, outputLatex: dto.step.output_latex } : null, diagnostics: dto.diagnostics };
    },
    validatePolynomialDerivation(request) {
      return validatePolynomialDerivation(wasmEngine, request);
    },
    compareAuthoredLinearSolutionSets(request) {
      return compareAuthoredLinearSolutionSets(wasmEngine, request);
    },
    solveLinearEquation(request) {
      const dto = parseJson<SolveLinearEquationResponseDto>(
        wasmEngine.solveLinearEquation(request.equation, request.variable),
      );
      return mapSolveLinearEquationResponse(dto);
    },

    compareEquationSolutionSets(request) {
      const dto = parseJson<CompareEquationSolutionSetsResponseDto>(
        wasmEngine.compareEquationSolutionSets(
          request.leftEquation,
          request.rightEquation,
          request.variable,
        ),
      );
      return mapCompareEquationSolutionSetsResponse(dto);
    },

    normalizeMathExpression(request) {
      if (!wasmEngine.normalizeMathExpression) {
        return {
          outcome: "unknown",
          normalized: null,
          diagnostics: [
            {
              code: "Engine.UnsupportedOperation",
              message: "The loaded math engine cannot normalize expressions.",
            },
          ],
        };
      }

      const dto = parseJson<NormalizeMathExpressionResponseDto>(
        wasmEngine.normalizeMathExpression(
          request.expression,
          request.inputFormat,
          request.variable,
        ),
      );
      return mapNormalizeMathExpressionResponse(dto);
    },

    compareMathExpressions(request) {
      if (!wasmEngine.compareMathExpressions) {
        return {
          outcome: "unknown",
          relation: "expression.equivalent",
          equal: null,
          leftNormalized: null,
          rightNormalized: null,
          diagnostics: [
            {
              code: "Engine.UnsupportedOperation",
              message: "The loaded math engine cannot compare expressions.",
            },
          ],
        };
      }

      const dto = parseJson<CompareMathExpressionsResponseDto>(
        wasmEngine.compareMathExpressions(
          request.leftExpression,
          request.rightExpression,
          request.inputFormat,
          request.variable,
        ),
      );
      return mapCompareMathExpressionsResponse(dto);
    },

    normalizeSetExpression(request) {
      if (!wasmEngine.normalizeSetExpression) {
        return {
          outcome: "unknown",
          normalized: null,
          diagnostics: [
            {
              code: "Engine.UnsupportedOperation",
              message: "The loaded math engine cannot normalize set expressions.",
            },
          ],
        };
      }

      const dto = parseJson<NormalizeSetExpressionResponseDto>(
        wasmEngine.normalizeSetExpression(request.expression, request.inputFormat),
      );
      return mapNormalizeSetExpressionResponse(dto);
    },

    compareSetExpressions(request) {
      if (!wasmEngine.compareSetExpressions) {
        return {
          outcome: "unknown",
          relation: "set.extensional_equal",
          equal: null,
          leftNormalized: null,
          rightNormalized: null,
          diagnostics: [
            {
              code: "Engine.UnsupportedOperation",
              message: "The loaded math engine cannot compare set expressions.",
            },
          ],
        };
      }

      const dto = parseJson<CompareSetExpressionsResponseDto>(
        wasmEngine.compareSetExpressions(
          request.leftExpression,
          request.rightExpression,
          request.inputFormat,
        ),
      );
      return mapCompareSetExpressionsResponse(dto);
    },

    compareSetExpressionsInContext(request) {
      if (!wasmEngine.compareSetExpressionsInContext) {
        return {
          outcome: "unknown",
          relation: "set.extensional_equal.in_context",
          equal: null,
          leftNormalized: null,
          rightNormalized: null,
          diagnostics: [
            {
              code: "Engine.UnsupportedOperation",
              message:
                "The loaded math engine cannot compare contextual set expressions.",
            },
          ],
        };
      }

      const dto = parseJson<CompareSetExpressionsResponseDto>(
        wasmEngine.compareSetExpressionsInContext(
          request.leftExpression,
          request.rightExpression,
          request.universeExpression,
          JSON.stringify(request.bindings),
          request.inputFormat,
        ),
      );
      return mapCompareSetExpressionsResponse(dto);
    },

    evaluateSetStatement(request) {
      if (!wasmEngine.evaluateSetStatement) {
        return {
          outcome: "unknown",
          relation: "logic.truth",
          truth: null,
          normalized: null,
          diagnostics: [
            {
              code: "Engine.UnsupportedOperation",
              message: "The loaded math engine cannot evaluate set statements.",
            },
          ],
        };
      }

      const dto = parseJson<EvaluateSetStatementResponseDto>(
        wasmEngine.evaluateSetStatement(request.statement, request.inputFormat),
      );
      return mapEvaluateSetStatementResponse(dto);
    },

    evaluateSetCardinality(request) {
      if (!wasmEngine.evaluateSetCardinality) {
        return {
          outcome: "unknown",
          relation: "set.cardinality",
          cardinality: null,
          cardinalityLatex: null,
          normalizedSet: null,
          diagnostics: [
            {
              code: "Engine.UnsupportedOperation",
              message: "The loaded math engine cannot evaluate set cardinality.",
            },
          ],
        };
      }

      const dto = parseJson<EvaluateSetCardinalityResponseDto>(
        wasmEngine.evaluateSetCardinality(
          request.expression,
          request.inputFormat,
        ),
      );
      return mapEvaluateSetCardinalityResponse(dto);
    },

    evaluateRelationFrom(request) {
      if (!wasmEngine.evaluateRelationFrom) {
        return unsupportedFiniteRelationPredicate(
          "relation.from",
          "The loaded math engine cannot evaluate finite relation predicates.",
        );
      }

      const dto = parseJson<EvaluateFiniteRelationPredicateResponseDto>(
        wasmEngine.evaluateRelationFrom(
          request.relationExpression,
          request.domainExpression,
          request.codomainExpression,
          request.inputFormat,
        ),
      );
      return mapEvaluateFiniteRelationPredicateResponse(dto);
    },

    evaluateFunctionFrom(request) {
      if (!wasmEngine.evaluateFunctionFrom) {
        return unsupportedFiniteRelationPredicate(
          "function.from",
          "The loaded math engine cannot evaluate finite function predicates.",
        );
      }

      const dto = parseJson<EvaluateFiniteRelationPredicateResponseDto>(
        wasmEngine.evaluateFunctionFrom(
          request.relationExpression,
          request.domainExpression,
          request.codomainExpression,
          request.inputFormat,
        ),
      );
      return mapEvaluateFiniteRelationPredicateResponse(dto);
    },

    evaluateRelationProperty(request) {
      if (!wasmEngine.evaluateRelationProperty) {
        return unsupportedFiniteRelationPredicate(
          `relation.${request.property}`,
          "The loaded math engine cannot evaluate finite relation properties.",
        );
      }

      const dto = parseJson<EvaluateFiniteRelationPredicateResponseDto>(
        wasmEngine.evaluateRelationProperty(
          request.relationExpression,
          request.setExpression,
          request.property,
          request.inputFormat,
        ),
      );
      return mapEvaluateFiniteRelationPredicateResponse(dto);
    },

    evaluateRelationDomain(request) {
      if (!wasmEngine.evaluateRelationDomain) {
        return unsupportedSetNormalization(
          "The loaded math engine cannot evaluate relation domains.",
        );
      }

      const dto = parseJson<NormalizeSetExpressionResponseDto>(
        wasmEngine.evaluateRelationDomain(
          request.relationExpression,
          request.inputFormat,
        ),
      );
      return mapNormalizeSetExpressionResponse(dto);
    },

    evaluateRelationRange(request) {
      if (!wasmEngine.evaluateRelationRange) {
        return unsupportedSetNormalization(
          "The loaded math engine cannot evaluate relation ranges.",
        );
      }

      const dto = parseJson<NormalizeSetExpressionResponseDto>(
        wasmEngine.evaluateRelationRange(
          request.relationExpression,
          request.inputFormat,
        ),
      );
      return mapNormalizeSetExpressionResponse(dto);
    },

    evaluateRelationInverse(request) {
      if (!wasmEngine.evaluateRelationInverse) {
        return unsupportedSetNormalization(
          "The loaded math engine cannot evaluate relation inverses.",
        );
      }

      const dto = parseJson<NormalizeSetExpressionResponseDto>(
        wasmEngine.evaluateRelationInverse(
          request.relationExpression,
          request.inputFormat,
        ),
      );
      return mapNormalizeSetExpressionResponse(dto);
    },

    compareNumericAnswer(request) {
      if (!wasmEngine.compareNumericAnswer) {
        return {
          outcome: "unknown",
          relation: request.grading.mode === "exact"
            ? "number.exact_equal"
            : "number.within_tolerance",
          equal: null,
          submittedNormalized: null,
          expectedNormalized: null,
          absoluteError: null,
          acceptedTolerance: null,
          diagnostics: [
            {
              code: "Engine.UnsupportedOperation",
              message: "The loaded math engine cannot compare numeric answers.",
            },
          ],
        };
      }

      const dto = parseJson<CompareNumericAnswerResponseDto>(
        wasmEngine.compareNumericAnswer(
          request.submitted,
          request.expected,
          request.inputFormat,
          request.grading.mode,
          request.grading.mode === "approximate"
            ? request.grading.absoluteTolerance
            : "0",
          request.grading.mode === "approximate"
            ? request.grading.relativeTolerance ?? undefined
            : undefined,
        ),
      );
      return mapCompareNumericAnswerResponse(dto);
    },

    differentiateMathExpression(request) {
      if (!wasmEngine.differentiateMathExpression) {
        return unsupportedTransform(
          "calculus.derivative",
          "The loaded math engine cannot differentiate expressions.",
        );
      }

      const dto = parseJson<TransformMathExpressionResponseDto>(
        wasmEngine.differentiateMathExpression(
          request.expression,
          request.inputFormat,
          request.variable,
        ),
      );
      return mapTransformMathExpressionResponse(dto);
    },

    integrateMathExpression(request) {
      if (!wasmEngine.integrateMathExpression) {
        return unsupportedTransform(
          "calculus.antiderivative",
          "The loaded math engine cannot integrate expressions.",
        );
      }

      const dto = parseJson<TransformMathExpressionResponseDto>(
        wasmEngine.integrateMathExpression(
          request.expression,
          request.inputFormat,
          request.variable,
        ),
      );
      return mapTransformMathExpressionResponse(dto);
    },

    listApplicableMathExpressionRules(request) {
      if (!wasmEngine.listApplicableMathExpressionRules) {
        return {
          outcome: "unknown",
          rules: [],
          diagnostics: [
            {
              code: "Engine.UnsupportedOperation",
              message:
                "The loaded math engine cannot list applicable expression rules.",
            },
          ],
        };
      }

      const dto = parseJson<ListApplicableRulesResponseDto>(
        wasmEngine.listApplicableMathExpressionRules(
          request.expression,
          request.inputFormat,
          request.variable,
          serializeTarget(request.target),
        ),
      );
      return mapListApplicableRulesResponse(dto);
    },

    applyMathExpressionRule(request) {
      if (!wasmEngine.applyMathExpressionRule) {
        return {
          outcome: "unknown",
          relation: "rule.application",
          previous: null,
          result: null,
          step: null,
          diagnostics: [
            {
              code: "Engine.UnsupportedOperation",
              message:
                "The loaded math engine cannot apply expression rules directly.",
            },
          ],
        };
      }

      const dto = parseJson<ApplyRuleResponseDto>(
        wasmEngine.applyMathExpressionRule(
          request.expression,
          request.inputFormat,
          request.variable,
          request.rule,
          serializeTarget(request.target),
        ),
      );
      return mapApplyRuleResponse(dto);
    },
  };
}

function validatePolynomialDerivation(wasmEngine: WasmMathEngineBinding, request: ValidatePolynomialDerivationRequest): ValidatePolynomialDerivationResult {
  if (!wasmEngine.compareMathExpressions || request.submittedSteps.length === 0) {
    return { outcome: "unknown", relation: "derivation.polynomial_identity", valid: null, steps: [], reachesGoal: null, diagnostics: [{ code: "Derivation.MissingStepsOrEngine", message: "At least one step and expression comparison support are required." }] };
  }
  const expressions = [request.initialExpression, ...request.submittedSteps];
  const steps = expressions.slice(1).map((output, index) => {
    const input = expressions[index]!;
    const comparison = mapCompareMathExpressionsResponse(parseJson<CompareMathExpressionsResponseDto>(wasmEngine.compareMathExpressions!(input, output, request.inputFormat, request.variable)));
    return { input, output, outcome: comparison.outcome, equivalent: comparison.equal, inputNormalized: comparison.leftNormalized, outputNormalized: comparison.rightNormalized, diagnostics: comparison.diagnostics };
  });
  const finalStep = request.submittedSteps[request.submittedSteps.length - 1]!;
  const goal = mapCompareMathExpressionsResponse(parseJson<CompareMathExpressionsResponseDto>(wasmEngine.compareMathExpressions(finalStep, request.goalExpression, request.inputFormat, request.variable)));
  const unknown = steps.some(step => step.equivalent === null) || goal.equal === null;
  const valid = unknown ? null : steps.every(step => step.equivalent === true) && goal.equal === true;
  return { outcome: unknown ? "unknown" : valid ? "proven" : "disproven", relation: "derivation.polynomial_identity", valid, steps, reachesGoal: goal.equal, diagnostics: [...steps.flatMap(step => step.diagnostics), ...goal.diagnostics] };
}

function compareAuthoredLinearSolutionSets(
  wasmEngine: WasmMathEngineBinding,
  request: CompareAuthoredLinearSolutionSetsRequest,
): CompareAuthoredLinearSolutionSetsResult {
  const diagnostics: { code: string; message: string }[] = [];
  const compareExact = (left: string, right: string): boolean | null => {
    if (!wasmEngine.compareNumericAnswer) {
      diagnostics.push({ code: "Engine.UnsupportedOperation", message: "The loaded engine cannot compare exact boundaries." });
      return null;
    }
    const dto = parseJson<CompareNumericAnswerResponseDto>(wasmEngine.compareNumericAnswer(
      left,
      right,
      "plain",
      "exact",
      "0",
      undefined,
    ));
    diagnostics.push(...dto.diagnostics);
    return dto.equal;
  };
  const sameBoundary = (left: string, right: string): boolean | null => compareExact(left, right);
  const left = request.left;
  const right = request.right;
  if (left.kind !== right.kind) return { outcome: "disproven", relation: "linear_solution_set.equal", equal: false, diagnostics };

  let equal: boolean | null;
  switch (left.kind) {
    case "empty":
    case "all_reals": equal = true; break;
    case "point": equal = sameBoundary(left.value, (right as Extract<AuthoredLinearSolutionSet, { kind: "point" }>).value); break;
    case "ray": {
      const other = right as Extract<AuthoredLinearSolutionSet, { kind: "ray" }>;
      if (left.direction !== other.direction || left.inclusive !== other.inclusive) equal = false;
      else equal = sameBoundary(left.boundary, other.boundary);
      break;
    }
    case "interval": {
      const other = right as Extract<AuthoredLinearSolutionSet, { kind: "interval" }>;
      if (left.lower_inclusive !== other.lower_inclusive || left.upper_inclusive !== other.upper_inclusive) equal = false;
      else {
        const lower = sameBoundary(left.lower, other.lower);
        const upper = sameBoundary(left.upper, other.upper);
        equal = lower === null || upper === null ? null : lower && upper;
      }
      break;
    }
  }
  return {
    outcome: equal === null ? "unknown" : equal ? "proven" : "disproven",
    relation: "linear_solution_set.equal",
    equal,
    diagnostics,
  };
}

function parseJson<T>(json: string): T {
  return JSON.parse(json) as T;
}

function serializeTarget(
  target: Parameters<typeof toRuleTargetDto>[0] | null | undefined,
): string | null {
  return target ? JSON.stringify(toRuleTargetDto(target)) : null;
}

function unsupportedFiniteRelationPredicate(
  relation: ReturnType<
    typeof mapEvaluateFiniteRelationPredicateResponse
  >["relation"],
  message: string,
) {
  return {
    outcome: "unknown" as const,
    relation,
    truth: null,
    normalizedRelation: null,
    normalizedDomain: null,
    normalizedCodomain: null,
    diagnostics: [
      {
        code: "Engine.UnsupportedOperation",
        message,
      },
    ],
  };
}

function unsupportedSetNormalization(message: string) {
  return {
    outcome: "unknown" as const,
    normalized: null,
    diagnostics: [
      {
        code: "Engine.UnsupportedOperation",
        message,
      },
    ],
  };
}

function unsupportedTransform(
  relation: "calculus.derivative" | "calculus.antiderivative",
  message: string,
) {
  return {
    outcome: "unknown" as const,
    relation,
    result: null,
    steps: [],
    diagnostics: [
      {
        code: "Engine.UnsupportedOperation",
        message,
      },
    ],
  };
}
