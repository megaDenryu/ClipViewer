//! 起動の手順の試験。一時フォルダをアプリのデータのフォルダにして、1つ目・2つ目・1つ目が応答しないとき・1つ目が落ちた後の起動を確かめる。
//! 利用者の %APPDATA% のライブラリと受け口を使わない。
#![allow(clippy::expect_used)]

use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use eframe::egui;
use video_source::実行ファイルの検索パス;

use super::environment::起動時の環境;
use super::launch_plan::起動の手順;
use super::launch_preparation::{起動の決着, 起動の準備};
use crate::launch::起動の頼み;

fn 一時フォルダ(名前: &str) -> PathBuf {
    let フォルダ = std::env::temp_dir().join(format!(
        "clip_viewer_起動の手順_{名前}_{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&フォルダ);
    フォルダ
}

fn 決める(アプリのデータ: &Path, 引数: &[&str]) -> 起動の決着 {
    起動の手順::作成する(起動時の環境 {
        アプリのデータのフォルダ: Some(アプリのデータ.to_path_buf()),
        ローカルのアプリのデータのフォルダ: None,
        ffmpegのフォルダ: None,
        検索パス: 実行ファイルの検索パス::作成する(Vec::new()),
        起動の引数: 引数.iter().map(OsString::from).collect(),
    })
    .決める()
}

fn 起動する準備(決着: 起動の決着) -> 起動の準備 {
    match 決着 {
        起動の決着::起動する(準備) => *準備,
        起動の決着::一つ目へ渡した => panic!("起動するはずが、1つ目へ渡した"),
    }
}

#[test]
fn 二つ目は一つ目へ渡して終わり_一つ目が落ちた後は一つ目として起動する() {
    let フォルダ = 一時フォルダ("渡す");
    let 一つ目 = 起動する準備(決める(&フォルダ, &[]));
    assert!(
        一つ目
            .ライブラリ
            .as_ref()
            .is_some_and(|試した| 試した.書けるか())
    );
    let 受け取っている = 一つ目
        .受け口
        .expect("1つ目は受け口を開く")
        .受け取り始める(egui::Context::default())
        .expect("受け取り始められる");
    assert!(matches!(
        決める(&フォルダ, &[r"C:\二つ目.mp4"]),
        起動の決着::一つ目へ渡した
    ));
    assert_eq!(
        受け取っている.届いた頼みを受け取る(),
        vec![起動の頼み::引数から読む(
            [OsString::from(r"C:\二つ目.mp4")]
        )]
    );
    drop(受け取っている);
    drop(一つ目.ライブラリ);
    let 次 = 起動する準備(決める(&フォルダ, &[]));
    assert!(
        次.ライブラリ
            .as_ref()
            .is_some_and(|試した| 試した.書けるか())
    );
    assert!(次.受け口.is_some() && 次.知らせ.is_empty());
    drop(次);
    let _ = std::fs::remove_dir_all(&フォルダ);
}

#[test]
fn 一つ目が応答しなければ期限の後に読み取り専用で起動する() {
    let フォルダ = 一時フォルダ("応答しない");
    let 一つ目 = 起動する準備(決める(&フォルダ, &[]));
    let 始め = Instant::now();
    let 二つ目 = 起動する準備(決める(&フォルダ, &[r"C:\二つ目.mp4"]));
    assert!(
        始め.elapsed() < Duration::from_secs(5),
        "期限を大きく過ぎて待たない"
    );
    assert!(
        二つ目
            .ライブラリ
            .as_ref()
            .is_some_and(|試した| 試した.別のアプリが持っているか())
    );
    assert!(二つ目.受け口.is_none());
    assert!(
        二つ目.知らせ[0].contains("読み取り専用"),
        "{:?}",
        二つ目.知らせ
    );
    assert!(matches!(二つ目.頼み, 起動の頼み::動画を開く { .. }));
    drop((一つ目, 二つ目));
    let _ = std::fs::remove_dir_all(&フォルダ);
}
