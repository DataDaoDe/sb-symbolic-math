import type { MathematicalOutcome } from "@socrates/math";

export type SetTheoryInputFormat = "latex";

export type SetTheoryExerciseKind =
  | "set_literal_normalization"
  | "set_expression"
  | "set_statement"
  | "set_membership"
  | "set_subset"
  | "set_cardinality"
  | "set_power_set"
  | "set_cartesian_product"
  | "set_builder"
  | "set_identity"
  | "venn_region"
  | "relation"
  | "function"
  | "indexed_family"
  | "proof_step";

export type SetTheoryEngineCapability =
  | "parse.set.latex"
  | "normalize.finite-set"
  | "compare.set.extensional-equality"
  | "evaluate.set.statement"
  | "evaluate.set.operation"
  | "evaluate.set.cardinality"
  | "evaluate.power-set"
  | "evaluate.cartesian-product"
  | "evaluate.relation.properties"
  | "evaluate.function.properties"
  | "compare.logical-equivalence"
  | "list.set.rules"
  | "apply.set.rule";

export interface NaiveSetTheoryExerciseSpec {
  kind: SetTheoryExerciseKind;
  title: string;
  prompt: string;
  acceptedStudentInputs: readonly string[];
  rejectedStudentInputs: readonly string[];
  expectedOutcome: MathematicalOutcome;
  expectedRelation: string;
  expectedCanonicalLatex?: string;
  expectedBoolean?: boolean;
  requiredCapabilities: readonly SetTheoryEngineCapability[];
  pedagogicFeedback: readonly string[];
  sourceMotivation: readonly string[];
}

