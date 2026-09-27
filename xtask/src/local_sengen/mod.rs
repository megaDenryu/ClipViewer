//! `local-sengen` コマンド: push する前の手元の SengenEgui で ClipViewer を試すときだけ使う入口である。
//! 普段は SengenEgui を push してから crates/clip_viewer/Cargo.toml の sengen_egui の rev を上げ、この入口を使わない。
//! cargo の `--config` で、その実行の間だけ SengenEgui の git 依存を手元のフォルダへ差し替える。差し替えた cargo は
//! Cargo.lock の sengen_egui から source の行を消すため、見張り役が実行の前の中身を覚え、終わったら書き戻す(`guard.rs`)。

mod guard;
mod sengen_folder;

pub use guard::{見張り役として待って書き戻す, 見張り役のコマンド名};

use crate::verify::{リポジトリルートを求める, 工程を実行する};
use guard::依存の固定ファイルの見張り;
use sengen_folder::{フォルダを指す環境変数, 手元のSengenEguiのフォルダ};

/// 手元の SengenEgui へ差し替えた cargo を、渡された引数で1回実行する。実行の成否を問わず、Cargo.lock を実行の前の中身に戻す。
pub fn 手元のsengen_eguiへ差し替えてcargoを実行する(
    cargoの引数: &[String],
) -> Result<(), String> {
    match cargoの引数.first().map(String::as_str) {
        None => {
            return Err(
                "cargo の引数が無い。例: cargo xtask local-sengen run --package clip_viewer"
                    .to_string(),
            );
        }
        // xtask のコマンドの中で起こす cargo には --config が届かず、差し替えが一部にしか効かないため断る。
        Some("xtask") => {
            return Err("local-sengen は cargo の引数だけを受ける。xtask のコマンドは中で起こす cargo に差し替えが届かない。例: cargo xtask local-sengen test --workspace".to_string());
        }
        Some(_) => {}
    }
    let リポジトリルート = リポジトリルートを求める();
    let フォルダ = 手元のSengenEguiのフォルダ::見つける(
        &リポジトリルート,
        std::env::var_os(フォルダを指す環境変数),
    )?;
    println!(
        "手元の SengenEgui を使う: {}",
        フォルダ.フォルダ().display()
    );
    let 見張り = 依存の固定ファイルの見張り::起こす()?;
    let 設定 = フォルダ.差し替えの設定();
    let 引数: Vec<&str> = ["--config", 設定.as_str()]
        .into_iter()
        .chain(cargoの引数.iter().map(String::as_str))
        .collect();
    let 実行の結果 = 工程を実行する(&引数, &リポジトリルート, None);
    let 書き戻しの結果 = 見張り.書き戻させる();
    match (実行の結果, 書き戻しの結果) {
        (Err(実行の失敗), Err(書き戻しの失敗)) => {
            Err(format!("{実行の失敗}。さらに {書き戻しの失敗}"))
        }
        (実行の結果, 書き戻しの結果) => 実行の結果.and(書き戻しの結果),
    }
}
