import katex from "katex";
import "katex/dist/katex.min.css";
import { createMathEngine } from "@socrates/math";
import type { LinearEquationRule, MathEngine, MathematicalOutcome } from "@socrates/math";
import initWasm, { WasmMathEngine } from "@socrates/math-wasm";
import "./style.css";

type Operation =
  | "normalize"
  | "compareExpressions"
  | "solveLinear"
  | "compareEquations"
  | "applyEquationRule"
  | "compareNumeric"
  | "differentiate"
  | "integrate";

interface Experiment {
  operation: Operation;
  primary: string;
  secondary: string;
  variable: string;
  rule: LinearEquationRule;
  tolerance: string;
}

interface HistoryEntry {
  id: string;
  createdAt: string;
  experiment: Experiment;
  result: unknown;
}

const labels: Record<Operation, string> = {
  normalize: "Normalize expression",
  compareExpressions: "Compare expressions",
  solveLinear: "Solve linear equation",
  compareEquations: "Compare equation solution sets",
  applyEquationRule: "Apply equation rule",
  compareNumeric: "Compare numeric answers",
  differentiate: "Differentiate",
  integrate: "Integrate",
};

const examples: Record<Operation, Pick<Experiment, "primary" | "secondary" | "variable" | "rule" | "tolerance">[]> = {
  normalize: [{ primary: "3(x - 2) + 4", secondary: "", variable: "x", rule: "algebra.linear-equation.simplify-both-sides", tolerance: "0.001" }],
  compareExpressions: [
    { primary: "(x + 1)(x - 1)", secondary: "x^2 - 1", variable: "x", rule: "algebra.linear-equation.simplify-both-sides", tolerance: "0.001" },
    { primary: "\\frac{1}{2}x + \\frac{1}{3}x", secondary: "\\frac{5}{6}x", variable: "x", rule: "algebra.linear-equation.simplify-both-sides", tolerance: "0.001" },
  ],
  solveLinear: [
    { primary: "3(x - 2) + 4 = 2x + 9", secondary: "", variable: "x", rule: "algebra.linear-equation.solve", tolerance: "0.001" },
    { primary: "2(x + 1) = 2x + 2", secondary: "", variable: "x", rule: "algebra.linear-equation.solve", tolerance: "0.001" },
  ],
  compareEquations: [{ primary: "x + 1 = 3", secondary: "2x = 4", variable: "x", rule: "algebra.linear-equation.solve", tolerance: "0.001" }],
  applyEquationRule: [{ primary: "3(x - 2) + 4 = 2x + 9", secondary: "", variable: "x", rule: "algebra.linear-equation.simplify-both-sides", tolerance: "0.001" }],
  compareNumeric: [{ primary: "\\frac{333}{1000}", secondary: "\\frac{1}{3}", variable: "x", rule: "algebra.linear-equation.solve", tolerance: "0.001" }],
  differentiate: [{ primary: "x^3 + 2x", secondary: "", variable: "x", rule: "algebra.linear-equation.solve", tolerance: "0.001" }],
  integrate: [{ primary: "x^3", secondary: "", variable: "x", rule: "algebra.linear-equation.solve", tolerance: "0.001" }],
};

const initial: Experiment = { operation: "normalize", ...examples.normalize[0] };
let engine: MathEngine;
let current = { ...initial };
let lastResult: unknown = null;
let history = readHistory();
let workbenchEquation = "3(x - 2) + 4 = 2x + 9";
let workbenchVariable = "x";
let workbenchSteps: Array<{ rule: LinearEquationRule; input: string; output: string }> = [];

const root = document.querySelector<HTMLDivElement>("#app");
if (!root) throw new Error("Laboratory root element is missing.");

