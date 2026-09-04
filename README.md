# Modern Rust Crash Course (For C#, Go, Java, Python, C++ Developers)

C#, Go, Java, Python, C++ などの言語を習得済みのエンジニアが、**最短でモダン Rust（Rust 2024 Edition / 1.85+）の真髄をマスターするための実践リファレンス**です。

ガベージコレクション（GC）なしで完全なメモリ安全性を保証する「所有権・借用システム」、C 言語に匹敵する「ゼロコスト抽象化」、そしてデータ競合をコンパイル時に排除する「恐れなき並行性（Fearless Concurrency）」までを網羅しています。

---

## 🚀 クイックスタート (実行方法)

```powershell
# ビルド & 実行 (全 8 モジュールが一括実行されます)
cargo run

# リリースビルド (最高速最適化 + LTO 有効)
cargo build --release

# リンターによる静的解析
cargo clippy
```

---

## 🗺️ 言語対比マッピング早見表 (Rust vs C# vs Go vs Java vs Python vs C++)

| 概念・機能 | Modern Rust (2024) | Modern C# (12+) | Go | Java (21+) | Python (3.10+) | Modern C++ (20+) |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **メモリ管理** | **所有権 (Ownership) / RAII** | GC (CLR) | GC (Go Runtime) | GC (JVM) | 参照カウント + GC | RAII / 手動 |
| **変数の既定** | **不変 (Immutable)** | 可変 (Mutable) | 可変 (Mutable) | 可変 (Mutable) | 可変 (Mutable) | 可変 (Mutable) |
| **変数のコピー** | **ムーブ (Move)** | 参照コピー / 構造体値コピー | 値コピー | 参照コピー | 参照コピー | コピー (ムーブは明示) |
| **例外機構** | ❌ なし (`Result<T, E>`) | `try-catch` | ❌ なし (`error`) | `try-catch` | `try-except` | `try-catch` |
| **エラー伝播** | **`?` 演算子** | 自動バブリング | `if err != nil` | `throws` 宣言 | 自動バブリング | 自動バブリング |
| **Null 安全性** | **`Option<T>` (null 不在)**| `string?` (NRT) | `nil` (ポインタ) | `Optional<T>` | `None` | `std::optional<T>` |
| **ポリモーフィズム** | **`trait`** | `interface` | `interface` | `interface` | ダックタイピング | `concept` / 仮想関数 |
| **スレッド間共有** | `Arc<Mutex<T>>` | `lock` / `Monitor` | `sync.Mutex` | `synchronized` | `threading.Lock` | `std::mutex` |
| **並行キュー** | `mpsc::channel` | `Channels` | **`chan T`** | `BlockingQueue` | `asyncio.Queue` | `std::queue` |
| **無名関数** | `\|x\| x * 2` | `x => x * 2` | `func(x int)` | `x -> x * 2` | `lambda x: x*2` | `[](auto x){}` |

---

## ⚠️ 他言語経験者が最もハマる Rust の「罠」と作法

### 1. 借用チェッカーの「可変参照は同時に 1 つだけ」ルール
- C++ や Java では当たり前にできる「イテレーションしながらコレクション要素を変更・追加する操作」は、Rust では**コンパイルエラー（エイリアシングと可変性の禁止）**になります。
- **原則**: 不変参照 `&T` が生きている間は、可変参照 `&mut T` を同時に作成することはできません。
- **対策**: インデックスで回すか、`retain` メソッドを使うか、`RefCell` などの内部可変性パターンを採用します。

### 2. 文字列のインデックスアクセス禁止 (`s[0]` は不可)
- Rust の `String` / `&str` は厳格に UTF-8 バイト列です。
- 日本語などの多バイト文字が含まれる場合、`s[0]` では文字の途中の不完全なバイトを指してしまうため、**角括弧による添字アクセスがコンパイルエラー**で禁止されています。
- **対策**: 文字単位で走査したい場合は `s.chars().nth(0)`、バイト単位なら `s.as_bytes()[0]` を使用します。

