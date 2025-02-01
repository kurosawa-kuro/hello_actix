
# Rustプロジェクトセットアップガイド

## 1. 開発環境の準備

### 1.1 必要なパッケージのインストール
以下のコマンドで、Rust開発に必要な基本ツールをインストールします。

```bash
sudo apt update
sudo apt install build-essential pkg-config libssl-dev
```

- `build-essential`: コンパイルに必要なツールチェーン
- `pkg-config`: ライブラリの設定を管理するツール
- `libssl-dev`: SSL/TLS関連の開発ライブラリ

---

## 2. Rustのインストール

### 2.1 Rustインストーラーの実行
以下のコマンドで、Rustのインストーラーをダウンロードして実行します。

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2.2 パスの更新
インストール後、以下のコマンドでパスを更新します。

```bash
source $HOME/.cargo/env
```

### 2.3 インストールの確認
以下のコマンドで、RustとCargoが正しくインストールされたことを確認します。

```bash
rustc --version
cargo --version
```

---

## 3. プロジェクトの作成と実行

### 3.1 プロジェクトの作成
以下のコマンドで、新しいRustプロジェクトを作成します。

```bash
cargo new hello_actix
cd hello_actix
```

- `cargo new`: 新しいRustプロジェクトを作成
- `hello_actix`: プロジェクト名（任意の名前を指定可能）

### 3.2 プロジェクトの実行
以下のコマンドで、プロジェクトをビルドして実行します。

```bash
cargo run
```

- サーバーが起動し、`http://localhost:8080`でアクセス可能になります。

### 3.3 動作確認
以下のコマンドで、サーバーが正しく動作していることを確認します。

```bash
curl http://localhost:8080
```

---

## 4. テストの実行

### 4.1 テストの実行
以下のコマンドで、プロジェクトのテストを実行します。

```bash
cargo test
```

- テストが成功すると、すべてのテストケースがパスしたことが表示されます。

---

## 5. プロジェクトの構成

### 5.1 主要ファイル
- `src/main.rs`: アプリケーションのエントリーポイント
- `Cargo.toml`: プロジェクトの設定と依存関係を定義
- `tests/`: テストコードを格納するディレクトリ

---

## 6. 次のステップ
- 新しいエンドポイントを追加するには、`src/main.rs`を編集します。
- 依存関係を追加するには、`Cargo.toml`の`[dependencies]`セクションにクレートを追加します。
- 詳細なドキュメントは、[Rust公式ドキュメント](https://doc.rust-lang.org/)を参照してください。

---

このリファクタリングにより、ドキュメントがより構造化され、各ステップの目的が明確になりました。
