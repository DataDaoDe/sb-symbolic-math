use socrates_math_app::MathEngine;

fn main() {
    println!("Symbolic Math Rust Playground");
    println!();

    let expression = MathEngine::normalize_math_expression("3(x - 2) + 4", "latex", "x");
    println!("normalize math_expression: 3(x - 2) + 4");
    println!("{expression:#?}");
    println!();

    let expression_comparison =
        MathEngine::compare_math_expressions("2(x + 1)", "2x + 2", "latex", "x");
    println!("compare math_expression: 2(x + 1) vs 2x + 2");
    println!("{expression_comparison:#?}");
    println!();

    let polynomial_comparison =
        MathEngine::compare_math_expressions("(x + 1)(x - 1)", "x^2 - 1", "latex", "x");
    println!("compare polynomial math_expression: (x + 1)(x - 1) vs x^2 - 1");
    println!("{polynomial_comparison:#?}");
    println!();

    let derivative = MathEngine::differentiate_math_expression("x^3 + 2x", "latex", "x");
    println!("differentiate math_expression: x^3 + 2x");
    println!("{derivative:#?}");
    println!();

    let integral = MathEngine::integrate_math_expression("x^3", "latex", "x");
    println!("integrate math_expression: x^3");
    println!("{integral:#?}");
    println!();

    let equation_comparison =
        MathEngine::compare_equation_solution_sets("x + 1 = 3", "2x = 4", "x");
    println!("compare math_equation by solution set: x + 1 = 3 vs 2x = 4");
    println!("{equation_comparison:#?}");
    println!();

    let numeric =
        MathEngine::compare_numeric_answer("\\frac{333}{1000}", "\\frac{1}{3}", "latex", 0.001);
    println!("compare numeric_answer: \\frac{{333}}{{1000}} vs \\frac{{1}}{{3}}, tolerance 0.001");
    println!("{numeric:#?}");
}
