use socrates_math_app::MathEngine;
use std::env;
use std::process;

fn main() {
    let mut args = env::args().skip(1).collect::<Vec<_>>();
    if args.first().is_some_and(|arg| arg == "--") {
        args.remove(0);
    }
    let Some(command) = args.first().map(String::as_str) else {
        usage();
    };

    let json = match command {
        "solve-linear" => {
            expect_len(&args, 3);
            serde_json::to_string(&MathEngine::solve_linear_equation(&args[1], &args[2]))
        }
        "compare-equations" => {
            expect_len(&args, 4);
            serde_json::to_string(&MathEngine::compare_equation_solution_sets(
                &args[1], &args[2], &args[3],
            ))
        }
        "normalize-expression" => {
            expect_len(&args, 4);
            serde_json::to_string(&MathEngine::normalize_math_expression(
                &args[1], &args[2], &args[3],
            ))
        }
        "compare-expressions" => {
            expect_len(&args, 5);
            serde_json::to_string(&MathEngine::compare_math_expressions(
                &args[1], &args[2], &args[3], &args[4],
            ))
        }
        "compare-numeric" => {
            expect_len(&args, 5);
            let tolerance = args[4].parse::<f64>().unwrap_or_else(|error| {
                eprintln!("invalid tolerance '{}': {error}", args[4]);
                process::exit(2);
            });
            serde_json::to_string(&MathEngine::compare_numeric_answer(
                &args[1], &args[2], &args[3], tolerance,
            ))
        }
        "differentiate" => {
            expect_len(&args, 4);
            serde_json::to_string(&MathEngine::differentiate_math_expression(
                &args[1], &args[2], &args[3],
            ))
        }
        "integrate" => {
            expect_len(&args, 4);
            serde_json::to_string(&MathEngine::integrate_math_expression(
                &args[1], &args[2], &args[3],
            ))
        }
        _ => usage(),
    };

    match json {
        Ok(json) => println!("{json}"),
        Err(error) => {
            eprintln!("failed to serialize response: {error}");
            process::exit(1);
        }
    }
}

fn expect_len(args: &[String], expected: usize) {
    if args.len() != expected {
        usage();
    }
}

fn usage() -> ! {
    eprintln!(
        "usage:
  api solve-linear <equation> <variable>
  api compare-equations <left-equation> <right-equation> <variable>
  api normalize-expression <expression> <input-format> <variable>
  api compare-expressions <left-expression> <right-expression> <input-format> <variable>
  api compare-numeric <submitted> <expected> <input-format> <tolerance>
  api differentiate <expression> <input-format> <variable>
  api integrate <expression> <input-format> <variable>"
    );
    process::exit(2);
}
