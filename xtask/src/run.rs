//! `run` コマンド: ClipViewer のアプリをビルドして起動する。ウインドウを閉じるまで戻らない。

use crate::verify::{リポジトリルートを求める, 工程を実行する};

/// crates/clip_viewer を開発のビルドで起動する。依存クレートはルートの Cargo.toml の設定で最適化される。
pub fn アプリを起動する() -> Result<(), String> {
    工程を実行する(
        &["run", "--package", "clip_viewer"],
        &リポジトリルートを求める(),
        None,
    )
}
