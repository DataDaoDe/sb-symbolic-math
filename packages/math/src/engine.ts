import type {
  ApplyRuleResponseDto,
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

export interface CreateMathEngineOptions {
  wasmEngine: WasmMathEngineBinding;
}

export async function createMathEngine(
  options: CreateMathEngineOptions,
): Promise<MathEngine> {
  const wasmEngine = options.wasmEngine;

  return {
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
          relation: "number.within_tolerance",
          equal: null,
          submittedValue: null,
          expectedValue: null,
          absoluteError: null,
          tolerance: request.tolerance,
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
          request.tolerance,
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
