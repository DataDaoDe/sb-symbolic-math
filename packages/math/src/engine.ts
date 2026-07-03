import type {
  CompareEquationSolutionSetsResponseDto,
  CompareMathExpressionsResponseDto,
  CompareNumericAnswerResponseDto,
  NormalizeMathExpressionResponseDto,
  SolveLinearEquationResponseDto,
  TransformMathExpressionResponseDto,
  WasmMathEngineBinding,
} from "./dto.js";
import {
  mapCompareEquationSolutionSetsResponse,
  mapCompareMathExpressionsResponse,
  mapCompareNumericAnswerResponse,
  mapNormalizeMathExpressionResponse,
  mapSolveLinearEquationResponse,
  mapTransformMathExpressionResponse,
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
  };
}

function parseJson<T>(json: string): T {
  return JSON.parse(json) as T;
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
