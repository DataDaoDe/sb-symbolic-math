import { createMathEngine } from "@socrates/math";
import { LocalRustMathEngineBinding } from "./rust-engine-binding.js";

const engine = await createMathEngine({
  wasmEngine: new LocalRustMathEngineBinding(),
});

const args = process.argv.slice(2);
if (args[0] === "--") {
  args.shift();
}

if (args.length > 0) {
  runCommand(args);
} else {
  runDemo();
}

function runCommand(args: string[]): void {
  const [command, ...values] = args;

  switch (command) {
    case "normalize": {
      const [expression, variable = "x"] = values;
      requireArgs(command, values, 1);
      print(
        `normalize math_expression: ${expression}`,
        engine.normalizeMathExpression({
          expression,
          inputFormat: "latex",
          variable,
        }),
      );
      return;
    }

    case "compare-expressions": {
      const [leftExpression, rightExpression, variable = "x"] = values;
      requireArgs(command, values, 2);
      print(
        `compare math_expression: ${leftExpression} vs ${rightExpression}`,
        engine.compareMathExpressions({
          leftExpression,
          rightExpression,
          inputFormat: "latex",
          variable,
        }),
      );
      return;
    }

    case "compare-equations": {
      const [leftEquation, rightEquation, variable = "x"] = values;
      requireArgs(command, values, 2);
      print(
        `compare math_equation: ${leftEquation} vs ${rightEquation}`,
        engine.compareEquationSolutionSets({
          leftEquation,
          rightEquation,
          variable,
        }),
      );
      return;
    }

    case "solve-linear": {
      const [equation, variable = "x"] = values;
      requireArgs(command, values, 1);
      print(
        `solve linear equation: ${equation}`,
        engine.solveLinearEquation({
          equation,
          variable,
        }),
      );
      return;
    }

    case "compare-numeric": {
      const [submitted, expected, toleranceText = "0"] = values;
      requireArgs(command, values, 2);
      const tolerance = Number(toleranceText);

      if (!Number.isFinite(tolerance)) {
        throw new Error(`compare-numeric tolerance must be a number, got '${toleranceText}'`);
      }

      print(
        `compare numeric_answer: ${submitted} vs ${expected}, tolerance ${tolerance}`,
        engine.compareNumericAnswer({
          submitted,
          expected,
          inputFormat: "latex",
          tolerance,
        }),
      );
      return;
    }

    case "differentiate": {
      const [expression, variable = "x"] = values;
      requireArgs(command, values, 1);
      print(
        `differentiate math_expression: ${expression}`,
        engine.differentiateMathExpression({
          expression,
          inputFormat: "latex",
          variable,
        }),
      );
      return;
    }

    case "integrate": {
      const [expression, variable = "x"] = values;
      requireArgs(command, values, 1);
      print(
        `integrate math_expression: ${expression}`,
        engine.integrateMathExpression({
          expression,
          inputFormat: "latex",
          variable,
        }),
      );
      return;
    }

    case "help":
    case "--help":
    case "-h":
      printUsage();
      return;

    default:
      printUsage();
      throw new Error(`Unknown playground command '${command}'`);
  }
}

function requireArgs(command: string, values: string[], minimum: number): void {
  if (values.length < minimum) {
    printUsage();
    throw new Error(`${command} expected at least ${minimum} argument(s)`);
  }
}

