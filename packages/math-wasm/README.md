# @socrates/math-wasm

Local workspace package for the generated WebAssembly bindings.

This package is private and does not need to be published to npm. It exists so
Socrates Academy can depend on `@socrates/math-wasm` through a workspace, file,
or monorepo dependency.

## Build

Install `wasm-pack` once:

```bash
cargo install wasm-pack
```

Then build from the repository root:

```bash
pnpm --filter @socrates/math-wasm build
```

The generated package is written to:

```text
packages/math-wasm/pkg
```

`@socrates/math` remains the handwritten public facade. Application code should
construct `WasmMathEngine` from this package and pass it into
`createMathEngine`.
