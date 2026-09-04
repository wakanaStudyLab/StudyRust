//! ============================================================================
//! モジュール 05: クロージャ & 高階関数 (Closures: Fn, FnMut, FnOnce)
//! ============================================================================
//!
//! 【他言語経験者向け要点】
//! 1. クロージャトレイトの 3 兄弟:
//!    - `FnOnce`: 環境を消費（所有権ムーブ）して 1 回だけ呼べる。
//!    - `FnMut`: 環境を書き換える（可変借用）ため、複数回呼べる。
//!    - `Fn`: 環境を読み取るだけ（不変借用）のため、複数回安全に呼べる（並行実行も容易）。
//!
//! 2. `move` キーワード:
//!    - キャプチャする環境の変数を強制的にクロージャ内へムーブ。
//!    - 別スレッドに変数を渡す際（`thread::spawn`）に必須。
//!
//! 3. アロケーションなしのインライン展開:
//!    - Rust のクロージャはコンパイラが固有の無名構造体（Anonymous Struct）を生成するため、
//!      デフォルトではヒープ確保（GC や Box）が一切発生しない。

pub fn run() {
    demo_basic_closures();
    demo_closure_traits();
    demo_move_closure();
    demo_returning_closures();
}

fn demo_basic_closures() {
    println!("=== 1. Basic Closures & Type Inference ===");

    let factor = 3;
    // factor を不変借用してキャプチャ
    let multiply = |x: i32| x * factor;

    println!("Multiply 10 by factor {}: {}", factor, multiply(10));
}

fn demo_closure_traits() {
    println!("\n=== 2. The 3 Closure Traits: Fn, FnMut, and FnOnce ===");

    // 1. Fn (不変借用: 読み取り専用)
    let greeting = String::from("Hello");
    let print_greeting = || println!("  [Fn]: {}", greeting);
    call_fn(&print_greeting);
    call_fn(&print_greeting); // 何度でも呼べる

    // 2. FnMut (可変借用: 内部状態の書き換え)
    let mut counter = 0;
    let mut increment = || {
        counter += 1;
        println!("  [FnMut]: Counter incremented to {}", counter);
    };
    call_fn_mut(&mut increment);
    call_fn_mut(&mut increment);

    // 3. FnOnce (所有権消費: 1回しか呼べない)
    let secret = String::from("TopSecretData");
    let consume_secret = || {
        println!("  [FnOnce]: Dropping and consuming {}", secret);
        drop(secret); // 所有権が消費される
    };
    call_fn_once(consume_secret);
    // call_fn_once(consume_secret); // ❌ コンパイルエラー: 既にムーブ済み
}

fn call_fn<F: Fn()>(f: &F) {
    f();
}

fn call_fn_mut<F: FnMut()>(f: &mut F) {
    f();
}

fn call_fn_once<F: FnOnce()>(f: F) {
    f();
}

fn demo_move_closure() {
    println!("\n=== 3. 'move' Closures (Taking Ownership of Environment) ===");

    let mut data = vec![1, 2, 3];

    // move キーワードにより data の所有権がクロージャ内部に移譲される
    let mut owns_data = move || {
        data.push(4);
        println!("  [Move Closure]: Data pushed, new len: {:?}", data);
    };

    owns_data();
    // println!("{:?}", data); // ❌ コンパイルエラー: data はすでに move されている
}

fn demo_returning_closures() {
    println!("\n=== 4. Returning Closures (impl Fn vs Box<dyn Fn>) ===");

    let add_five = make_adder(5);
    println!("make_adder(5)(10) = {}", add_five(10));

    let doubler = make_boxed_multiplier(2);
    println!("make_boxed_multiplier(2)(15) = {}", doubler(15));
}

// 静的ディスパッチでクロージャを返す (アロケーションなし)
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

// 動的ディスパッチでクロージャを返す (ヒープに格納)
fn make_boxed_multiplier(n: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x * n)
}
