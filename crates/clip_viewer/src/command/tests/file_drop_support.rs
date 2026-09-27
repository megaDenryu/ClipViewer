//! ダイアログを開いている間に落としたファイルの試験の道具。設定ファイルを一時フォルダへ置いて1回に落とす応答を作ることと、
//! 閉じてから落とし直す知らせだけが1件出ていることを確かめることを受け持つ。
#![allow(clippy::expect_used)]

use super::library_support::試験のライブラリ;
use crate::command::{ファイルの操作, 一度に落としたファイル, 応答};
use crate::state::アプリの状態;

pub(super) const 閉じてから落とし直す知らせ: &str =
    "開いているダイアログを閉じてから、もう一度ドラッグ＆ドロップしてください";

/// クリップを2つ持つ設定ファイルの本文。
pub(super) const 二つの設定: &str = r#"{"application":"ModifierVideoStack","version":"4.0","expectedVideoName":"sample.mp4",
"modifiers":[{"id":"mod-1-1","name":"一つ目","active":true,"start":1.5,"end":3,"repeat":1,
"crop":{"x":0,"y":0,"w":100,"h":100},"triggerEvent":"none"},
{"id":"mod-1-2","name":"二つ目","active":true,"start":4,"end":5,"repeat":1,
"crop":{"x":0,"y":0,"w":100,"h":100},"triggerEvent":"none"}]}"#;

/// クリップを1つ持つ設定ファイルの本文。
pub(super) const 一つの設定: &str = r#"{"application":"ModifierVideoStack","version":"4.0","expectedVideoName":"sample.mp4",
"modifiers":[{"id":"mod-1-1","name":"一つ目","active":true,"start":1.5,"end":3,"repeat":1,
"crop":{"x":0,"y":0,"w":100,"h":100},"triggerEvent":"none"}]}"#;

pub(super) fn 設定ファイルを置いて落とす(
    試験: &試験のライブラリ,
    ファイル: &[(&str, &str)],
) -> 応答 {
    let パス一覧 = ファイル
        .iter()
        .map(|(名前, 本文)| {
            let パス = 試験.一時フォルダ.join(名前);
            std::fs::write(&パス, 本文).expect("置ける");
            パス
        })
        .collect();
    応答::ファイル(ファイルの操作::落としたファイルを開く(
        一度に落としたファイル::作成する(パス一覧),
    ))
}

/// 閉じてから落とし直す知らせだけが1件出ていることを確かめる。
pub(super) fn 知らせが1件だけ出ている(状態: &アプリの状態) {
    assert_eq!(
        状態.通知.最新().map(|(_, 文面)| 文面.to_string()),
        Some(閉じてから落とし直す知らせ.to_string())
    );
    assert_eq!(状態.通知.並べる通知().len(), 1);
}
