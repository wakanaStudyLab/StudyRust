//! ============================================================================
//! モジュール 04: トレイト & ジェネリクス (Traits, Generics & Polymorphism)
//! ============================================================================
//!
//! 【他言語経験者向け要点】
//! 1. トレイト (Trait):
//!    - Java/C# のインターフェースや Go の interface、C++20 の Concept に相当。
//!    - 既存の型に対して後から外部トレイトを実装可能（Extension Methods や Rust流のポリモーフィズム）。
//!
//! 2. 静的ディスパッチ (Monomorphization: `impl Trait` / ジェネリクス):
//!    - コンパイル時に型ごとに専用コードが展開される。実行時オーバーヘッドはゼロ。
//!
//! 3. 動的ディスパッチ (Trait Objects: `dyn Trait`):
//!    - 実行時に vtable（仮想関数テーブル）経由で呼び出す。異種コレクションを保持する場合に必須。
//!
//! 4. 演算子オーバーロード:
//!    - `std::ops` のトレイトを実装することで `+` や `*` などの演算子を自作型に適用可能。

use std::ops::Add;

// トレイトの定義 (デフォルト実装付き)
pub trait Summary {
    fn summarize_author(&self) -> String;

    // デフォルト実装
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub author: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }

    // デフォルト実装をオーバーライド
    fn summarize(&self) -> String {
        format!("{} by {}", self.headline, self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("@{}: \"{}\"", self.username, self.content)
    }
}

// 演算子オーバーロードのデモ用構造体
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

pub fn run() {
    demo_static_dispatch();
    demo_dynamic_dispatch();
    demo_operator_overloading();
}

// 1. 静的ディスパッチ (コンパイル時に単一化: ゼロオーバーヘッド)
fn print_summary<T: Summary>(item: &T) {
    println!("  [Static Dispatch]: {}", item.summarize());
}

fn demo_static_dispatch() {
    println!("=== 1. Traits & Static Dispatch (Generics) ===");

    let article = NewsArticle {
        headline: String::from("Rust 2024 Released"),
        author: String::from("Ferris"),
    };
    let tweet = Tweet {
        username: String::from("rustlang"),
        content: String::from("Rust is fast and safe!"),
    };

    print_summary(&article);
    print_summary(&tweet);
}

// 2. 動的ディスパッチ (トレイトオブジェクト: vtable 経由)
fn demo_dynamic_dispatch() {
    println!("\n=== 2. Dynamic Dispatch via Trait Objects (Box<dyn Trait>) ===");

    // 異なる型のオブジェクトを単一のベクタに格納！
    let feed: Vec<Box<dyn Summary>> = vec![
        Box::new(NewsArticle {
            headline: String::from("Breakthrough in Systems AI"),
            author: String::from("Alice"),
        }),
        Box::new(Tweet {
            username: String::from("tech_insider"),
            content: String::from("Zero-cost abstractions are amazing."),
        }),
    ];

    for item in &feed {
        println!("  [Dynamic Dispatch]: {}", item.summarize());
    }
}

fn demo_operator_overloading() {
    println!("\n=== 3. Operator Overloading (std::ops::Add) ===");

    let v1 = Vector2 { x: 3.0, y: 4.0 };
    let v2 = Vector2 { x: 1.5, y: 2.5 };
    let sum = v1 + v2; // Add トレイトの呼び出し

    println!("v1 ({:?}) + v2 ({:?}) = {:?}", v1, v2, sum);
}
