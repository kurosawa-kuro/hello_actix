# 開発ツールのインストール
sudo apt update
sudo apt install build-essential

# 他の必要なパッケージもインストール
sudo apt install pkg-config libssl-dev

# Rust インストーラーのダウンロードと実行
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# パスを更新
source $HOME/.cargo/env

# インストールの確認
rustc --version
cargo --version

# プロジェクトディレクトリの作成と移動
cargo new hello_actix
cd hello_actix

cargo run

curl http://localhost:8080