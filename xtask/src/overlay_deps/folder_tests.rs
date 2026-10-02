//! 依存の向きの検査の試験のうち、フォルダを調べる試験。重ね合わせの作業場の層の下のファイルだけを数え、禁じた参照と読めないファイルを報告し、
//! 層のフォルダが無ければ失敗することと、このリポジトリの重ね合わせの作業場の層が決まりを守っていることを確かめる。
#![allow(clippy::expect_used)]

use std::path::PathBuf;

use super::findings::見つけたこと;
use super::source_root::ソースルート;

/// 一時フォルダに `overlay` の下のファイルを置いたソースルート。
fn 一時のソースルート(
    名前: &str,
    ファイルの並び: &[(&str, &[u8])],
) -> (ソースルート, PathBuf) {
    let フォルダ =
        std::env::temp_dir().join(format!("xtask_overlay_deps_{名前}_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&フォルダ);
    for (位置, 中身) in ファイルの並び {
        let パス = フォルダ.join(位置);
        std::fs::create_dir_all(パス.parent().expect("親")).expect("フォルダを作れる");
        std::fs::write(&パス, 中身).expect("書ける");
    }
    (ソースルート::フォルダから作る(フォルダ.clone()), フォルダ)
}

#[test]
fn フォルダの下のファイルを数え_禁じた参照と文字として読めないファイルを報告する() {
    let (調べるソースルート, フォルダ) = 一時のソースルート(
        "報告",
        &[
            ("overlay/mod.rs", b"mod view;\nuse crate::state;\n"),
            ("overlay/view/mod.rs", b"use super::super::overlay;\n"),
            ("overlay/broken.rs", &[0xff, 0xfe]),
            ("state/mod.rs", b"use crate::view;\n"),
        ],
    );
    let 結果 = 調べるソースルート
        .重ね合わせの層を検査する()
        .expect("調べられる");
    assert_eq!(結果.調べたファイルの数, 3);
    let 文の並び: Vec<String> = 結果
        .見つけたことの並び
        .iter()
        .map(見つけたこと::to_string)
        .collect();
    assert_eq!(文の並び.len(), 2, "{文の並び:?}");
    assert!(文の並び[0].contains("UTF-8 でない"), "{文の並び:?}");
    assert!(
        文の並び[1].contains(":2: スタックの作業場の crate::state を使っている"),
        "{文の並び:?}"
    );
    let _ = std::fs::remove_dir_all(&フォルダ);
}

#[test]
fn 重ね合わせの層のフォルダが無ければ失敗する() {
    let (調べるソースルート, フォルダ) =
        一時のソースルート("無い", &[("state/mod.rs", b"")]);
    assert!(調べるソースルート.重ね合わせの層を検査する().is_err());
    let _ = std::fs::remove_dir_all(&フォルダ);
}

#[test]
fn このリポジトリの重ね合わせの層はスタックの作業場のモジュールを使っていない() {
    let 調べるソースルート = ソースルート::リポジトリから決める(
        &crate::verify::リポジトリルートを求める(),
    );
    let 結果 = 調べるソースルート
        .重ね合わせの層を検査する()
        .expect("調べられる");
    assert!(結果.調べたファイルの数 > 0);
    let 文の並び: Vec<String> = 結果
        .見つけたことの並び
        .iter()
        .map(見つけたこと::to_string)
        .collect();
    assert!(文の並び.is_empty(), "{文の並び:?}");
}
