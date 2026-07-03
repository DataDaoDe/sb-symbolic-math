# @socrates/math

Handwritten TypeScript facade for Symbolic Math.

The generated WebAssembly package is intentionally kept behind this facade.
`@socrates/math` does not depend on a published `@socrates/math-wasm` package.
Pass in whatever local/generated Wasm binding object your application builds.

The first integration shape is:

```ts
import { createMathEngine } from "@socrates/math";
import init, { WasmMathEngine } from "@socrates/math-wasm";

await init();

const engine = await createMathEngine({
  wasmEngine: new WasmMathEngine(),
});

const solved = engine.solveLinearEquation({
  equation: "3(x - 2) + 4 = 2x + 9",
  variable: "x",
});

solved.solutionSetLatex; // "x = 11"

const comparison = engine.compareEquationSolutionSets({
  leftEquation: "x + 1 = 3",
  rightEquation: "x = 2",
  variable: "x",
});

comparison.equal; // true
```

The facade returns stable, camelCase TypeScript result objects. It does not
expose generated `wasm-bindgen` classes or raw Rust DTOs as mathematical result
types.
