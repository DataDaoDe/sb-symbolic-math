use socrates_math_app::MathEngine;
use socrates_math_protocol::{
    ExactQuantityDto, ExactValueDto, RealDomainDto, RealFunctionSourceDto, SetBindingDto,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmMathEngine;

#[wasm_bindgen]
impl WasmMathEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self
    }

    #[wasm_bindgen(js_name = protocolManifest)]
    pub fn protocol_manifest(&self) -> Result<String, JsValue> {
        serde_json::to_string(&MathEngine::protocol_manifest())
            .map_err(|error| JsValue::from_str(&error.to_string()))
    }

    #[wasm_bindgen(js_name = validateMathExpression)]
    pub fn validate_math_expression(
        &self,
        source: &str,
        input_format: &str,
        variable: &str,
    ) -> Result<String, JsValue> {
        serde_json::to_string(&MathEngine::validate_math_expression(
            source,
            input_format,
            variable,
        ))
        .map_err(|error| JsValue::from_str(&error.to_string()))
    }

    #[wasm_bindgen(js_name = normalizeRealDomain)]
    pub fn normalize_real_domain(&self, domain_json: &str) -> Result<String, JsValue> {
        let domain = parse_json::<RealDomainDto>(domain_json)?;
        serialize(&MathEngine::normalize_real_domain(&domain))
    }

    #[wasm_bindgen(js_name = intersectRealDomains)]
    pub fn intersect_real_domains(
        &self,
        left_json: &str,
        right_json: &str,
    ) -> Result<String, JsValue> {
        let left = parse_json::<RealDomainDto>(left_json)?;
        let right = parse_json::<RealDomainDto>(right_json)?;
        serialize(&MathEngine::intersect_real_domains(&left, &right))
    }

    #[wasm_bindgen(js_name = compareRealDomains)]
    pub fn compare_real_domains(
        &self,
        left_json: &str,
        right_json: &str,
    ) -> Result<String, JsValue> {
        let left = parse_json::<RealDomainDto>(left_json)?;
        let right = parse_json::<RealDomainDto>(right_json)?;
        serialize(&MathEngine::compare_real_domains(&left, &right))
    }

    #[wasm_bindgen(js_name = realDomainContains)]
    pub fn real_domain_contains(
        &self,
        domain_json: &str,
        value_json: &str,
    ) -> Result<String, JsValue> {
        let domain = parse_json::<RealDomainDto>(domain_json)?;
        let value = parse_json::<ExactValueDto>(value_json)?;
        serialize(&MathEngine::real_domain_contains(&domain, &value))
    }

    #[wasm_bindgen(js_name = validateRealFunction)]
    pub fn validate_real_function(&self, source_json: &str) -> Result<String, JsValue> {
        let source = parse_json::<RealFunctionSourceDto>(source_json)?;
        serialize(&MathEngine::validate_real_function(&source))
    }

    #[wasm_bindgen(js_name = compareRealFunctions)]
    pub fn compare_real_functions(
        &self,
        left_json: &str,
        right_json: &str,
        relation: &str,
    ) -> Result<String, JsValue> {
        let left = parse_json::<RealFunctionSourceDto>(left_json)?;
        let right = parse_json::<RealFunctionSourceDto>(right_json)?;
        serialize(&MathEngine::compare_real_functions(&left, &right, relation))
    }

    #[wasm_bindgen(js_name = evaluateRealFunction)]
    pub fn evaluate_real_function(
        &self,
        source_json: &str,
        input_json: &str,
    ) -> Result<String, JsValue> {
        let source = parse_json::<RealFunctionSourceDto>(source_json)?;
        let input = parse_json::<ExactQuantityDto>(input_json)?;
        serialize(&MathEngine::evaluate_real_function(&source, &input))
    }

    #[wasm_bindgen(js_name = evaluateRealFunctionTable)]
    pub fn evaluate_real_function_table(
        &self,
        source_json: &str,
        inputs_json: &str,
    ) -> Result<String, JsValue> {
        let source = parse_json::<RealFunctionSourceDto>(source_json)?;
        let inputs = parse_json::<Vec<ExactQuantityDto>>(inputs_json)?;
        serialize(&MathEngine::evaluate_real_function_table(&source, &inputs))
    }

    #[wasm_bindgen(js_name = averageRate)]
    pub fn average_rate(
        &self,
        source_json: &str,
        left_json: &str,
        right_json: &str,
    ) -> Result<String, JsValue> {
        let source = parse_json::<RealFunctionSourceDto>(source_json)?;
        let left = parse_json::<ExactQuantityDto>(left_json)?;
        let right = parse_json::<ExactQuantityDto>(right_json)?;
        serialize(&MathEngine::average_rate(&source, &left, &right))
    }

    #[wasm_bindgen(js_name = deriveDifferenceQuotient)]
    pub fn derive_difference_quotient(
        &self,
        source_json: &str,
        increment_variable: &str,
    ) -> Result<String, JsValue> {
        let source = parse_json::<RealFunctionSourceDto>(source_json)?;
        serialize(&MathEngine::derive_difference_quotient(
            &source,
            increment_variable,
        ))
    }

    #[wasm_bindgen(js_name = applyDifferenceQuotientRule)]
    pub fn apply_difference_quotient_rule(
        &self,
        source_json: &str,
        increment_variable: &str,
        rule: &str,
    ) -> Result<String, JsValue> {
        let source = parse_json::<RealFunctionSourceDto>(source_json)?;
        serialize(&MathEngine::apply_difference_quotient_rule(
            &source,
            increment_variable,
            rule,
        ))
    }

    #[wasm_bindgen(js_name = solveLinearEquation)]
    pub fn solve_linear_equation(&self, source: &str, variable: &str) -> Result<String, JsValue> {
        serde_json::to_string(&MathEngine::solve_linear_equation(source, variable))
            .map_err(|error| JsValue::from_str(&error.to_string()))
    }

    #[wasm_bindgen(js_name = runLinearEquationStrategy)]
    pub fn run_linear_equation_strategy(
        &self,
        source: &str,
        variable: &str,
        strategy: &str,
    ) -> Result<String, JsValue> {
        serde_json::to_string(&MathEngine::run_linear_equation_strategy(
            source, variable, strategy,
        ))
        .map_err(|error| JsValue::from_str(&error.to_string()))
    }

    #[wasm_bindgen(js_name = applyLinearEquationRule)]
    pub fn apply_linear_equation_rule(
        &self,
        source: &str,
        variable: &str,
        rule: &str,
        operand: Option<String>,
    ) -> Result<String, JsValue> {
        serde_json::to_string(&MathEngine::apply_linear_equation_rule(
            source,
            variable,
            rule,
            operand.as_deref(),
        ))
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
        grading_mode: &str,
        absolute_tolerance_source: &str,
        relative_tolerance_source: Option<String>,
    ) -> Result<String, JsValue> {
        serde_json::to_string(&MathEngine::compare_numeric_answer(
            submitted_source,
            expected_source,
            input_format,
            grading_mode,
            absolute_tolerance_source,
            relative_tolerance_source.as_deref(),
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

fn parse_json<T: serde::de::DeserializeOwned>(json: &str) -> Result<T, JsValue> {
    serde_json::from_str(json).map_err(|error| JsValue::from_str(&error.to_string()))
}

fn serialize<T: serde::Serialize>(value: &T) -> Result<String, JsValue> {
    serde_json::to_string(value).map_err(|error| JsValue::from_str(&error.to_string()))
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
