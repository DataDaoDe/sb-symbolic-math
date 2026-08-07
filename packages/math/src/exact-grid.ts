import type { ExactValue, MathDiagnostic, MathematicalOutcome } from './types.js'

export interface GenerateExactRationalTicksRequest {
  readonly minimum: ExactValue
  readonly maximum: ExactValue
  readonly step: ExactValue
  /** Safety bound for authored or generated material. Defaults to 200; maximum 1,000. */
  readonly maximumTickCount?: number
}

export interface ExactRationalTick {
  readonly value: ExactValue
  readonly label: string
  readonly latex: string
  /** Rendering projection only. Mathematical consumers must use `value`. */
  readonly approximate: number
}

export interface GenerateExactRationalTicksResult {
  readonly outcome: Extract<MathematicalOutcome, 'proven' | 'undefined'>
  readonly ticks: readonly ExactRationalTick[]
  readonly diagnostics: readonly MathDiagnostic[]
}

export interface ParseExactRationalLiteralResult {
  readonly outcome: Extract<MathematicalOutcome, 'proven' | 'undefined'>
  readonly value: ExactValue | null
  readonly diagnostics: readonly MathDiagnostic[]
}

interface Fraction {
  readonly numerator: bigint
  readonly denominator: bigint
}

const DEFAULT_MAXIMUM_TICK_COUNT = 200
const MAXIMUM_ALLOWED_TICK_COUNT = 1_000

function gcd(left: bigint, right: bigint): bigint {
  let a = left < 0n ? -left : left
  let b = right < 0n ? -right : right
  while (b !== 0n) [a, b] = [b, a % b]
  return a
}

function normalize(numerator: bigint, denominator: bigint): Fraction {
  if (denominator === 0n) throw new RangeError('Exact rational denominator cannot be zero')
  const sign = denominator < 0n ? -1n : 1n
  const divisor = gcd(numerator, denominator)
  return {
    numerator: (numerator / divisor) * sign,
    denominator: (denominator / divisor) * sign,
  }
}

function fraction(value: ExactValue): Fraction {
  return value.kind === 'integer'
    ? normalize(BigInt(value.value), 1n)
    : normalize(BigInt(value.numerator), BigInt(value.denominator))
}

function exactValue(value: Fraction): ExactValue {
  return value.denominator === 1n
    ? { kind: 'integer', value: value.numerator.toString() }
    : {
        kind: 'rational',
        numerator: value.numerator.toString(),
        denominator: value.denominator.toString(),
      }
}

function compare(left: Fraction, right: Fraction): number {
  const difference = left.numerator * right.denominator - right.numerator * left.denominator
  return difference < 0n ? -1 : difference > 0n ? 1 : 0
}

function add(left: Fraction, right: Fraction): Fraction {
  return normalize(
    left.numerator * right.denominator + right.numerator * left.denominator,
    left.denominator * right.denominator,
  )
}

function render(value: Fraction): Pick<ExactRationalTick, 'label' | 'latex'> {
  if (value.denominator === 1n) {
    const label = value.numerator.toString()
    return { label, latex: label }
  }
  const label = `${value.numerator}/${value.denominator}`
  const latex = value.numerator < 0n
    ? `-\\frac{${-value.numerator}}{${value.denominator}}`
    : `\\frac{${value.numerator}}{${value.denominator}}`
  return { label, latex }
}

function failure(code: string, message: string): GenerateExactRationalTicksResult {
  return { outcome: 'undefined', ticks: [], diagnostics: [{ code, message }] }
}

/** Parses and canonically normalizes an authored integer, finite decimal, or fraction literal. */
export function parseExactRationalLiteral(source: string): ParseExactRationalLiteralResult {
  const trimmed = source.trim()
  const decimal = /^(-?)(\d+)\.(\d+)$/u.exec(trimmed)
  if (decimal) {
    const [, sign, whole, fractional] = decimal
    const scale = 10n ** BigInt(fractional!.length)
    const magnitude = BigInt(`${whole}${fractional}`)
    return {
      outcome: 'proven',
      value: exactValue(normalize(sign === '-' ? -magnitude : magnitude, scale)),
      diagnostics: [],
    }
  }
  const parts = trimmed.split('/')
  if (parts.length > 2 || !/^-?\d+$/u.test(parts[0] ?? '') || (parts.length === 2 && !/^-?\d+$/u.test(parts[1] ?? ''))) {
    return { outcome: 'undefined', value: null, diagnostics: [{ code: 'exact.invalid_literal', message: 'Expected an integer, finite decimal, or fraction such as 12.305 or -3/4.' }] }
  }
  try {
    const normalized = normalize(BigInt(parts[0]!), parts.length === 2 ? BigInt(parts[1]!) : 1n)
    return { outcome: 'proven', value: exactValue(normalized), diagnostics: [] }
  } catch {
    return { outcome: 'undefined', value: null, diagnostics: [{ code: 'exact.invalid_literal', message: 'An exact fraction must have a nonzero denominator.' }] }
  }
}

/**
 * Enumerates a finite exact-rational axis grid without floating-point accumulation.
 * The returned number is only a projection for renderers; exact values remain canonical.
 */
export function generateExactRationalTicks(
  request: GenerateExactRationalTicksRequest,
): GenerateExactRationalTicksResult {
  let minimum: Fraction
  let maximum: Fraction
  let step: Fraction
  try {
    minimum = fraction(request.minimum)
    maximum = fraction(request.maximum)
    step = fraction(request.step)
  } catch {
    return failure('grid.exact.invalid_value', 'Grid bounds and step must be valid exact integers or rationals.')
  }

  if (compare(step, normalize(0n, 1n)) <= 0) {
    return failure('grid.exact.nonpositive_step', 'The exact grid step must be positive.')
  }
  if (compare(minimum, maximum) > 0) {
    return failure('grid.exact.reversed_bounds', 'The exact grid minimum must not exceed its maximum.')
  }

  const maximumTickCount = request.maximumTickCount ?? DEFAULT_MAXIMUM_TICK_COUNT
  if (!Number.isInteger(maximumTickCount) || maximumTickCount < 1 || maximumTickCount > MAXIMUM_ALLOWED_TICK_COUNT) {
    return failure(
      'grid.exact.invalid_tick_limit',
      `The maximum tick count must be an integer from 1 to ${MAXIMUM_ALLOWED_TICK_COUNT}.`,
    )
  }

  const ticks: ExactRationalTick[] = []
  for (let current = minimum; compare(current, maximum) <= 0; current = add(current, step)) {
    if (ticks.length === maximumTickCount) {
      return failure(
        'grid.exact.tick_limit_exceeded',
        `The exact grid requires more than ${maximumTickCount} ticks. Increase the step or narrow the bounds.`,
      )
    }
    const labels = render(current)
    ticks.push({
      value: exactValue(current),
      ...labels,
      approximate: Number(current.numerator) / Number(current.denominator),
    })
  }

  return { outcome: 'proven', ticks, diagnostics: [] }
}
