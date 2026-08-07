import assert from 'node:assert/strict'
import test from 'node:test'
import { generateExactRationalTicks, parseExactRationalLiteral } from '../dist/index.js'

const integer = (value) => ({ kind: 'integer', value })
const rational = (numerator, denominator) => ({ kind: 'rational', numerator, denominator })

test('generates canonical rational ticks without floating-point accumulation', () => {
  const result = generateExactRationalTicks({
    minimum: rational('-1', '2'),
    maximum: rational('1', '2'),
    step: rational('1', '4'),
  })

  assert.equal(result.outcome, 'proven')
  assert.deepEqual(result.ticks.map((tick) => tick.value), [
    rational('-1', '2'),
    rational('-1', '4'),
    integer('0'),
    rational('1', '4'),
    rational('1', '2'),
  ])
  assert.deepEqual(result.ticks.map((tick) => tick.label), ['-1/2', '-1/4', '0', '1/4', '1/2'])
  assert.equal(result.ticks[0].latex, '-\\frac{1}{2}')
})

test('normalizes authored values before enumeration', () => {
  const result = generateExactRationalTicks({
    minimum: rational('2', '4'),
    maximum: rational('6', '4'),
    step: rational('2', '4'),
  })

  assert.deepEqual(result.ticks.map((tick) => tick.value), [
    rational('1', '2'),
    integer('1'),
    rational('3', '2'),
  ])
})

test('rejects invalid and unbounded grid requests', () => {
  assert.equal(generateExactRationalTicks({
    minimum: integer('0'),
    maximum: integer('1'),
    step: integer('0'),
  }).diagnostics[0].code, 'grid.exact.nonpositive_step')

  assert.equal(generateExactRationalTicks({
    minimum: integer('0'),
    maximum: integer('10'),
    step: rational('1', '10'),
    maximumTickCount: 10,
  }).diagnostics[0].code, 'grid.exact.tick_limit_exceeded')
})

test('parses and normalizes authored exact literals', () => {
  assert.deepEqual(parseExactRationalLiteral(' 6/-8 ').value, rational('-3', '4'))
  assert.deepEqual(parseExactRationalLiteral('5').value, integer('5'))
  assert.deepEqual(parseExactRationalLiteral('12.305').value, rational('2461', '200'))
  assert.deepEqual(parseExactRationalLiteral('0.0400').value, rational('1', '25'))
  assert.equal(parseExactRationalLiteral('1/0').outcome, 'undefined')
})
