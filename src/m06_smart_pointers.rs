//! ============================================================================
//! モジュール 06: スマートポインタ & 内部可変性 (Smart Pointers & Memory Management)
//! ============================================================================
//!
//! 【他言語経験者向け要点】
//! 1. Box<T>:
//!    - データをヒープに確保し、スタックにはポインタのみ置く。
//!    - 再帰的な型（連結リストや構文木）のサイズ確定に必須。
//!
//! 2. Rc<T> & Arc<T> (Reference Counting):
//!    - 複数の所有者（Shared Ownership）を持つためのスマートポインタ。
//!    - `Rc` はシングルスレッド用（高速）、`Arc` はアトミックでマルチスレッド安全。
//!
//! 3. RefCell<T> (Interior Mutability):
//!    - 不変参照経由でも内部の値を変更可能にする「借用チェッカーを実行時に遅延」させる仕組み。
//!    - 規則違反時はコンパイルエラーではなくパニックする。
//!
//! 4. Drop トレイト:
//!    - C++ のデストラクタや C# の Dispose/using に相当する自動リソース解放。

use std::cell::RefCell;
use std::rc::Rc;

// Box による再帰的データ構造 (連結リスト)
#[allow(dead_code)]
#[derive(Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

// Drop トレイトのデモ用リソース
struct CustomResource {
    name: String,
}

impl Drop for CustomResource {
    fn drop(&mut self) {
        println!("  [Drop Triggered]: Resource '{}' was automatically freed!", self.name);
    }
}

pub fn run() {
    demo_box_recursive_type();
    demo_rc_shared_ownership();
    demo_refcell_interior_mutability();
    demo_drop_raii();
}

fn demo_box_recursive_type() {
    println!("=== 1. Box<T> for Recursive Data Structures ===");

    // 1 -> 2 -> 3 -> Nil の連結リスト
    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );

    println!("Constructed linked list with Box: {:?}", list);
}

fn demo_rc_shared_ownership() {
    println!("\n=== 2. Rc<T> (Shared Ownership via Reference Counting) ===");

    let shared_data = Rc::new(String::from("Shared Config"));
    println!("Initial Rc count: {}", Rc::strong_count(&shared_data)); // 1

    {
        // 参照カウントを増やして共有 (ディープコピーではなくポインタ共有)
        let consumer_a = Rc::clone(&shared_data);
        let consumer_b = Rc::clone(&shared_data);
        println!("Rc count inside block: {}", Rc::strong_count(&shared_data)); // 3
        println!("Consumer A sees: {}", consumer_a);
        println!("Consumer B sees: {}", consumer_b);
    }

    println!("Rc count after block: {}", Rc::strong_count(&shared_data)); // 1
}

fn demo_refcell_interior_mutability() {
    println!("\n=== 3. RefCell<T> (Interior Mutability Pattern) ===");

    // 不変な変数として宣言しているが...
    let data = RefCell::new(vec![10, 20]);

    // 不変参照経由で内部を変更可能！
    data.borrow_mut().push(30);

    println!("RefCell modified contents: {:?}", data.borrow());
}

fn demo_drop_raii() {
    println!("\n=== 4. RAII & Drop Trait (Automatic Cleanup) ===");

    {
        let _res1 = CustomResource {
            name: String::from("DatabaseConnection"),
        };
        println!("  Doing work inside inner scope...");
        // スコープ終了時に _res1 の drop() が自動呼出しされる
    }
    println!("Exited scope successfully.");
}