export const naiveSetTheoryConsumerExercises = [
  {
    kind: "set_literal_normalization",
    title: "Normalize a finite roster set",
    prompt: "Write the set {3, 1, 2, 2} in canonical form.",
    acceptedStudentInputs: ["\\{1,2,3\\}", "\\{1, 2, 3\\}"],
    rejectedStudentInputs: ["\\{1,2,2,3\\}", "\\{3,2,1,2\\}"],
    expectedOutcome: "proven",
    expectedRelation: "set.extensional_equal",
    expectedCanonicalLatex: "\\{1,2,3\\}",
    requiredCapabilities: [
      "parse.set.latex",
      "normalize.finite-set",
      "compare.set.extensional-equality",
    ],
    pedagogicFeedback: [
      "Sets ignore repeated elements.",
      "Sets are equal extensionally, so display order does not matter.",
    ],
    sourceMotivation: [
      "intro textbooks emphasize extensional equality, order irrelevance, and duplicate irrelevance",
    ],
  },
  {
    kind: "set_membership",
    title: "Evaluate a membership statement",
    prompt: "Is 2 an element of {1, 2, 3}?",
    acceptedStudentInputs: ["2 \\in \\{1,2,3\\}", "\\mathrm{true}"],
    rejectedStudentInputs: ["2 \\notin \\{1,2,3\\}", "\\mathrm{false}"],
    expectedOutcome: "proven",
    expectedRelation: "logic.truth",
    expectedBoolean: true,
    requiredCapabilities: ["parse.set.latex", "evaluate.set.statement"],
    pedagogicFeedback: [
      "Membership asks whether one object occurs as an element of the set.",
    ],
    sourceMotivation: ["membership is the primitive relation of naive set theory"],
  },
  {
    kind: "set_statement",
    title: "Distinguish element from singleton subset",
    prompt: "Is {2} an element of {1, 2, 3}?",
    acceptedStudentInputs: ["\\{2\\} \\notin \\{1,2,3\\}", "\\mathrm{false}"],
    rejectedStudentInputs: ["\\{2\\} \\in \\{1,2,3\\}", "\\mathrm{true}"],
    expectedOutcome: "proven",
    expectedRelation: "logic.truth",
    expectedBoolean: false,
    requiredCapabilities: ["parse.set.latex", "evaluate.set.statement"],
    pedagogicFeedback: [
      "The number 2 is an element, but the set {2} is a different object.",
    ],
    sourceMotivation: [
      "textbooks commonly contrast an element with the singleton containing it",
    ],
  },
  {
    kind: "set_subset",
    title: "Evaluate subset inclusion",
    prompt: "Decide whether {1, 2} is a subset of {1, 2, 3}.",
    acceptedStudentInputs: ["\\{1,2\\} \\subseteq \\{1,2,3\\}", "\\mathrm{true}"],
    rejectedStudentInputs: ["\\{1,2\\} \\nsubseteq \\{1,2,3\\}", "\\mathrm{false}"],
    expectedOutcome: "proven",
    expectedRelation: "logic.truth",
    expectedBoolean: true,
    requiredCapabilities: ["parse.set.latex", "evaluate.set.statement"],
    pedagogicFeedback: [
      "A set A is a subset of B when every element of A is also an element of B.",
    ],
    sourceMotivation: ["subset checking follows directly from membership"],
  },
  {
    kind: "set_expression",
    title: "Compute a union",
    prompt: "Simplify {1, 2} union {2, 3}.",
    acceptedStudentInputs: ["\\{1,2,3\\}", "\\{3,2,1\\}"],
    rejectedStudentInputs: ["\\{2\\}", "\\{1,2,2,3\\}"],
    expectedOutcome: "proven",
    expectedRelation: "set.extensional_equal",
    expectedCanonicalLatex: "\\{1,2,3\\}",
    requiredCapabilities: [
      "parse.set.latex",
      "evaluate.set.operation",
      "normalize.finite-set",
      "compare.set.extensional-equality",
    ],
    pedagogicFeedback: [
      "The union contains elements that are in either set.",
      "The common element 2 appears once in the resulting set.",
    ],
    sourceMotivation: ["union is a fundamental operation in naive set theory"],
  },
  {
    kind: "set_expression",
    title: "Compute an intersection",
    prompt: "Simplify {1, 2, 3} intersect {2, 3, 4}.",
    acceptedStudentInputs: ["\\{2,3\\}", "\\{3,2\\}"],
    rejectedStudentInputs: ["\\{1,2,3,4\\}", "\\{1,4\\}"],
    expectedOutcome: "proven",
    expectedRelation: "set.extensional_equal",
    expectedCanonicalLatex: "\\{2,3\\}",
    requiredCapabilities: [
      "parse.set.latex",
      "evaluate.set.operation",
      "normalize.finite-set",
      "compare.set.extensional-equality",
    ],
    pedagogicFeedback: ["The intersection contains exactly the shared elements."],
    sourceMotivation: ["intersection is paired with union in basic set exercises"],
  },
  {
    kind: "set_expression",
    title: "Compute relative complement",
    prompt: "Simplify {1, 2, 3} \\ {2, 4}.",
    acceptedStudentInputs: ["\\{1,3\\}", "\\{3,1\\}"],
    rejectedStudentInputs: ["\\{1,3,4\\}", "\\{2\\}"],
    expectedOutcome: "proven",
    expectedRelation: "set.extensional_equal",
    expectedCanonicalLatex: "\\{1,3\\}",
    requiredCapabilities: [
      "parse.set.latex",
      "evaluate.set.operation",
      "normalize.finite-set",
      "compare.set.extensional-equality",
    ],
    pedagogicFeedback: [
      "A set difference A \\ B keeps elements of A that are not elements of B.",
    ],
    sourceMotivation: ["relative complement appears with union and intersection"],
  },
  {
    kind: "set_cardinality",
    title: "Compute cardinality of a finite set",
    prompt: "What is |{a, b, b, c}|?",
    acceptedStudentInputs: ["3"],
    rejectedStudentInputs: ["4"],
    expectedOutcome: "proven",
    expectedRelation: "number.equal",
    expectedCanonicalLatex: "3",
    requiredCapabilities: [
      "parse.set.latex",
      "normalize.finite-set",
      "evaluate.set.cardinality",
    ],
    pedagogicFeedback: ["Repeated roster entries do not increase cardinality."],
    sourceMotivation: ["finite cardinality exercises test extensional set meaning"],
  },
  {
    kind: "set_power_set",
    title: "List a power set",
    prompt: "List the power set of {a, b}.",
    acceptedStudentInputs: [
      "\\{\\varnothing,\\{a\\},\\{b\\},\\{a,b\\}\\}",
      "\\{\\{a,b\\},\\{a\\},\\{b\\},\\varnothing\\}",
    ],
    rejectedStudentInputs: ["\\{a,b\\}", "\\{\\{a\\},\\{b\\}\\}"],
    expectedOutcome: "proven",
    expectedRelation: "set.extensional_equal",
    expectedCanonicalLatex: "\\{\\varnothing,\\{a\\},\\{b\\},\\{a,b\\}\\}",
    requiredCapabilities: [
      "parse.set.latex",
      "evaluate.power-set",
      "normalize.finite-set",
      "compare.set.extensional-equality",
    ],
    pedagogicFeedback: [
      "The power set contains every subset, including the empty set and the original set.",
    ],
    sourceMotivation: ["power sets are standard after subsets"],
  },
  {
    kind: "set_cartesian_product",
    title: "List a Cartesian product",
    prompt: "List {1, 2} x {a, b}.",
    acceptedStudentInputs: [
      "\\{(1,a),(1,b),(2,a),(2,b)\\}",
      "\\{(2,b),(1,a),(2,a),(1,b)\\}",
    ],
    rejectedStudentInputs: ["\\{(a,1),(b,1),(a,2),(b,2)\\}", "\\{1,2,a,b\\}"],
    expectedOutcome: "proven",
    expectedRelation: "set.extensional_equal",
    expectedCanonicalLatex: "\\{(1,a),(1,b),(2,a),(2,b)\\}",
    requiredCapabilities: [
      "parse.set.latex",
      "evaluate.cartesian-product",
      "normalize.finite-set",
      "compare.set.extensional-equality",
    ],
    pedagogicFeedback: [
      "Cartesian products contain ordered pairs.",
      "The first coordinate comes from the first set and the second coordinate from the second set.",
    ],
    sourceMotivation: [
      "relations and functions are normally introduced as subsets of Cartesian products",
    ],
  },
  {
    kind: "set_builder",
    title: "Evaluate bounded set-builder notation",
    prompt: "Simplify {x in {1,2,3,4} | x is even}.",
    acceptedStudentInputs: ["\\{2,4\\}", "\\{4,2\\}"],
    rejectedStudentInputs: ["\\{1,3\\}", "\\{2\\}"],
    expectedOutcome: "proven",
    expectedRelation: "set.extensional_equal",
    expectedCanonicalLatex: "\\{2,4\\}",
    requiredCapabilities: [
      "parse.set.latex",
      "evaluate.set.statement",
      "evaluate.set.operation",
      "normalize.finite-set",
      "compare.set.extensional-equality",
    ],
    pedagogicFeedback: [
      "Bounded set-builder notation filters a known domain by a predicate.",
      "The first supported predicate families are equality, inequality, numeric comparisons, membership, even or odd, and divisibility.",
    ],
    sourceMotivation: [
      "bounded comprehension is safer pedagogically than unrestricted comprehension",
    ],
  },
  {
    kind: "set_identity",
    title: "Verify De Morgan's law on a finite universe",
    prompt:
      "Let U = {1,2,3,4}, A = {1,2}, and B = {2,3}. Verify (A union B)^c = A^c intersect B^c.",
    acceptedStudentInputs: ["\\{4\\}", "\\mathrm{true}"],
    rejectedStudentInputs: ["\\{1,2,3\\}", "\\mathrm{false}"],
    expectedOutcome: "proven",
    expectedRelation: "logic.equivalent",
    expectedBoolean: true,
    requiredCapabilities: [
      "parse.set.latex",
      "evaluate.set.operation",
      "compare.logical-equivalence",
      "list.set.rules",
      "apply.set.rule",
    ],
    pedagogicFeedback: [
      "Complements must be interpreted relative to the declared universe.",
      "Both sides reduce to the same finite set.",
    ],
    sourceMotivation: ["Boolean algebra of sets includes De Morgan identities"],
  },
  {
    kind: "venn_region",
    title: "Translate a Venn region to notation",
    prompt: "Write the region inside A but outside B.",
    acceptedStudentInputs: ["A \\setminus B", "A \\cap B^{c}"],
    rejectedStudentInputs: ["A \\cap B", "B \\setminus A"],
    expectedOutcome: "proven",
    expectedRelation: "set.expression_equivalent",
    requiredCapabilities: [
      "parse.set.latex",
      "compare.logical-equivalence",
      "list.set.rules",
      "apply.set.rule",
    ],
    pedagogicFeedback: [
      "The phrase 'inside A but outside B' means elements in A and not in B.",
    ],
    sourceMotivation: ["Venn diagram translation is common in beginner exercises"],
  },
  {
    kind: "relation",
    title: "Classify a relation property",
    prompt: "On {1,2,3}, is <= reflexive?",
    acceptedStudentInputs: ["\\mathrm{true}", "\\text{yes}"],
    rejectedStudentInputs: ["\\mathrm{false}", "\\text{no}"],
    expectedOutcome: "proven",
    expectedRelation: "logic.truth",
    expectedBoolean: true,
    requiredCapabilities: [
      "parse.set.latex",
      "evaluate.relation.properties",
      "evaluate.set.statement",
    ],
    pedagogicFeedback: [
      "A relation is reflexive when every element is related to itself.",
    ],
    sourceMotivation: ["relations follow naturally after Cartesian products"],
  },
  {
    kind: "function",
    title: "Decide whether a relation is a function",
    prompt: "Is {(1,a), (2,a), (2,b)} a function from {1,2} to {a,b}?",
    acceptedStudentInputs: ["\\mathrm{false}", "\\text{no}"],
    rejectedStudentInputs: ["\\mathrm{true}", "\\text{yes}"],
    expectedOutcome: "proven",
    expectedRelation: "logic.truth",
    expectedBoolean: false,
    requiredCapabilities: [
      "parse.set.latex",
      "evaluate.function.properties",
      "evaluate.set.statement",
    ],
    pedagogicFeedback: [
      "A function assigns each domain element exactly one output.",
      "The input 2 is assigned both a and b.",
    ],
    sourceMotivation: ["functions are typically treated as special relations"],
  },
  {
    kind: "indexed_family",
    title: "Evaluate an indexed union",
    prompt: "Let A_i = {i, i+1} for i in {1,2,3}. Find union A_i.",
    acceptedStudentInputs: ["\\{1,2,3,4\\}", "\\{4,3,2,1\\}"],
    rejectedStudentInputs: ["\\{1,2,3\\}", "\\{2,3\\}"],
    expectedOutcome: "proven",
    expectedRelation: "set.extensional_equal",
    expectedCanonicalLatex: "\\{1,2,3,4\\}",
    requiredCapabilities: [
      "parse.set.latex",
      "evaluate.set.operation",
      "normalize.finite-set",
      "compare.set.extensional-equality",
    ],
    pedagogicFeedback: [
      "An indexed union contains elements that occur in at least one member of the family.",
    ],
    sourceMotivation: [
      "indexed unions and intersections prepare students for more advanced mathematics",
    ],
  },
  {
    kind: "proof_step",
    title: "Apply extensionality as a proof step",
    prompt: "Prove {1,2} union {2,3} = {1,2,3} by showing mutual inclusion.",
    acceptedStudentInputs: [
      "\\{1,2\\}\\cup\\{2,3\\}\\subseteq\\{1,2,3\\}\\text{ and }\\{1,2,3\\}\\subseteq\\{1,2\\}\\cup\\{2,3\\}",
    ],
    rejectedStudentInputs: ["\\text{They look the same.}"],
    expectedOutcome: "proven",
    expectedRelation: "set.extensional_equal",
    requiredCapabilities: [
      "parse.set.latex",
      "evaluate.set.statement",
      "list.set.rules",
      "apply.set.rule",
      "compare.set.extensional-equality",
    ],
    pedagogicFeedback: [
      "Extensional equality can be proved by showing each set is a subset of the other.",
      "Each subset claim reduces to membership reasoning.",
    ],
    sourceMotivation: [
      "proof-oriented courses ask students to justify set equalities, not only compute examples",
    ],
  },
] as const satisfies readonly NaiveSetTheoryExerciseSpec[];

export const naiveSetTheoryRequiredCapabilities = Array.from(
  new Set(
    naiveSetTheoryConsumerExercises.flatMap(
      (exercise) => exercise.requiredCapabilities,
    ),
  ),
).sort();

export function exercisesRequiringCapability(
  capability: SetTheoryEngineCapability,
): readonly NaiveSetTheoryExerciseSpec[] {
  const exercises: readonly NaiveSetTheoryExerciseSpec[] =
    naiveSetTheoryConsumerExercises;

  return exercises.filter((exercise) =>
    exercise.requiredCapabilities.includes(capability),
  );
}