root.innerHTML = `
  <header class="lab-header">
    <div><p class="eyebrow">Socrates developer tools</p><h1>Symbolic Math Laboratory</h1></div>
    <div class="engine-status" id="engine-status"><span></span> Loading the real WASM engine…</div>
  </header>
  <main class="lab-shell">
    <nav class="mode-tabs" aria-label="Laboratory modes">
      <button class="active" data-mode="experiment">API explorer</button>
      <button data-mode="workbench">Equation workbench</button>
    </nav>
    <section id="experiment-mode" class="mode-panel"></section>
    <section id="workbench-mode" class="mode-panel hidden"></section>
  </main>`;

await initWasm();
engine = await createMathEngine({ wasmEngine: new WasmMathEngine() });
const status = document.querySelector<HTMLElement>("#engine-status");
if (status) status.innerHTML = "<span></span> Real WASM engine ready";

renderExperiment();
renderWorkbench();
bindModeTabs();
document.addEventListener("keydown", event => {
  if ((event.metaKey || event.ctrlKey) && event.key === "Enter" && !requiredElement("experiment-mode").classList.contains("hidden")) {
    event.preventDefault();
    runCurrent();
  }
});

function renderExperiment(): void {
  const panel = requiredElement("experiment-mode");
  const needsSecondary = ["compareExpressions", "compareEquations", "compareNumeric"].includes(current.operation);
  const isRule = current.operation === "applyEquationRule";
  const isNumeric = current.operation === "compareNumeric";
  const result = lastResult as { outcome?: MathematicalOutcome; diagnostics?: unknown[] } | null;
  panel.innerHTML = `
    <div class="workspace-grid">
      <aside class="operation-list">
        <h2>Operations</h2>
        ${Object.entries(labels).map(([key, label]) => `<button data-operation="${key}" class="${current.operation === key ? "active" : ""}"><span>${label}</span><small>${operationHint(key as Operation)}</small></button>`).join("")}
      </aside>
      <section class="experiment-card">
        <div class="section-heading"><div><p class="eyebrow">Public API operation</p><h2>${labels[current.operation]}</h2></div><button class="secondary-button" id="load-example">Load another example</button></div>
        <div class="field-grid">
          ${field("primary", primaryLabel(current.operation), current.primary, "textarea")}
          ${needsSecondary ? field("secondary", secondaryLabel(current.operation), current.secondary, "textarea") : ""}
          ${!isNumeric ? field("variable", "Variable", current.variable) : ""}
          ${isRule ? selectRule(current.rule) : ""}
          ${isNumeric ? field("tolerance", "Absolute tolerance", current.tolerance) : ""}
        </div>
        <div class="math-preview-grid">
          ${mathCard("Input", current.primary)}
          ${needsSecondary ? mathCard(isNumeric ? "Expected" : "Comparison", current.secondary) : ""}
        </div>
        <button class="run-button" id="run-operation">Run operation <span>⌘↵</span></button>
      </section>
      <section class="result-card">
        <div class="section-heading"><div><p class="eyebrow">Engine response</p><h2>Result</h2></div>${result?.outcome ? outcomeBadge(result.outcome) : ""}</div>
        ${lastResult === null ? `<div class="empty-state"><div>∴</div><p>Run an operation to inspect its mathematical outcome, rendered result, diagnostics, and raw DTO.</p></div>` : resultView(lastResult)}
      </section>
    </div>
    <section class="history-card">
      <div class="section-heading"><div><p class="eyebrow">Stored locally</p><h2>Experiment history</h2></div>${history.length ? `<button class="text-button" id="clear-history">Clear history</button>` : ""}</div>
      ${historyView()}
    </section>`;

  panel.querySelectorAll<HTMLButtonElement>("[data-operation]").forEach(button => button.addEventListener("click", () => {
    const operation = button.dataset.operation as Operation;
    current = { operation, ...examples[operation][0] };
    lastResult = null;
    renderExperiment();
  }));
  panel.querySelectorAll<HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement>("[data-field]").forEach(input => input.addEventListener("input", () => {
    current = { ...current, [input.dataset.field as keyof Experiment]: input.value } as Experiment;
    panel.querySelectorAll<HTMLElement>("[data-math-source]").forEach(preview => {
      const source = preview.dataset.mathSource === "secondary" ? current.secondary : current.primary;
      renderMath(preview, source);
    });
  }));
  panel.querySelector<HTMLButtonElement>("#run-operation")?.addEventListener("click", runCurrent);
  panel.querySelector<HTMLButtonElement>("#load-example")?.addEventListener("click", loadNextExample);
  panel.querySelector<HTMLButtonElement>("#clear-history")?.addEventListener("click", () => { history = []; persistHistory(); renderExperiment(); });
  panel.querySelectorAll<HTMLButtonElement>("[data-history-id]").forEach(button => button.addEventListener("click", () => {
    const entry = history.find(item => item.id === button.dataset.historyId);
    if (entry) { current = { ...entry.experiment }; lastResult = entry.result; renderExperiment(); }
  }));
  panel.querySelectorAll<HTMLElement>("[data-math-source]").forEach(preview => renderMath(preview, preview.dataset.mathSource === "secondary" ? current.secondary : current.primary));
  panel.querySelectorAll<HTMLElement>("[data-render-latex]").forEach(preview => renderMath(preview, preview.dataset.renderLatex ?? ""));
}

