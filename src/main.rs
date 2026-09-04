// ============================================================================
// Modern Rust (Rust 2024 Edition / 1.85+) Crash Course - Main Runner
// For C# / Go / Java / Python / C++ Developers
// ============================================================================

mod m01_ownership_and_types;
mod m02_pattern_and_error;
mod m03_collections_and_iterators;
mod m04_traits_and_generics;
mod m05_closures_and_fn;
mod m06_smart_pointers;
mod m07_concurrency;
mod m08_advanced_features;

fn main() {
    print_banner("MODERN RUST CRASH COURSE (Memory Safety & Zero-Cost Abstractions)");

    // モジュール 01: 所有権・借用・ライフタイム・基本型
    print_section("01: Ownership, Move Semantics, Borrowing (&T/&mut T), and Enums");
    m01_ownership_and_types::run();

    // モジュール 02: パターンマッチング・エラー処理 (Option, Result, ?)
    print_section("02: Pattern Matching, let-else, Option, Result, and '?' Operator");
    m02_pattern_and_error::run();

    // モジュール 03: コレクション・ゼロコストイテレータパイプライン
    print_section("03: Vectors, Slices, HashMap Entry API, and Iterator Pipelines");
    m03_collections_and_iterators::run();

    // モジュール 04: トレイト・ジェネリクス・ディスパッチ・演算子オーバーロード
    print_section("04: Traits, Static/Dynamic Dispatch (dyn Trait), and Operator Overload");
    m04_traits_and_generics::run();

    // モジュール 05: クロージャ (Fn, FnMut, FnOnce)・move
    print_section("05: Closures (Fn, FnMut, FnOnce), Environment Capture, and move");
    m05_closures_and_fn::run();

    // モジュール 06: スマートポインタ (Box, Rc, Arc)・内部可変性 (RefCell)・RAII
    print_section("06: Smart Pointers (Box, Rc), Interior Mutability (RefCell), and Drop");
    m06_smart_pointers::run();

    // モジュール 07: 並行処理・スコープ付きスレッド・mpsc チャネル・Arc<Mutex<T>>
    print_section("07: Concurrency, Scoped Threads (std::thread::scope), Channels, and Mutex");
    m07_concurrency::run();

    // モジュール 08: 高度な機能 (マクロ, const ジェネリクス, unsafe の安全な抽象化)
    print_section("08: Declarative Macros, Const Generics, and Safe Unsafe Abstractions");
    m08_advanced_features::run();

    print_banner("ALL RUST TUTORIAL MODULES COMPLETED SUCCESSFULLY!");
}

fn print_banner(title: &str) {
    println!("\n{}", "=".repeat(72));
    println!("  {}", title);
    println!("{}\n", "=".repeat(72));
}

fn print_section(title: &str) {
    println!("\n{}", "#".repeat(72));
    println!("# {}", title);
    println!("{}\n", "#".repeat(72));
}
