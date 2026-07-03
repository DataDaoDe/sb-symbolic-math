import { execFileSync } from "node:child_process";
import { fileURLToPath } from "node:url";
import { dirname, resolve } from "node:path";

const here = dirname(fileURLToPath(import.meta.url));
const workspaceRoot = resolve(here, "../../..");

function runApi(args: readonly string[]): string {
  return execFileSync(
    "cargo",
    ["run", "-q", "-p", "socrates-math-app", "--example", "api", "--", ...args],
    {
      cwd: workspaceRoot,
      encoding: "utf8",
      env: { ...process.env, CARGO_INCREMENTAL: "0" },
      stdio: ["ignore", "pipe", "inherit"],
    },
  ).trim();
}

export class LocalRustMathEngineBinding {
  solveLinearEquation(source: string, variable: string): string {
    return runApi(["solve-linear", source, variable]);
  }

  compareEquationSolutionSets(
    leftSource: string,
    rightSource: string,
    variable: string,
  ): string {
    return runApi(["compare-equations", leftSource, rightSource, variable]);
  }

  normalizeMathExpression(
    source: string,
    inputFormat: string,
    variable: string,
  ): string {
    return runApi(["normalize-expression", source, inputFormat, variable]);
  }

  compareMathExpressions(
    leftSource: string,
    rightSource: string,
    inputFormat: string,
    variable: string,
  ): string {
    return runApi([
      "compare-expressions",
      leftSource,
      rightSource,
      inputFormat,
      variable,
    ]);
  }

  compareNumericAnswer(
    submittedSource: string,
    expectedSource: string,
    inputFormat: string,
    tolerance: number,
  ): string {
    return runApi([
      "compare-numeric",
      submittedSource,
      expectedSource,
      inputFormat,
      tolerance.toString(),
    ]);
  }

  differentiateMathExpression(
    source: string,
    inputFormat: string,
    variable: string,
  ): string {
    return runApi(["differentiate", source, inputFormat, variable]);
  }

  integrateMathExpression(
    source: string,
    inputFormat: string,
    variable: string,
  ): string {
    return runApi(["integrate", source, inputFormat, variable]);
  }
}