function runCurrent(): void {
  try {
    switch (current.operation) {
      case "normalize": lastResult = engine.normalizeMathExpression({ expression: current.primary, inputFormat: "latex", variable: current.variable }); break;
      case "compareExpressions": lastResult = engine.compareMathExpressions({ leftExpression: current.primary, rightExpression: current.secondary, inputFormat: "latex", variable: current.variable }); break;
      case "solveLinear": lastResult = engine.solveLinearEquation({ equation: current.primary, variable: current.variable }); break;
      case "compareEquations": lastResult = engine.compareEquationSolutionSets({ leftEquation: current.primary, rightEquation: current.secondary, variable: current.variable }); break;
      case "applyEquationRule": lastResult = engine.applyLinearEquationRule({ equation: current.primary, variable: current.variable, rule: current.rule }); break;
      case "compareNumeric": lastResult = engine.compareNumericAnswer({ submitted: current.primary, expected: current.secondary, inputFormat: "latex", grading: { mode: "approximate", absoluteTolerance: current.tolerance, relativeTolerance: null } }); break;
      case "differentiate": lastResult = engine.differentiateMathExpression({ expression: current.primary, inputFormat: "latex", variable: current.variable }); break;
      case "integrate": lastResult = engine.integrateMathExpression({ expression: current.primary, inputFormat: "latex", variable: current.variable }); break;
    }
  } catch (error) {
    lastResult = { outcome: "undefined", diagnostics: [{ code: "Laboratory.RuntimeError", message: error instanceof Error ? error.message : String(error) }] };
  }
  history = [{ id: crypto.randomUUID(), createdAt: new Date().toISOString(), experiment: { ...current }, result: lastResult }, ...history].slice(0, 30);
  persistHistory();
  renderExperiment();
}

