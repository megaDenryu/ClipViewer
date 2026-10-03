//! 起動の手順の `overlays` の錠の試験。スタックのライブラリの錠を取れたアプリだけが続けて `overlays` の錠を試して取り、
//! 錠を取れなかったアプリは取りに行かずに読み取り専用になり、置き場所の無いアプリは重ね合わせのライブラリを持たないことを確かめる。
//! 利用者の %APPDATA% のライブラリと受け口を使わない。参照: _doc/設計/同時再生.md 3-2(触れる所の表の起動の手順の行)
#![allow(clippy::expect_used)]

use std::path::PathBuf;

use clip_library::{
    重ね合わせの書き込みの許し, 重ね合わせの錠を取れない理由
};
use video_source::実行ファイルの検索パス;

use super::super::environment::起動時の環境;
use super::super::launch_plan::起動の手順;
use super::super::launch_preparation::起動の準備;

fn 決める(アプリのデータ: Option<PathBuf>) -> 起動の準備 {
    起動の手順::作成する(起動時の環境 {
        アプリのデータのフォルダ: アプリのデータ,
        ローカルのアプリのデータのフォルダ: None,
        ffmpegのフォルダ: None,
        検索パス: 実行ファイルの検索パス::作成する(Vec::new()),
        起動の引数: Vec::new(),
        フレームの時間を測る頼み: None,
    })
    .決める()
}

#[test]
fn スタックのライブラリの錠を取れたアプリだけがoverlaysの錠を試して取る() {
    let フォルダ = std::env::temp_dir().join(format!(
        "clip_viewer_起動の手順_overlays_{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&フォルダ);
    let 一つ目 = 決める(Some(フォルダ.clone()));
    assert!(
        一つ目
            .ライブラリ
            .as_ref()
            .is_some_and(|組| 組.重ね合わせ.書けるか())
    );
    assert!(
        フォルダ
            .join("ClipViewer")
            .join("library")
            .join("overlays")
            .join("ClipViewer.lock")
            .is_file()
    );
    let 二つ目 = 決める(Some(フォルダ.clone()));
    assert!(!二つ目.ライブラリの錠を持つか());
    assert!(matches!(
        二つ目.ライブラリ.as_ref().map(|組| 組.重ね合わせ.許し()),
        Some(重ね合わせの書き込みの許し::読むだけ(
            重ね合わせの錠を取れない理由::スタックのライブラリの錠を持っていない
        ))
    ));
    drop((一つ目, 二つ目));
    let _ = std::fs::remove_dir_all(&フォルダ);
}

#[test]
fn 置き場所の無いアプリは錠を試したライブラリの組を持たない() {
    assert!(決める(None).ライブラリ.is_none());
}
