//! cargo のビルドの出力先(ターゲットフォルダ)と、その中でインストーラーの材料と成果物が置かれる場所。

use std::path::{Path, PathBuf};

/// ビルドの出力先とは、cargo がビルドの成果物を書くフォルダ(ターゲットフォルダ)のことである。
/// 環境変数 CARGO_TARGET_DIR があればそれを使い、無ければリポジトリのルートの `target` を使う。
/// 前提: `.cargo/config.toml` の `build.target-dir` で出力先を変えていない。
pub struct ビルドの出力先(PathBuf);

impl ビルドの出力先 {
    /// 相対パスの CARGO_TARGET_DIR は、cargo と同じくリポジトリのルート(cargo を起動する作業ディレクトリ)から解く。
    pub fn 環境から決める(リポジトリルート: &Path) -> Self {
        let フォルダ = std::env::var_os("CARGO_TARGET_DIR")
            .map(PathBuf::from)
            .map_or_else(
                || リポジトリルート.join("target"),
                |指定| リポジトリルート.join(指定),
            );
        Self(フォルダ)
    }

    /// release のビルドで作られるアプリの実行ファイルのパス。
    pub fn アプリの実行ファイル(&self) -> PathBuf {
        self.0
            .join("release")
            .join(format!("clip_viewer{}", std::env::consts::EXE_SUFFIX))
    }

    /// 第三者のライセンス表示を書くファイル。インストーラーの材料になり、インストール先へ同梱する。
    pub fn ライセンス表示のファイル(&self) -> PathBuf {
        self.インストーラーの置き場所()
            .join("THIRD-PARTY-NOTICES.html")
    }

    /// 組み立てたインストーラー(setup.exe)を置くフォルダ。
    pub fn インストーラーの置き場所(&self) -> PathBuf {
        self.0.join("installer")
    }
}