function renderWorkbench(): void {
  const panel = requiredElement("workbench-mode");
  panel.innerHTML = `
    <div class="workbench-layout">
      <section class="workbench-main">
        <div class="section-heading"><div><p class="eyebrow">Replayable derivation</p><h2>Linear-equation workbench</h2></div><button class="secondary-button" id="reset-workbench">Reset example</button></div>
        <p class="lede">Begin with an equation, then ask Symbolic Math to apply verified transformations. Every state below comes from the engine.</p>
        <div class="field-grid compact">${field("workbench-equation", "Initial equation", workbenchSteps.length ? workbenchSteps[0].input : workbenchEquation, "textarea", workbenchSteps.length > 0)}${field("workbench-variable", "Variable", workbenchVariable, "input", workbenchSteps.length > 0)}</div>
        <div class="derivation-chain">
          ${derivationLine(workbenchSteps.length ? workbenchSteps[0].input : workbenchEquation, "Given")}
          ${workbenchSteps.map(step => derivationLine(step.output, ruleLabel(step.rule))).join("")}
        </div>
        <div class="rule-actions">
          <p>Apply a verified whole-equation rule</p>
          <button data-workbench-rule="algebra.linear-equation.simplify-both-sides">Simplify both sides</button>
          <button data-workbench-rule="algebra.linear-equation.solve">Solve completely</button>
        </div>
      </section>
      <aside class="workbench-inspector"><p class="eyebrow">Derivation data</p><h2>${workbenchSteps.length} verified ${workbenchSteps.length === 1 ? "step" : "steps"}</h2><pre>${escapeHtml(JSON.stringify(workbenchSteps, null, 2))}</pre></aside>
    </div>`;
  const equationInput = panel.querySelector<HTMLTextAreaElement>("[data-field='workbench-equation']");
  equationInput?.addEventListener("input", () => { workbenchEquation = equationInput.value; renderWorkbench(); });
  const variableInput = panel.querySelector<HTMLInputElement>("[data-field='workbench-variable']");
  variableInput?.addEventListener("input", () => { workbenchVariable = variableInput.value; });
  panel.querySelectorAll<HTMLButtonElement>("[data-workbench-rule]").forEach(button => button.addEventListener("click", () => applyWorkbenchRule(button.dataset.workbenchRule as LinearEquationRule)));
  panel.querySelector<HTMLButtonElement>("#reset-workbench")?.addEventListener("click", () => { workbenchEquation = "3(x - 2) + 4 = 2x + 9"; workbenchVariable = "x"; workbenchSteps = []; renderWorkbench(); });
  panel.querySelectorAll<HTMLElement>("[data-render-latex]").forEach(preview => renderMath(preview, preview.dataset.renderLatex ?? ""));
}

function applyWorkbenchRule(rule: LinearEquationRule): void {
  const input = workbenchSteps.at(-1)?.output ?? workbenchEquation;
  const result = engine.applyLinearEquationRule({ equation: input, variable: workbenchVariable, rule });
  if (result.outcome === "proven" && result.resultLatex) workbenchSteps.push({ rule, input, output: result.resultLatex });
  else window.alert(result.diagnostics.map(item => `${item.code}: ${item.message}`).join("\n") || `Rule outcome: ${result.outcome}`);
  renderWorkbench();
}

function resultView(value: unknown): string {
  const latex = collectLatex(value);
  const diagnostics = (value as { diagnostics?: Array<{ code: string; message: string }> }).diagnostics ?? [];
  return `<div class="result-content">
    ${latex.length ? `<div class="rendered-results">${latex.slice(0, 4).map((item, index) => `<div><small>${index === 0 ? "Primary mathematical result" : "Related form"}</small><div data-render-latex="${escapeAttribute(item)}"></div></div>`).join("")}</div>` : ""}
    ${diagnostics.length ? `<div class="diagnostics"><h3>Diagnostics</h3>${diagnostics.map(item => `<article><code>${escapeHtml(item.code)}</code><p>${escapeHtml(item.message)}</p></article>`).join("")}</div>` : ""}
    <details open><summary>Structured API result</summary><pre>${escapeHtml(JSON.stringify(value, null, 2))}</pre></details>
  </div>`;
}

function collectLatex(value: unknown): string[] {
  const found: string[] = [];
  const visit = (item: unknown, key = "") => {
    if (typeof item === "string" && (key.toLowerCase().includes("latex") || key === "equation")) found.push(item);
    else if (item && typeof item === "object") Object.entries(item).forEach(([childKey, child]) => visit(child, childKey));
  };
  visit(value);
  return [...new Set(found)];
}

