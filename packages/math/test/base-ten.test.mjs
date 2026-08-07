import assert from 'node:assert/strict'
import test from 'node:test'
import { createMathEngine } from '../dist/index.js'

const integer = (value) => ({ kind: 'integer', value })

test('maps semantic base-ten operations through the public engine', async () => {
  const binding = {
    decomposeBaseTen(valueJson, minimumExponent, maximumExponent) {
      assert.deepEqual(JSON.parse(valueJson), integer('40306'))
      assert.equal(minimumExponent, 0)
      assert.equal(maximumExponent, 4)
      return JSON.stringify({ outcome: 'proven', value: integer('40306'), places: [
        { exponent: 4, coefficient: 4 },
        { exponent: 3, coefficient: 0 },
        { exponent: 2, coefficient: 3 },
        { exponent: 1, coefficient: 0 },
        { exponent: 0, coefficient: 6 },
      ], diagnostics: [] })
    },
    composeBaseTen(placesJson) {
      const places = JSON.parse(placesJson)
      return JSON.stringify({ outcome: 'proven', value: integer('40306'), places, diagnostics: [] })
    },
    compareBaseTen(expectedJson, submittedJson) {
      const expected = JSON.parse(expectedJson)
      const submitted = JSON.parse(submittedJson)
      return JSON.stringify({ outcome: 'proven', relation: 'number.base-ten-place-value', equal: true, expected_normalized: expected, submitted_normalized: expected, expected_places: submitted, submitted_places: submitted, diagnostics: [] })
    },
  }
  const engine = await createMathEngine({ wasmEngine: binding })
  const decomposition = engine.decomposeBaseTen({ value: integer('40306'), minimumExponent: 0, maximumExponent: 4 })
  assert.equal(decomposition.outcome, 'proven')
  assert.equal(decomposition.places[1].coefficient, 0)
  assert.equal(engine.composeBaseTen({ places: decomposition.places }).value.value, '40306')
  assert.equal(engine.compareBaseTen({ value: integer('40306'), submitted: decomposition.places, minimumExponent: 0, maximumExponent: 4 }).equal, true)
})

test('reports unsupported base-ten capability explicitly', async () => {
  const engine = await createMathEngine({ wasmEngine: {} })
  const result = engine.decomposeBaseTen({ value: integer('1'), minimumExponent: 0, maximumExponent: 0 })
  assert.equal(result.outcome, 'unknown')
  assert.equal(result.diagnostics[0].code, 'Engine.UnsupportedOperation')
})