### 3. 自己参照構造体（Self-Referential Structs）の作成困難
- 「ノードが親ノードの参照を保持するツリー構造」などを素朴な参照 `&'a Node` で書こうとすると、借用チェッカーとライフタイム地獄（Borrow Checker Hell）に陥ります。
- **対策**: ポインタではなく **インデックス番号（`usize`）** で管理するか、`Rc<RefCell<Node>>`（または `Weak`）を使用します。

### 4. ライフタイム注釈 `'a` の本当の役割
- ライフタイム注釈は「変数の生存期間を延ばす」ものではありません。
- コンパイラに対して「入力された引数の参照と、返される戻り値の参照の生存期間の関係性」を検証させるための**型レベルのアサーション**です。

---

## 📁 提供サンプルコードの解説

| ファイル | テーマ | 主な学習内容 |
| :--- | :--- | :--- |
| [`m01_ownership_and_types.rs`](./sample/src/m01_ownership_and_types.rs) | **所有権・借用・型システム** | ムーブセマンティクス, Copy vs Clone, `&T` vs `&mut T`, データ付き enum, ライフタイム `'a` |
| [`m02_pattern_and_error.rs`](./sample/src/m02_pattern_and_error.rs) | **パターンマッチング & エラー処理** | `match` ガード, `let-else` (Rust 1.65+), `Option<T>` コンビネータ, `Result<T, E>` と `?` 演算子 |
| [`m03_collections_and_iterators.rs`](./sample/src/m03_collections_and_iterators.rs) | **コレクション & ゼロコストイテレータ** | `Vec`, スライス (`&[T]`), HashMap Entry API (`or_insert`), `map/filter/fold/zip`, カスタム Iterator |
| [`m04_traits_and_generics.rs`](./sample/src/m04_traits_and_generics.rs) | **トレイト & ジェネリクス** | 静的ディスパッチ (単一化), 動的ディスパッチ (`Box<dyn Trait>`), 演算子オーバーロード (`std::ops::Add`) |
| [`m05_closures_and_fn.rs`](./sample/src/m05_closures_and_fn.rs) | **クロージャ & 高階関数** | `Fn`, `FnMut`, `FnOnce` の 3 兄弟, `move` クロージャ, クロージャを返す関数 (`impl Fn` / `Box<dyn Fn>`) |
| [`m06_smart_pointers.rs`](./sample/src/m06_smart_pointers.rs) | **スマートポインタ & メモリ管理** | `Box<T>` (再帰型), `Rc<T>` (参照カウント), `RefCell<T>` (内部可変性), `Drop` トレイト (RAII) |
| [`m07_concurrency.rs`](./sample/src/m07_concurrency.rs) | **並行処理 & スレッド安全** | `thread::spawn`, **スコープ付きスレッド (`thread::scope`)**, `mpsc::channel`, `Arc<Mutex<T>>` |
| [`m08_advanced_features.rs`](./sample/src/m08_advanced_features.rs) | **高度な機能 & Unsafe** | 宣言的マクロ (`macro_rules!`), `const` ジェネリクス (`[T; N]`), 型エイリアス, `unsafe` の安全な抽象化 |
| [`main.rs`](./sample/src/main.rs) | **統合エントリーポイント** | 全 8 モジュールを順番にバナー付きで実行するメインランナー |

---

## 📚 関連ドキュメント案内

> 📖 **Rust クロージャ完全理解ガイド**:  
> `Fn`, `FnMut`, `FnOnce` のトレイト継承関係、コンパイラが裏で生成する無名構造体（Anonymous Struct）、ヒープ確保ゼロの仕組み、`move` クロージャの使い分けまで完全網羅した解説は [**`LAMBDA.md`**](./sample/LAMBDA.md) を参照してください。

> 🛠️ **Cargo & Cargo.toml 完全理解ガイド**:  
> `Cargo.toml` の構成、依存関係の指定方法、ワークスペース設計、機能フラグ (`[features]`)、リリース最適化プロファイル (`[profile.release]`)、LTO の設定まで完全網羅した解説は [**`CARGO_GUIDE.md`**](./sample/CARGO_GUIDE.md) を参照してください。
