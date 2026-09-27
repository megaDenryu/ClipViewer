//! インストーラーへ入れる release のビルドに渡す rustc の指定。
//! (1) C の実行時ライブラリを実行ファイルへ静的に取り込む。Visual C++ の再頒布パッケージ(vcruntime140.dll)が入っていない PC でも起動できるようにするためである。
//! (2) 実行ファイルに埋まるソースのパス(panic の位置情報)から、ビルドした計算機の利用者のフォルダとリポジトリの置き場所を消す。
//! 利用者の名などの開発機の情報を、配る実行ファイルへ漏らさないためである。

use std::ffi::OsString;
use std::path::{Path, PathBuf};

/// rustc の指定を渡す環境変数。区切りに 0x1f を使うため、空白を含むパスもそのまま渡せる(RUSTFLAGS は空白で区切る)。
pub const 指定を渡す環境変数: &str = "CARGO_ENCODED_RUSTFLAGS";

/// 置き換えるフォルダとは、ビルドした計算機の上で、実行ファイルへ埋めたくないフォルダの組のことである。
pub struct 置き換えるフォルダ {
    利用者のフォルダ: Option<PathBuf>,
    cargoのフォルダ: Option<PathBuf>,
    リポジトリルート: PathBuf,
}

impl 置き換えるフォルダ {
    /// 環境変数 USERPROFILE と CARGO_HOME から読む。
    pub fn 環境から読む(リポジトリルート: &Path) -> Self {
        Self {
            利用者のフォルダ: std::env::var_os("USERPROFILE").map(PathBuf::from),
            cargoのフォルダ: std::env::var_os("CARGO_HOME").map(PathBuf::from),
            リポジトリルート: リポジトリルート.to_path_buf(),
        }
    }

    /// 置き換えの並び。rustc は最後に一致した置き換えを使うため、広い範囲(利用者のフォルダ)を先に、狭い範囲(リポジトリのルート)を後に並べる。
    /// リポジトリの1つ上のフォルダを置き換えるのは、隣のフォルダに置いた依存(手元の SengenEgui 等)でビルドしたときにも開発機のパスを残さないためである。
    /// ただしそのフォルダが利用者のフォルダか cargo のフォルダを含む(同じか上位の)ときは置き換えない。含むときに置き換えると、
    /// 後に書いた置き換えが勝ち、`C:\Users\<名>\.cargo` が `..\Users\<名>\.cargo` になって利用者の名が残るためである。
    fn 置き換えの並び(&self) -> Vec<(PathBuf, &'static str)> {
        let 私的なフォルダ: Vec<&PathBuf> = [&self.利用者のフォルダ, &self.cargoのフォルダ]
            .into_iter()
            .flatten()
            .collect();
        let 一つ上 = self
            .リポジトリルート
            .parent()
            .filter(|上| !私的なフォルダ.iter().any(|私的| 私的.starts_with(上)))
            .map(Path::to_path_buf);
        [
            (self.利用者のフォルダ.clone(), "~"),
            (self.cargoのフォルダ.clone(), "cargo"),
            (一つ上, ".."),
            (Some(self.リポジトリルート.clone()), "."),
        ]
        .into_iter()
        .filter_map(|(元, 先)| 元.map(|元| (元, 先)))
        .collect()
    }
}

/// リリースのビルドの指定とは、release のビルドで rustc へ渡す指定の並びのことである。
pub struct リリースのビルドの指定(Vec<String>);

impl リリースのビルドの指定 {
    pub fn 作成する(フォルダ: &置き換えるフォルダ) -> Self {
        let mut 指定 = vec!["-C".to_string(), "target-feature=+crt-static".to_string()];
        for (元, 先) in フォルダ.置き換えの並び() {
            指定.push(format!("--remap-path-prefix={}={先}", 元.display()));
        }
        Self(指定)
    }

    /// 環境変数 CARGO_ENCODED_RUSTFLAGS へ渡す値。
    pub fn 環境変数へ渡す値(&self) -> OsString {
        OsString::from(self.0.join("\u{1f}"))
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::置き換えるフォルダ;

    fn フォルダ(利用者: &str, リポジトリ: &str) -> 置き換えるフォルダ {
        置き換えるフォルダ {
            利用者のフォルダ: Some(PathBuf::from(利用者)),
            cargoのフォルダ: None,
            リポジトリルート: PathBuf::from(リポジトリ),
        }
    }

    #[test]
    fn 一つ上が利用者のフォルダと別の経路なら置き換える() {
        let 並び = フォルダ(r"C:\Users\a", r"C:\devs\ClipViewer").置き換えの並び();
        assert!(並び.contains(&(PathBuf::from(r"C:\devs"), "..")));
    }

    #[test]
    fn 一つ上が利用者のフォルダを含むなら置き換えない() {
        let 並び = フォルダ(r"C:\Users\a", r"C:\ClipViewer").置き換えの並び();
        assert!(並び.iter().all(|(_, 先)| *先 != ".."));
        let 同じ = フォルダ(r"C:\Users\a", r"C:\Users\a\ClipViewer").置き換えの並び();
        assert!(同じ.iter().all(|(_, 先)| *先 != ".."));
    }
}
