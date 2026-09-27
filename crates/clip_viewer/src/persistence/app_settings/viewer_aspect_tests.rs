//! 出力のアスペクト比の最初の値の試験。何も覚えていないときはクロップ枠に合わせ、settings.json に覚えた値があればその値で始める。
#![allow(clippy::expect_used)]

use clip_domain::アスペクト比設定;

use super::format::設定のファイルの本文;
use crate::state::出力の設定;
use crate::viewer_settings::見る側の設定;

fn 本文から読んだアスペクト比(本文: &str) -> アスペクト比設定 {
    設定のファイルの本文::作成する(本文.to_string())
        .最新の設定として読む()
        .expect("読める")
        .見る側の設定()
        .アスペクト比
}

#[test]
fn 何も覚えていないときのアスペクト比はクロップ枠に合わせる() {
    assert_eq!(
        見る側の設定::既定.アスペクト比,
        アスペクト比設定::クロップ連動
    );
    assert_eq!(
        出力の設定::default().アスペクト比,
        アスペクト比設定::クロップ連動
    );
    let 項目の無い本文 = r#"{"format":"ClipViewer.settings","version":1,"volume":0.5}"#;
    assert_eq!(
        本文から読んだアスペクト比(項目の無い本文),
        アスペクト比設定::クロップ連動
    );
}

#[test]
fn 覚えた固定の比があればその比で始める() {
    let 本文 = r#"{"format":"ClipViewer.settings","version":1,"aspectRatio":"16:9"}"#;
    assert_eq!(
        本文から読んだアスペクト比(本文),
        アスペクト比設定::ワイド16対9
    );
}