function historyView(): string {
  if (!history.length) return `<p class="history-empty">Completed experiments appear here and persist across reloads.</p>`;
  return `<div class="history-list">${history.map(entry => `<button data-history-id="${entry.id}"><span>${labels[entry.experiment.operation]}</span><code>${escapeHtml(entry.experiment.primary)}</code><time>${new Date(entry.createdAt).toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" })}</time></button>`).join("")}</div>`;
}

function loadNextExample(): void {
  const choices = examples[current.operation];
  const index = choices.findIndex(item => item.primary === current.primary && item.secondary === current.secondary);
  current = { operation: current.operation, ...choices[(index + 1) % choices.length] };
  lastResult = null;
  renderExperiment();
}

function bindModeTabs(): void {
  document.querySelectorAll<HTMLButtonElement>("[data-mode]").forEach(button => button.addEventListener("click", () => {
    document.querySelectorAll("[data-mode]").forEach(item => item.classList.toggle("active", item === button));
    requiredElement("experiment-mode").classList.toggle("hidden", button.dataset.mode !== "experiment");
    requiredElement("workbench-mode").classList.toggle("hidden", button.dataset.mode !== "workbench");
  }));
}

function field(name: string, label: string, value: string, kind: "input" | "textarea" = "input", disabled = false): string {
  const tag = kind === "textarea" ? `<textarea data-field="${name}" ${disabled ? "disabled" : ""}>${escapeHtml(value)}</textarea>` : `<input data-field="${name}" value="${escapeAttribute(value)}" ${disabled ? "disabled" : ""}/>`;
  return `<label class="field"><span>${label}</span>${tag}</label>`;
}

function selectRule(value: LinearEquationRule): string {
  return `<label class="field"><span>Rule</span><select data-field="rule"><option value="algebra.linear-equation.simplify-both-sides" ${value.endsWith("simplify-both-sides") ? "selected" : ""}>Simplify both sides</option><option value="algebra.linear-equation.solve" ${value.endsWith("solve") ? "selected" : ""}>Solve completely</option></select></label>`;
}

function mathCard(label: string, source: string): string { return `<div class="math-card"><small>${label} preview</small><div data-math-source="${label === "Input" ? "primary" : "secondary"}">${escapeHtml(source)}</div></div>`; }
function derivationLine(latex: string, reason: string): string { return `<article><div data-render-latex="${escapeAttribute(latex)}"></div><p>${escapeHtml(reason)}</p></article>`; }
function outcomeBadge(outcome: MathematicalOutcome): string { return `<span class="outcome outcome-${outcome}">${outcome}</span>`; }
function ruleLabel(rule: LinearEquationRule): string { return rule.endsWith("solve") ? "Solve completely" : "Simplify both sides"; }
function primaryLabel(operation: Operation): string { return operation === "compareNumeric" ? "Submitted value" : operation.includes("Equation") || operation === "solveLinear" ? "Equation" : "Expression"; }
function secondaryLabel(operation: Operation): string { return operation === "compareNumeric" ? "Expected value" : operation === "compareEquations" ? "Second equation" : "Second expression"; }
function operationHint(operation: Operation): string { return ({ normalize: "Canonical polynomial form", compareExpressions: "Polynomial identity", solveLinear: "Exact rational solutions", compareEquations: "Solution-set equality", applyEquationRule: "Verified transformation", compareNumeric: "Exact or tolerance policy", differentiate: "Symbolic derivative", integrate: "Symbolic antiderivative" })[operation]; }

function renderMath(element: HTMLElement, source: string): void {
  katex.render(source || "\\text{No input}", element, { displayMode: true, throwOnError: false, strict: false });
}

function readHistory(): HistoryEntry[] { try { return JSON.parse(localStorage.getItem("socrates-math-lab-history") ?? "[]") as HistoryEntry[]; } catch { return []; } }
function persistHistory(): void { localStorage.setItem("socrates-math-lab-history", JSON.stringify(history)); }
function requiredElement(id: string): HTMLElement { const element = document.getElementById(id); if (!element) throw new Error(`Missing #${id}`); return element; }
function escapeHtml(value: string): string { return value.replace(/[&<>"']/g, char => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;", "'": "&#039;" })[char] ?? char); }
function escapeAttribute(value: string): string { return escapeHtml(value).replace(/`/g, "&#096;"); }
