//! 配線が重ね合わせの作業場へ渡す、重ね合わせのライブラリの様子の試験。クリップビューアーが持つ裏で動く重ね合わせのライブラリが読み取り専用なら、
//! 重ね合わせの作業場の画面にその旨が出て、書けるなら出ないことを確かめる。利用者の %APPDATA% のライブラリを使わず、一時フォルダを置き場所にする。
//! 参照: _doc/設計/同時再生.md 2-7・6節の第16段階
#![allow(clippy::expect_used)]

use std::path::PathBuf;
use std::time::Instant;

use clip_library::{
    スタックのライブラリ, ライブラリのフォルダ, 裏で動く重ね合わせのライブラリ,
    錠を試したライブラリの組,
};
use eframe::egui;

use super::super::クリップビューアー;
use super::launch_requests_test_support::試験のビューアー;

/// 読み取り専用の旨の文の書き出し。理由は続けて括弧の中に出る。
const 読み取り専用の旨: &str = "重ね合わせのライブラリ: 読み取り専用";

fn 一時フォルダ(名前: &str) -> PathBuf {
    let フォルダ = std::env::temp_dir().join(format!(
        "clip_viewer_重ね合わせのライブラリ_{名前}_{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&フォルダ);
    フォルダ
}

fn 二つの錠を試す(フォルダ: &std::path::Path) -> 錠を試したライブラリの組 {
    スタックのライブラリ::作成する(ライブラリのフォルダ::作成する(
        フォルダ.to_path_buf(),
    ))
    .錠を試す()
    .重ね合わせの錠も試す()
}

/// 重ね合わせの作業場へ移ったビューアーの画面を1フレーム描き、描いた文字を並べる。
fn 重ね合わせの作業場で描いた文字(
    ビューアー: &mut クリップビューアー,
) -> Vec<String> {
    ビューアー.重ね合わせの作業場へ移る(Instant::now());
    let 本体 = egui::Context::default();
    let 出力 = 本体.run(egui::RawInput::default(), |本体| {
        egui::CentralPanel::default().show(本体, |ui| {
            let _ = ビューアー.画面().描画して集める(ui);
        });
    });
    出力
        .shapes
        .iter()
        .filter_map(|切り抜いた形| match &切り抜いた形.shape {
            egui::Shape::Text(文字の形) => Some(文字の形.galley.text().to_string()),
            _ => None,
        })
        .collect()
}

#[test]
fn 読み取り専用で起動したアプリの重ね合わせの作業場にはその旨が出て_書けるアプリには出ない() {
    let フォルダ = 一時フォルダ("読み取り専用");
    let 一つ目 = 二つの錠を試す(&フォルダ);
    let 二つ目 = 二つの錠を試す(&フォルダ);
    let mut 書ける = 試験のビューアー();
    書ける.重ね合わせのライブラリ = Some(
        裏で動く重ね合わせのライブラリ::錠を試した後で起動する(
            一つ目.重ね合わせ,
        ),
    );
    let mut 読むだけ = 試験のビューアー();
    読むだけ.重ね合わせのライブラリ = Some(
        裏で動く重ね合わせのライブラリ::錠を試した後で起動する(
            二つ目.重ね合わせ,
        ),
    );
    assert!(
        重ね合わせの作業場で描いた文字(&mut 読むだけ)
            .iter()
            .any(|文字| 文字.starts_with(読み取り専用の旨))
    );
    assert!(
        !重ね合わせの作業場で描いた文字(&mut 書ける)
            .iter()
            .any(|文字| 文字.starts_with("重ね合わせのライブラリ:"))
    );
    drop((書ける, 読むだけ, 一つ目.スタック, 二つ目.スタック));
    let _ = std::fs::remove_dir_all(&フォルダ);
}
