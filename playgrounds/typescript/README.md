# TypeScript Playground

Run from the workspace root:

```sh
pnpm run playground:ts
```

Or try one operation at a time:

```sh
pnpm --filter @socrates/playground run try -- normalize "3(x - 2) + 4"
pnpm --filter @socrates/playground run try -- compare-expressions "2(x + 1)" "2x + 2"
pnpm --filter @socrates/playground run try -- compare-expressions "(x + 1)(x - 1)" "x^2 - 1"
pnpm --filter @socrates/playground run try -- compare-equations "x + 1 = 3" "2x = 4"
pnpm --filter @socrates/playground run try -- compare-numeric "\\frac{333}{1000}" "\\frac{1}{3}" 0.001
pnpm --filter @socrates/playground run try -- differentiate "x^3 + 2x"
pnpm --filter @socrates/playground run try -- integrate "x^3"
```

This playground imports `@socrates/math` like a downstream TypeScript app. It
uses a local Rust-backed binding so it can exercise the real engine without
requiring a generated WASM package first.
