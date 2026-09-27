//! 第三者のライセンス表示(THIRD-PARTY-NOTICES.html)を cargo-about で作る。対象は配る実行ファイル(crates/clip_viewer)の依存である。
//! 設定と雛形は `installer/notices/` にある。cargo-about はライセンスを決められなかったクレートを警告だけ出して表示から外すため、
//! ここでその警告を読み、1つでもあれば失敗にする(黙って表示から抜けたまま配らないため)。

use std::path::{Path, PathBuf};
use std::process::Command;

/// cargo-about がライセンスを決められなかったクレートを知らせる警告の文言。この後に `'<名> <版>'` が続く。
const 決められなかった印: &str = "unable to synthesize license expression for '";

/// cargo-about を入れる手順。
const 入れる手順: &str = "cargo-about が無い。`cargo install cargo-about --locked --features cli` で入れてから実行し直す";

/// ライセンス表示の作り手とは、使えることを確かめた cargo-about と、設定を読むリポジトリのルートの組のことである。
pub struct ライセンス表示の作り手 {
    リポジトリルート: PathBuf,
}

impl ライセンス表示の作り手 {
    /// cargo-about が使えるかを確かめる。installer は長い release のビルドの前にこれを呼び、無いと分かっている状態で待たせない。
    pub fn 探す(リポジトリルート: &Path) -> Result<Self, String> {
        let 使えるか = Command::new("cargo")
            .args(["about", "--version"])
            .output()
            .is_ok_and(|結果| 結果.status.success());
        if 使えるか {
            Ok(Self {
                リポジトリルート: リポジトリルート.to_path_buf(),
            })
        } else {
            Err(入れる手順.to_string())
        }
    }

    /// 第三者のライセンス表示を書き出す先へ作り、そのパスを返す。
    pub fn 作る(&self, 書き出す先: PathBuf) -> Result<PathBuf, String> {
        let 設定のフォルダ = self.リポジトリルート.join("installer").join("notices");
        if let Some(フォルダ) = 書き出す先.parent() {
            std::fs::create_dir_all(フォルダ)
                .map_err(|原因| format!("{}: {原因}", フォルダ.display()))?;
        }
        println!(
            "> cargo about generate (書き出す先: {})",
            書き出す先.display()
        );
        let 結果 = Command::new("cargo")
            .args(["about", "generate", "-c"])
            .arg(設定のフォルダ.join("about.toml"))
            .arg("-m")
            .arg(
                self.リポジトリルート
                    .join("crates")
                    .join("clip_viewer")
                    .join("Cargo.toml"),
            )
            .arg("-o")
            .arg(&書き出す先)
            .arg(設定のフォルダ.join("about.hbs"))
            .current_dir(&self.リポジトリルート)
            .output()
            .map_err(|原因| format!("cargo の起動に失敗した: {原因}"))?;
        let 警告 = String::from_utf8_lossy(&結果.stderr);
        if !結果.status.success() {
            return Err(format!(
                "cargo about generate が失敗した ({})\n{警告}",
                結果.status
            ));
        }
        決められなかったクレートを確かめる(&警告)?;
        Ok(書き出す先)
    }
}

/// 警告から、ライセンスを決められなかったクレートの名を集め、1つでもあれば失敗にする。
fn 決められなかったクレートを確かめる(警告: &str) -> Result<(), String> {
    let 決められなかった: Vec<&str> = 警告
        .lines()
        .filter_map(|行| 行.split_once(決められなかった印))
        .filter_map(|(_, 残り)| 残り.split(' ').next())
        .collect();
    if 決められなかった.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "ライセンスを決められず、表示から抜けたクレートがある: {}。installer/notices/about.toml の clarify で本文を示すか、依存を見直す",
            決められなかった.join(", ")
        ))
    }
}
