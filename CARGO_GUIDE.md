# Modern Cargo & Cargo.toml 完全理解ガイド (Cargo Master Guide)

Rust の強力なビルドシステム兼パッケージマネージャである **「Cargo」** とマニフェストファイル **「Cargo.toml」** の完全解説書です。

C# の `.csproj`、C++ の `CMakeLists.txt`、Go の `go.mod`、Node.js の `package.json` に相当する Cargo の全貌、依存関係の指定方法、マルチパッケージ・ワークスペース設計、機能フラグ (`[features]`)、そして **本番用バイナリを極限まで高速化・軽量化するリリースプロファイル設定（LTO など）** までを網羅しています。

---

## 📑 目次

1. [Cargo の基本思想と標準ディレクトリ構成](#1-cargo-の基本思想と標準ディレクトリ構成)
2. [最重要基本プロパティ (`[package]`)](#2-最重要基本プロパティ-package)
3. [依存関係の指定方法完全リファレンス (`[dependencies]`)](#3-依存関係の指定方法完全リファレンス-dependencies)
4. [マルチパッケージ・ワークスペース設計 (`[workspace]`)](#4-マルチパッケージワークスペース設計-workspace)
5. [機能フラグによる条件付きコンパイル (`[features]`)](#5-機能フラグによる条件付きコンパイル-features)
6. [最高速・最小バイナリを作るリリース最適化 (`[profile.release]`)](#6-最高速最小バイナリを作るリリース最適化-profilerelease)
7. [Cargo.toml 実務テンプレート集](#7-cargotoml-実務テンプレート集)
8. [Cargo CLI 必須コマンド早見表](#8-cargo-cli-必須コマンド早見表)

---

## 1. Cargo の基本思想と標準ディレクトリ構成

### 1-1. 規約優先（Convention over Configuration）
Cargo は厳格なディレクトリ規約を持つため、`CMakeLists.txt` やレガシー `.csproj` のように「どのファイルをコンパイルするか」を明記する必要がありません。

```text
my_project/
├── Cargo.toml          # プロジェクト定義・依存関係マニフェスト (Git管理)
├── Cargo.lock          # 依存関係の完全な確定バージョン (バイナリならGit管理、ライブラリなら無視)
├── src/
│   ├── main.rs         # バイナリ (実行可能アプリ) のエントリーポイント
│   └── lib.rs          # クラスライブラリのエントリーポイント
├── src/bin/            # 複数の実行可能バイナリを追加する場合の置き場
├── tests/              # 外部からの結合テスト (Integration Tests)
├── examples/           # ライブラリの利用サンプル
└── benches/            # ベンチマークテスト (criterion 等)
```

### 1-2. `Cargo.toml` vs `Cargo.lock`
- **`Cargo.toml`**: 人間が編集する。依存パッケージのバージョン範囲（例: `serde = "1.0"`）を指定。
- **`Cargo.lock`**: Cargo が自動生成・更新する。実際に解決された確定バージョンとチェックサムが記録され、**全マシンで 100% 同一のビルド（再現性）** を保証する。

---

## 2. 最重要基本プロパティ (`[package]`)

```toml
[package]
name = "my_awesome_app"      # クレート名 (小文字スネークケース推奨)
version = "0.1.0"             # セマンティックバージョニング
edition = "2024"              # Rust エディション ("2021", "2024")
authors = ["Harun <harun@example.com>"]
description = "High performance systems application built with Rust."
license = "MIT OR Apache-2.0"
repository = "https://github.com/user/my_awesome_app"
readme = "README.md"
rust-version = "1.85.0"       # 最小サポート Rust バージョン (MSRV)
```

---

## 3. 依存関係の指定方法完全リファレンス (`[dependencies]`)

### 3-1. crates.io からの標準取得
```toml
[dependencies]
# 1. バージョン指定 (キャレット要求: ^1.0.0 と同等。後方互換のある最新版を自動取得)
serde = "1.0"

# 2. 機能フラグ (Features) の選択
tokio = { version = "1.38", features = ["full"] }

# 3. デフォルト機能の無効化 + 必要な機能だけ選択 (バイナリサイズ削減)
reqwest = { version = "0.12", default-features = false, features = ["json", "rustls-tls"] }
```

### 3-2. ローカルパス & Git リポジトリからの指定
```toml
[dependencies]
# 開発中のローカルクレートを参照
shared_kernel = { path = "../shared_kernel" }

# GitHub リポジトリから直接取得
custom_logger = { git = "https://github.com/company/custom_logger", branch = "main" }
```

### 3-3. 用途別依存関係 (`[dev-dependencies]`, `[build-dependencies]`)
- **`[dev-dependencies]`**: `cargo test` や `examples/` のみで使用するライブラリ（本番バイナリには含まれない）。
- **`[build-dependencies]`**: ビルドスクリプト（`build.rs`）の実行時にのみ使用するライブラリ。
- **`[target.'cfg(...)'.dependencies]`**: Windows 専用、Linux 専用などのプラットフォーム分岐。

---

## 4. マルチパッケージ・ワークスペース設計 (`[workspace]`)

大規模プロジェクトやマイクロサービスでは、**ルートに単一の `Cargo.lock` を共有するワークスペース（Workspace）** を構築します。

### ルートの `Cargo.toml`
```toml
[workspace]
members = [
    "crates/api_server",
    "crates/core_domain",
    "crates/db_infrastructure",
]
resolver = "2"

# ワークスペース全体で共有する依存関係のバージョンを一元管理 (Rust 1.64+)
[workspace.dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.38", features = ["full"] }
```

### 子クレートの `Cargo.toml`
```toml
[package]
name = "api_server"
version = "0.1.0"
edition = "2024"

[dependencies]
# ワークスペース定義のバージョンを継承！
serde = { workspace = true }
tokio = { workspace = true }
core_domain = { path = "../core_domain" }
```

---

## 5. 機能フラグによる条件付きコンパイル (`[features]`)

ライブラリの利用者が「必要な機能だけを ON にしてコンパイル」できるようにする機能です。

```toml
[features]
default = ["json"]             # cargo build 時にデフォルトで有効化される機能
json = ["dep:serde_json"]      # feature "json" が指定されたときだけ依存を追加
full = ["json", "metrics"]     # 複数機能のバンドルフラグ

[dependencies]
serde_json = { version = "1.0", optional = true } # optional = true で条件付き依存
```

### Rust コード側での条件分岐
```rust
#[cfg(feature = "json")]
pub fn serialize_to_json<T: serde::Serialize>(val: &T) -> String {
    serde_json::to_string(val).unwrap()
}
```

---

## 6. 最高速・最小バイナリを作るリリース最適化 (`[profile.release]`)

Rust の `cargo build --release` は標準でも高速ですが、以下のプロファイル設定を追加することで**極限のパフォーマンスとファイルサイズ削減**が達成できます。

```toml
[profile.release]
# 最適化レベル (0: なし, 1: 基本, 2: 推奨, 3: 最高速, "s"/"z": サイズ最小化)
opt-level = 3

# リンク時最適化 (LTO: Link-Time Optimization)
# クレート境界を越えてインライン展開を徹底的に行う (コンパイル時間は増えるが実行速度が大幅向上)
lto = true

# 単一のコード生成ユニットに統合してコンパイラのインライン化効率を最大化
codegen-units = 1

# パニック時にスタックを巻き戻さず、即座にプロセスをアボート (バイナリサイズの大幅削減)
panic = "abort"

# デバッグシンボルをバイナリから完全にストリップ (削除)
strip = true
```

---

## 7. Cargo.toml 実務テンプレート集

### テンプレートA: 本番グレード Web API / CLI ツール
```toml
[package]
name = "production_service"
version = "0.1.0"
edition = "2024"
authors = ["Harun"]

[dependencies]
tokio = { version = "1.38", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tracing = "0.1"
tracing-subscriber = "0.3"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true
```

---

## 8. Cargo CLI 必須コマンド早見表

| コマンド | 説明 |
| :--- | :--- |
| **`cargo run`** | デバッグビルドして即実行 |
| **`cargo check`** | バイナリを生成せず型・借用検査のみ実行 (**爆速で日常開発に必須**) |
| **`cargo build --release`** | 最高速最適化バイナリを `target/release/` にビルド |
| **`cargo test`** | 単体テスト・結合テスト・ドキュメントテストを一括実行 |
| **`cargo clippy`** | Rust 公式の強力な linter（コード品質・パフォーマンス改善提案） |
| **`cargo fmt`** | 公式スタイルガイドに沿って全ソースコードを自動フォーマット |
| **`cargo tree`** | プロジェクトの依存クレートの依存関係ツリーを表示 |
| **`cargo update`** | `Cargo.lock` 内の依存バージョンを `Cargo.toml` の範囲内で最新に更新 |