function runDemo(): void {
  console.log("Symbolic Math TypeScript Playground");
  console.log();

  const expression = engine.normalizeMathExpression({
    expression: "3(x - 2) + 4",
    inputFormat: "latex",
    variable: "x",
  });
  console.log("normalize math_expression: 3(x - 2) + 4");
  console.dir(expression, { depth: null });
  console.log();

  const collectedExpression = engine.normalizeMathExpression({
    expression: "15 + 10x + 39 + 4x",
    inputFormat: "latex",
    variable: "x",
  });
  console.log("normalize math_expression: 15 + 10x + 39 + 4x");
  console.dir(collectedExpression, { depth: null });
  console.log();

  const expressionComparison = engine.compareMathExpressions({
    leftExpression: "2(x + 1)",
    rightExpression: "2x + 2",
    inputFormat: "latex",
    variable: "x",
  });

  console.log("compare math_expression: 2(x + 1) vs 2x + 2");
  console.dir(expressionComparison, { depth: null });
  console.log();

  const largerExpressionComparison = engine.compareMathExpressions({
    leftExpression: "3(x + 2) + 4(5x + 7x)",
    rightExpression: "51x + 6",
    inputFormat: "latex",
    variable: "x",
  });
  console.log("compare math_expression: 3(x + 2) + 4(5x + 7x) vs 51x + 6");
  console.dir(largerExpressionComparison, { depth: null });
  console.log();

  const polynomialExpressionComparison = engine.compareMathExpressions({
    leftExpression: "(x + 1)(x - 1)",
    rightExpression: "x^2 - 1",
    inputFormat: "latex",
    variable: "x",
  });
  console.log("compare polynomial math_expression: (x + 1)(x - 1) vs x^2 - 1");
  console.dir(polynomialExpressionComparison, { depth: null });
  console.log();

  const derivative = engine.differentiateMathExpression({
    expression: "x^3 + 2x",
    inputFormat: "latex",
    variable: "x",
  });
  console.log("differentiate math_expression: x^3 + 2x");
  console.dir(derivative, { depth: null });
  console.log();

  const integral = engine.integrateMathExpression({
    expression: "x^3",
    inputFormat: "latex",
    variable: "x",
  });
  console.log("integrate math_expression: x^3");
  console.dir(integral, { depth: null });
  console.log();

  const equationComparison = engine.compareEquationSolutionSets({
    leftEquation: "x + 1 = 3",
    rightEquation: "2x = 4",
    variable: "x",
  });
  console.log("compare math_equation by solution set: x + 1 = 3 vs 2x = 4");
  console.dir(equationComparison, { depth: null });
  console.log();

  const numeric = engine.compareNumericAnswer({
    submitted: "\\frac{333}{1000}",
    expected: "\\frac{1}{3}",
    inputFormat: "latex",
    tolerance: 0.001,
  });
  console.log("compare numeric_answer: \\frac{333}{1000} vs \\frac{1}{3}, tolerance 0.001");
  console.dir(numeric, { depth: null });
}

function print(label: string, value: unknown): void {
  console.log(label);
  console.dir(value, { depth: null });
}

function printUsage(): void {
  console.log(`Symbolic Math TypeScript Playground

Usage:
  pnpm --filter @socrates/playground run try
  pnpm --filter @socrates/playground run try -- normalize <expression> [variable]
  pnpm --filter @socrates/playground run try -- compare-expressions <left> <right> [variable]
  pnpm --filter @socrates/playground run try -- compare-equations <left> <right> [variable]
  pnpm --filter @socrates/playground run try -- solve-linear <equation> [variable]
  pnpm --filter @socrates/playground run try -- compare-numeric <submitted> <expected> [tolerance]
  pnpm --filter @socrates/playground run try -- differentiate <expression> [variable]
  pnpm --filter @socrates/playground run try -- integrate <expression> [variable]

Examples:
  pnpm --filter @socrates/playground run try -- normalize "3(x - 2) + 4"
  pnpm --filter @socrates/playground run try -- compare-expressions "2(x + 1)" "2x + 2"
  pnpm --filter @socrates/playground run try -- compare-equations "x + 1 = 3" "2x = 4"
  pnpm --filter @socrates/playground run try -- compare-numeric "\\frac{333}{1000}" "\\frac{1}{3}" 0.001
  pnpm --filter @socrates/playground run try -- differentiate "x^3 + 2x"
  pnpm --filter @socrates/playground run try -- integrate "x^3"`);
}
