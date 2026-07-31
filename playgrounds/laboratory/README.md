# Symbolic Math Laboratory

Interactive browser test ground for the public `@socrates/math` API. It uses
the real generated WebAssembly engine; it contains no substitute mathematics.

From the repository root:

```sh
pnpm run build:wasm
pnpm run laboratory
```

The API explorer covers normalization, expression and equation comparison,
linear solving, executable equation rules, numeric comparison,
differentiation, and integration. The equation workbench creates a replayable
chain in which every output state is produced by Symbolic Math.

Experiment history is stored only in the browser's local storage and can be
cleared from the interface.
