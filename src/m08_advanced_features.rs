//! ============================================================================
//! モジュール 08: 高度な機能 (Advanced Features: Macros, Const Generics, Unsafe)
//! ============================================================================
//!
//! 【他言語経験者向け要点】
//! 1. 宣言的マクロ (macro_rules!):
//!    - C/C++ の単純なテキスト置換と異なり、AST（構文木）レベルで衛生的（Hygienic）に展開される。
//!
//! 2. const ジェネリクス:
//!    - 配列のサイズなどの「値」をジェネリクスの型パラメータとして扱える。
//!
//! 3. unsafe Rust:
//!    - 借用チェッカーを迂回し、生ポインタの参照解除や FFI（C言語連携）を行うための機能。
//!    - 「unsafe ブロックで囲み、外側には安全な API（Safe Abstraction）を提供する」のが Rust の鉄則。

// 1. 宣言的マクロの定義
#[macro_export]
macro_rules! map_create {
    ( $( $key:expr => $val:expr ),* $(,)? ) => {
        {
            let mut temp_map = std::collections::HashMap::new();
            $(
                temp_map.insert($key, $val);
            )*
            temp_map
        }
    };
}

pub fn run() {
    demo_declarative_macro();
    demo_const_generics();
    demo_type_aliases();
    demo_safe_wrapper_over_unsafe();
}

fn demo_declarative_macro() {
    println!("=== 1. Declarative Macros (macro_rules!) ===");

    // 自作のマクロで HashMap をリテラル風に初期化
    let scores = map_create! {
        "Alice" => 100,
        "Bob" => 85,
        "Charlie" => 92,
    };

    println!("Map initialized via custom macro: {:?}", scores);
}

// 2. const ジェネリクス (配列サイズ N を型パラメータとして受ける)
fn print_array_info<T: std::fmt::Debug, const N: usize>(arr: &[T; N]) {
    println!("  Array of size {}: {:?}", N, arr);
}

fn demo_const_generics() {
    println!("\n=== 2. Const Generics (Compile-Time Array Sizes) ===");

    let small_arr = [1, 2, 3];
    let large_arr = [10, 20, 30, 40, 50];

    print_array_info(&small_arr);
    print_array_info(&large_arr);
}

// 3. 型エイリアス
type Milliseconds = u64;

fn demo_type_aliases() {
    println!("\n=== 3. Type Aliases ===");

    let timeout: Milliseconds = 5000;
    println!("Timeout set to: {} ms", timeout);
}

// 4. unsafe Rust の安全な抽象化
fn demo_safe_wrapper_over_unsafe() {
    println!("\n=== 4. Unsafe Rust: Safe Abstraction over Raw Pointers ===");

    let val = 42;
    // 生ポインタの作成自体は safe
    let raw_ptr: *const i32 = &val;

    // 生ポインタの参照解除（Dereference）には unsafe が必要
    let dereferenced = unsafe { *raw_ptr };
    println!("Read value via raw pointer: {}", dereferenced);

    // split_at_mut のような標準ライブラリの unsafe 安全ラッパーの概念
    let mut data = [1, 2, 3, 4, 5, 6];
    let (left, right) = data.split_at_mut(3);
    println!("Safely split slice into two mutable borrows: left={:?}, right={:?}", left, right);
}
