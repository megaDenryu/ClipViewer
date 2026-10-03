//! `overlays` の置き去りの書きかけの片付けの試験。書けるアプリは一覧を作る前に、対応する重ね合わせのファイルが無く、
//! 最後に書き換えてから1日より古い `*.json.tmp` だけを消す。対応するファイルがある書きかけと、新しい書きかけは残す。
#![allow(clippy::expect_used)]

mod support;

use std::time::{Duration, SystemTime};

use clip_library::{
    保存の番号, 裏で動く重ね合わせのライブラリ, 重ね合わせのライブラリへの頼み
};
use support::overlay::重ね合わせを作る;
use support::一時のライブラリ;

fn 書ける裏で動く重ね合わせのライブラリ(
    一時: &一時のライブラリ,
) -> 裏で動く重ね合わせのライブラリ {
    裏で動く重ね合わせのライブラリ::錠を試した後で起動する(
        一時
            .ライブラリ
            .clone()
            .錠を試す()
            .重ね合わせの錠も試す()
            .重ね合わせ,
    )
}

#[test]
fn 書けるアプリは対応するファイルが無く1日より古い書きかけだけを消す() {
    let 一時 = 一時のライブラリ::作る("重ね合わせの書きかけを消す");
    let 裏 = 書ける裏で動く重ね合わせのライブラリ(&一時);
    let 重ね合わせ = 重ね合わせを作る("overlay-1-1", "甲", 一時.動画を置く("a.mp4"));
    裏.頼む(重ね合わせのライブラリへの頼み::保存する(
        重ね合わせ,
        保存の番号::default().次(),
    ))
    .expect("送れる");
    裏.書き込みを終えるまで待つ();
    let フォルダ = 裏.フォルダ().パス();
    for 名前 in [
        "overlay-1-1.json.tmp",
        "overlay-9-9.json.tmp",
        "overlay-8-8.json.tmp",
    ] {
        std::fs::write(フォルダ.join(名前), b"{").expect("置ける");
    }
    let 二日前 = SystemTime::now() - Duration::from_secs(2 * 24 * 60 * 60);
    for 名前 in ["overlay-1-1.json.tmp", "overlay-9-9.json.tmp"] {
        std::fs::File::options()
            .write(true)
            .open(フォルダ.join(名前))
            .and_then(|ファイル| ファイル.set_modified(二日前))
            .expect("書き換えた日時を変えられる");
    }
    裏.頼む(重ね合わせのライブラリへの頼み::一覧を作る)
        .expect("頼める");
    裏.走査も終えるまで待つ();
    assert_eq!(
        一時.重ね合わせのファイル名の一覧(),
        vec![
            "ClipViewer.lock",
            "overlay-1-1.json",
            "overlay-1-1.json.tmp",
            "overlay-8-8.json.tmp",
        ]
    );
}
