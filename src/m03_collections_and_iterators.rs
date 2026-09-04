//! ============================================================================
//! モジュール 03: コレクション & ゼロコストイテレータ (Collections & Iterators)
//! ============================================================================
//!
//! 【他言語経験者向け要点】
//! 1. ゼロコスト抽象化 (Zero-Cost Abstractions):
//!    - Rust のイテレータ（`map`, `filter` 等）はコンパイル時に手書きの C ループと同等の機械語に最適化される。
//!    - 中間配列やアロケーションは一切発生しない。
//!
//! 2. 3種類のイテレーション:
//!    - `iter()`: 不変参照を反復 (`&T`)
//!    - `iter_mut()`: 可変参照を反復 (`&mut T`)
//!    - `into_iter()`: 所有権を消費して反復 (`T`)
//!
//! 3. HashMap の Entry API:
//!    - `entry(key).or_insert(...)` により、ハッシュ計算1回だけで「検索・存在しなければ初期化」をアトミックに行う。

use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Product {
    pub name: String,
    pub category: String,
    pub price: u32,
}

pub fn run() {
    demo_vector_and_slices();
    demo_hashmap_entry_api();
    demo_iterator_pipeline();
    demo_custom_iterator();
}

fn demo_vector_and_slices() {
    println!("=== 1. Vectors & Zero-Copy Slices (&[T]) ===");

    let numbers = vec![10, 20, 30, 40, 50];

    // スライス (&[T]) はポインタと長さのペアだけでメモリコピーなし
    let slice: &[i32] = &numbers[1..4]; // [20, 30, 40]
    println!("Full vector: {:?}, Slice [1..4]: {:?}", numbers, slice);
}

fn demo_hashmap_entry_api() {
    println!("\n=== 2. HashMap & Entry API (Idiomatic In-Place Update) ===");

    let text = "apple banana apple orange banana apple";
    let mut word_counts: HashMap<&str, u32> = HashMap::new();

    for word in text.split_whitespace() {
        // Entry API: 見つからなければ 0 を挿入し、その可変参照に対して +1
        *word_counts.entry(word).or_insert(0) += 1;
    }

    println!("Word frequencies:");
    for (word, count) in &word_counts {
        println!("  - '{}': {} times", word, count);
    }
}

fn demo_iterator_pipeline() {
    println!("\n=== 3. Zero-Cost Iterator Pipeline ===");

    let products = [
        Product { name: "MacBook".into(), category: "Electronics".into(), price: 250000 },
        Product { name: "Keyboard".into(), category: "Electronics".into(), price: 18000 },
        Product { name: "Rust Book".into(), category: "Books".into(), price: 4200 },
        Product { name: "Coffee".into(), category: "Food".into(), price: 500 },
    ];

    // パイプライン: Electronics カテゴリで 20,000 円以下の商品名を取得
    let budget_electronics: Vec<&str> = products
        .iter()
        .filter(|p| p.category == "Electronics" && p.price <= 20000)
        .map(|p| p.name.as_str())
        .collect();

    println!("Budget electronics: {:?}", budget_electronics);

    // fold (アキュムレータによる集約のデモ: 単純合計なら .sum() も可)
    #[allow(clippy::unnecessary_fold)]
    let total_price: u32 = products.iter().map(|p| p.price).fold(0, |acc, x| acc + x);
    println!("Total catalog price: JPY {}", total_price);

    // zip による2つのリストの結合
    let names = ["Alice", "Bob", "Charlie"];
    let ages = [28, 34, 22];
    let paired: Vec<(&str, i32)> = names.iter().copied().zip(ages).collect();
    println!("Paired data: {:?}", paired);
}

fn demo_custom_iterator() {
    println!("\n=== 4. Implementing Custom Iterator Trait ===");

    let counter = Countdown::new(3);
    for count in counter {
        println!("  Countdown: {}...", count);
    }
    println!("  Blast off!");
}

// カウントダウン用のカスタム構造体
struct Countdown {
    current: u32,
}

impl Countdown {
    fn new(start: u32) -> Self {
        Countdown { current: start }
    }
}

// Iterator トレイトの実装 (next メソッドのみで全コンビネータが手に入る)
impl Iterator for Countdown {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current > 0 {
            let val = self.current;
            self.current -= 1;
            Some(val)
        } else {
            None
        }
    }
}
