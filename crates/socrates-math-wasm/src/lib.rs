use socrates_math_app::MathEngine;
use socrates_math_protocol::SetBindingDto;
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

    #[wasm_bindgen(js_name = normalizeSetExpression)]
    pub fn normalize_set_expression(
        &self,
        source: &str,
        input_format: &str,
    ) -> Result<String, JsValue> {
        serde_json::to_string(&MathEngine::normalize_set_expression(source, input_format))
            .map_err(|error| JsValue::from_str(&error.to_string()))
    }

    #[wasm_bindgen(js_name = compareSetExpressions)]
    pub fn compare_set_expressions(
        &self,
        left_source: &str,
        right_source: &str,
        input_format: &str,
    ) -> Result<String, JsValue> {
        serde_json::to_string(&MathEngine::compare_set_expressions(
            left_source,
            right_source,
            input_format,
        ))
        .map_err(|error| JsValue::from_str(&error.to_string()))
    }

    #[wasm_bindgen(js_name = compareSetExpressionsInContext)]
    pub fn compare_set_expressions_in_context(
        &self,
        left_source: &str,
        right_source: &str,
        universe_source: &str,
        bindings_json: &str,
        input_format: &str,
    ) -> Result<String, JsValue> {
        let bindings = parse_set_bindings(bindings_json)?;

        serde_json::to_string(&MathEngine::compare_set_expressions_in_context(
            left_source,
            right_source,
            universe_source,
            &bindings,
            input_format,
        ))
        .map_err(|error| JsValue::from_str(&error.to_string()))
    }

    #[wasm_bindgen(js_name = evaluateSetStatement)]
    pub fn evaluate_set_statement(
        &self,
        source: &str,
        input_format: &str,
    ) -> Result<String, JsValue> {
        serde_json::to_string(&MathEngine::evaluate_set_statement(source, input_format))
            .map_err(|error| JsValue::from_str(&error.to_string()))
    }

    #[wasm_bindgen(js_name = evaluateSetCardinality)]
    pub fn evaluate_set_cardinality(
        &self,
        source: &str,
        input_format: &str,
    ) -> Result<String, JsValue> {
        serde_json::to_string(&MathEngine::evaluate_set_cardinality(source, input_format))
            .map_err(|error| JsValue::from_str(&error.to_string()))
    }

    #[wasm_bindgen(js_name = evaluateRelationFrom)]
    pub fn evaluate_relation_from(
        &self,
        relation_source: &str,
        domain_source: &str,
        codomain_source: &str,
        input_format: &str,
    ) -> Result<String, JsValue> {
        serde_json::to_string(&MathEngine::evaluate_relation_from(
            relation_source,
            domain_source,
            codomain_source,
            input_format,
        ))
        .map_err(|error| JsValue::from_str(&error.to_string()))
    }

    #[wasm_bindgen(js_name = evaluateFunctionFrom)]
    pub fn evaluate_function_from(
        &self,
        relation_source: &str,
        domain_source: &str,
        codomain_source: &str,
        input_format: &str,
    ) -> Result<String, JsValue> {
        serde_json::to_string(&MathEngine::evaluate_function_from(
            relation_source,
            domain_source,
            codomain_source,
            input_format,
        ))
        .map_err(|error| JsValue::from_str(&error.to_string()))
    }

    #[wasm_bindgen(js_name = evaluateRelationProperty)]
    pub fn evaluate_relation_property(
        &self,
        relation_source: &str,
        set_source: &str,
        property: &str,
        input_format: &str,
    ) -> Result<String, JsValue> {
        serde_json::to_string(&MathEngine::evaluate_relation_property(
            relation_source,
            set_source,
            property,
            input_format,
        ))
        .map_err(|error| JsValue::from_str(&error.to_string()))
    }

    #[wasm_bindgen(js_name = evaluateRelationDomain)]
    pub fn evaluate_relation_domain(
        &self,
        relation_source: &str,
        input_format: &str,
    ) -> Result<String, JsValue> {
        serde_json::to_string(&MathEngine::evaluate_relation_domain(
            relation_source,
            input_format,
        ))
        .map_err(|error| JsValue::from_str(&error.to_string()))
    }

    #[wasm_bindgen(js_name = evaluateRelationRange)]
    pub fn evaluate_relation_range(
        &self,
        relation_source: &str,
        input_format: &str,
    ) -> Result<String, JsValue> {
        serde_json::to_string(&MathEngine::evaluate_relation_range(
            relation_source,
            input_format,
        ))
        .map_err(|error| JsValue::from_str(&error.to_string()))
    }

    #[wasm_bindgen(js_name = evaluateRelationInverse)]
    pub fn evaluate_relation_inverse(
        &self,
        relation_source: &str,
        input_format: &str,
    ) -> Result<String, JsValue> {
        serde_json::to_string(&MathEngine::evaluate_relation_inverse(
            relation_source,
            input_format,
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

    #[wasm_bindgen(js_name = listApplicableMathExpressionRules)]
    pub fn list_applicable_math_expression_rules(
        &self,
        source: &str,
        input_format: &str,
        variable: &str,
        target_json: Option<String>,
    ) -> Result<String, JsValue> {
        let target = parse_optional_rule_target(target_json)?;

        serde_json::to_string(&MathEngine::list_applicable_math_expression_rules(
            source,
            input_format,
            variable,
            target,
        ))
        .map_err(|error| JsValue::from_str(&error.to_string()))
    }

    #[wasm_bindgen(js_name = applyMathExpressionRule)]
    pub fn apply_math_expression_rule(
        &self,
        source: &str,
        input_format: &str,
        variable: &str,
        rule: &str,
        target_json: Option<String>,
    ) -> Result<String, JsValue> {
        let target = parse_optional_rule_target(target_json)?;

        serde_json::to_string(&MathEngine::apply_math_expression_rule(
            source,
            input_format,
            variable,
            rule,
            target,
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

fn parse_optional_rule_target(
    target_json: Option<String>,
) -> Result<Option<socrates_math_protocol::RuleTargetDto>, JsValue> {
    target_json
        .map(|json| {
            serde_json::from_str(&json).map_err(|error| JsValue::from_str(&error.to_string()))
        })
        .transpose()
}

fn parse_set_bindings(bindings_json: &str) -> Result<Vec<SetBindingDto>, JsValue> {
    serde_json::from_str(bindings_json)
        .map_err(|error| JsValue::from_str(&format!("invalid set bindings JSON: {error}")))
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
