//! ============================================================================
//! モジュール 02: パターンマッチング & エラー処理 (Pattern Matching & Error Handling)
//! ============================================================================
//!
//! 【他言語経験者向け要点】
//! 1. 例外 (Exception) の不在:
//!    - Rust には `try-catch` や例外機構は存在しない。
//!    - 回復可能なエラーは `Result<T, E>`、値の不在は `Option<T>` として型システムで扱う。
//!    - 致命的エラー（バグ）のみ `panic!` でプロセス停止。
//!
//! 2. `?` 演算子:
//!    - Go の `if err != nil { return err }` や Java の `throws` を1文字で書ける糖衣構文。
//!
//! 3. `let-else` 文 (Rust 1.65+):
//!    - ガード節での早期リターンを美しく書く構文。

use std::error::Error;
use std::fmt;

// ドメインカスタムエラー型
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum MathError {
    DivisionByZero,
    NegativeSquareRoot,
    Overflow,
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MathError::DivisionByZero => write!(f, "Cannot divide by zero"),
            MathError::NegativeSquareRoot => write!(f, "Cannot compute square root of negative number"),
            MathError::Overflow => write!(f, "Mathematical operation caused overflow"),
        }
    }
}

impl Error for MathError {}

pub fn run() {
    demo_pattern_matching();
    demo_let_else();
    demo_option_combinators();
    demo_result_and_question_operator();
}

fn demo_pattern_matching() {
    println!("=== 1. Advanced Pattern Matching & Match Guards ===");

    let scores = [95, 82, 60, 45];

    for &score in &scores {
        let grade = match score {
            90..=100 => "A+ (Excellent)",
            80..=89 => "A (Great)",
            70..=79 => "B (Good)",
            s if s >= 60 => "C (Pass with guard)",
            _ => "F (Fail)",
        };
        println!("Score: {} -> Grade: {}", score, grade);
    }
}

fn demo_let_else() {
    println!("\n=== 2. 'let-else' Early Return (Rust 1.65+) ===");

    let success_input = Some("valid_token_123");
    let failure_input: Option<&str> = None;

    process_token(success_input);
    process_token(failure_input);
}

fn process_token(token: Option<&str>) {
    // let-else 構文: Some でなければ else ブロックを実行して即リターン
    let Some(t) = token else {
        println!("  [Guard Failed] Missing token, skipping.");
        return;
    };

    println!("  [Token Verified]: {}", t);
}

fn demo_option_combinators() {
    println!("\n=== 3. Option<T> & Functional Combinators ===");

    let config_val: Option<&str> = Some("42");

    // map, and_then, unwrap_or を使った関数型パイプライン
    let parsed: i32 = config_val
        .and_then(|s| s.parse::<i32>().ok())
        .map(|n| n * 2)
        .unwrap_or(0);

    println!("Parsed and doubled value: {}", parsed);
}

fn demo_result_and_question_operator() {
    println!("\n=== 4. Result<T, E> & '?' Operator Propagation ===");

    match complex_calculation(10.0, 2.0, 9.0) {
        Ok(res) => println!("Complex calc succeeded: {}", res),
        Err(e) => println!("Complex calc failed: {}", e),
    }

    match complex_calculation(10.0, 0.0, 9.0) {
        Ok(res) => println!("Complex calc succeeded: {}", res),
        Err(e) => println!("Expected error caught: {}", e),
    }
}

fn complex_calculation(a: f64, b: f64, c: f64) -> Result<f64, MathError> {
    // ? 演算子により、Err の場合は呼び出し元へ即座に伝播される
    let div_res = safe_divide(a, b)?;
    let sqrt_res = safe_sqrt(c)?;
    Ok(div_res + sqrt_res)
}

fn safe_divide(a: f64, b: f64) -> Result<f64, MathError> {
    if b == 0.0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

fn safe_sqrt(x: f64) -> Result<f64, MathError> {
    if x < 0.0 {
        Err(MathError::NegativeSquareRoot)
    } else {
        Ok(x.sqrt())
    }
}
