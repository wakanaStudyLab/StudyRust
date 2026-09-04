//! ============================================================================
//! モジュール 01: 所有権・借用・型システム (Ownership, Borrowing & Types)
//! ============================================================================
//!
//! 【他言語経験者（C#, Go, Java, Python, C++）向け要点】
//! 1. 所有権 (Ownership):
//!    - 全ての値は唯一の所有者（変数）を持つ。
//!    - 所有者がスコープを抜けると値は自動的に破棄される（GCなしのRAII）。
//!    - 代入や関数への値渡しはデフォルトで「ムーブ（Move）」され、元の変数は使用不能になる。
//!
//! 2. Copy vs Clone:
//!    - プリミティブ型（整数・浮動小数点など）は `Copy` トレイトを持ち、自動でビットコピーされる。
//!    - ヒープ領域を持つ型（`String`, `Vec` 等）は明示的に `.clone()` が必要。
//!
//! 3. 借用規則 (Borrow Checker):
//!    - 任意の時点で「複数の不変参照 (&T)」または「唯一の可変参照 (&mut T)」のいずれか一方のみ許される。
//!    - データ競合（Data Race）をコンパイル時に 100% 防止する。

// 構造体とタプル構造体
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub active: bool,
}

#[derive(Debug, Copy, Clone)]
pub struct Point2D(pub f64, pub f64);

// データ付き列挙型 (直和型 / Tagged Union)
#[derive(Debug)]
pub enum Command {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

pub fn run() {
    demo_ownership_and_move();
    demo_borrowing_and_mutability();
    demo_enums_and_data();
    demo_lifetimes();
}

fn demo_ownership_and_move() {
    println!("=== 1. Ownership & Move Semantics ===");

    let s1 = String::from("Rust Systems");
    // s1 の所有権が s2 へムーブする（ヒープ再確保なし、ポインタ等の所有権移譲）
    let s2 = s1;
    // println!("{}", s1); // ❌ コンパイルエラー: s1 はすでにムーブ済み

    // 明示的なディープコピー
    let s3 = s2.clone();
    println!("s2: {}, cloned s3: {}", s2, s3);

    // Copy 型 (スタック割り当て)
    let p1 = Point2D(10.0, 20.0);
    let p2 = p1; // Copy トレイトにより自動複製
    println!("p1: ({}, {}), p2: ({}, {})", p1.0, p1.1, p2.0, p2.1);
}

fn demo_borrowing_and_mutability() {
    println!("\n=== 2. References & Borrowing (&T vs &mut T) ===");

    let mut user = User {
        id: String::from("u100"),
        username: String::from("Alice"),
        active: true,
    };

    // 1. 不変借用 (Read-Only)
    let len = calculate_name_length(&user);
    println!("User {} name length: {}", user.username, len);

    // 2. 可変借用 (&mut T)
    rename_user(&mut user, "Alice Cooper");
    println!("Updated username: {}", user.username);

    // 【借用規則の検証】
    let r1 = &user.username;
    let r2 = &user.username;
    println!("Simultaneous immutable borrows: {} and {}", r1, r2);
    // user.username.push_str("!"); // ❌ r1, r2 の生存期間中は可変変更不可
}

fn calculate_name_length(user: &User) -> usize {
    user.username.len()
}

fn rename_user(user: &mut User, new_name: &str) {
    user.username.clear();
    user.username.push_str(new_name);
}

fn demo_enums_and_data() {
    println!("\n=== 3. Enums with Embedded Data (Algebraic Data Types) ===");

    let commands = [
        Command::Write(String::from("Hello Rust")),
        Command::Move { x: 10, y: 25 },
        Command::ChangeColor(255, 128, 0),
        Command::Quit,
    ];

    for cmd in &commands {
        match cmd {
            Command::Quit => println!("  [Action] Quit requested"),
            Command::Move { x, y } => println!("  [Action] Move to x={}, y={}", x, y),
            Command::Write(text) => println!("  [Action] Print text: \"{}\"", text),
            Command::ChangeColor(r, g, b) => println!("  [Action] Change RGB color: ({}, {}, {})", r, g, b),
        }
    }
}

fn demo_lifetimes() {
    println!("\n=== 4. Explicit Lifetime Annotations ('a) ===");

    let text1 = String::from("long string is long");
    let text2 = "short";

    // 戻り値の参照がどちらの引数に由来するかコンパイラに伝えるライフタイム
    let result = longest(text1.as_str(), text2);
    println!("Longest string between '{}' and '{}' is: '{}'", text1, text2, result);
}

// 'a は x と y の両方が生存している共通のライフタイムを表す
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
