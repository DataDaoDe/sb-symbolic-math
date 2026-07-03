use socrates_math_app::MathEngine;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmMathEngine;

#[wasm_bindgen]
impl WasmMathEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self
    }

    #[wasm_bindgen(js_name = solveLinearEquation)]
    pub fn solve_linear_equation(&self, source: &str, variable: &str) -> Result<String, JsValue> {
        serde_json::to_string(&MathEngine::solve_linear_equation(source, variable))
            .map_err(|error| JsValue::from_str(&error.to_string()))
    }

    #[wasm_bindgen(js_name = normalizeMathExpression)]
    pub fn normalize_math_expression(
        &self,
        source: &str,
        input_format: &str,
        variable: &str,
    ) -> Result<String, JsValue> {
        serde_json::to_string(&MathEngine::normalize_math_expression(
            source,
            input_format,
            variable,
        ))
        .map_err(|error| JsValue::from_str(&error.to_string()))
    }

    #[wasm_bindgen(js_name = compareMathExpressions)]
    pub fn compare_math_expressions(
        &self,
        left_source: &str,
        right_source: &str,
        input_format: &str,
        variable: &str,
    ) -> Result<String, JsValue> {
        serde_json::to_string(&MathEngine::compare_math_expressions(
            left_source,
            right_source,
            input_format,
            variable,
        ))
        .map_err(|error| JsValue::from_str(&error.to_string()))
    }

    #[wasm_bindgen(js_name = compareNumericAnswer)]
    pub fn compare_numeric_answer(
        &self,
        submitted_source: &str,
        expected_source: &str,
        input_format: &str,
        tolerance: f64,
    ) -> Result<String, JsValue> {
        serde_json::to_string(&MathEngine::compare_numeric_answer(
            submitted_source,
            expected_source,
            input_format,
            tolerance,
        ))
        .map_err(|error| JsValue::from_str(&error.to_string()))
    }

    #[wasm_bindgen(js_name = differentiateMathExpression)]
    pub fn differentiate_math_expression(
        &self,
        source: &str,
        input_format: &str,
        variable: &str,
    ) -> Result<String, JsValue> {
        serde_json::to_string(&MathEngine::differentiate_math_expression(
            source,
            input_format,
            variable,
        ))
        .map_err(|error| JsValue::from_str(&error.to_string()))
    }

    #[wasm_bindgen(js_name = integrateMathExpression)]
    pub fn integrate_math_expression(
        &self,
        source: &str,
        input_format: &str,
        variable: &str,
    ) -> Result<String, JsValue> {
        serde_json::to_string(&MathEngine::integrate_math_expression(
            source,
            input_format,
            variable,
        ))
        .map_err(|error| JsValue::from_str(&error.to_string()))
    }

    #[wasm_bindgen(js_name = compareEquationSolutionSets)]
    pub fn compare_equation_solution_sets(
        &self,
        left_source: &str,
        right_source: &str,
        variable: &str,
    ) -> Result<String, JsValue> {
        serde_json::to_string(&MathEngine::compare_equation_solution_sets(
            left_source,
            right_source,
            variable,
        ))
        .map_err(|error| JsValue::from_str(&error.to_string()))
    }
}

impl Default for WasmMathEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_solve_response_as_json() {
        let engine = WasmMathEngine::new();
        let json = engine
            .solve_linear_equation("3(x - 2) + 4 = 2x + 9", "x")
            .unwrap();

        assert!(json.contains("\"outcome\":\"proven\""));
        assert!(json.contains("\"value\":\"11\""));
    }

    #[test]
    fn serializes_expression_comparison_response_as_json() {
        let engine = WasmMathEngine::new();
        let json = engine
            .compare_math_expressions("3(x - 2) + 4", "3x - 2", "latex", "x")
            .unwrap();

        assert!(json.contains("\"outcome\":\"proven\""));
        assert!(json.contains("\"equal\":true"));
    }
}
