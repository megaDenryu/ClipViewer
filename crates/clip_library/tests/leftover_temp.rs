//! 置き去りの書きかけのファイルの片付けの試験。書けるアプリは一覧を作る前に、対応するスタックのファイルが無く、
//! 最後に書き換えてから1日より古い `*.json.tmp` だけを消す。新しい書きかけ(初めての保存の置き換えに失敗した唯一の写しでありうる)と、
//! 対応するスタックのファイルがある書きかけと、ほかのファイルは残す。読み取り専用のアプリは消さない。スタックを削除すると書きかけも消す。
#![allow(clippy::expect_used)]

mod support;

use std::time::{Duration, SystemTime};

use clip_library::{ライブラリへの頼み, 裏で動くライブラリ};
use support::{スタックを作る, 一時のライブラリ, 識別子};

/// スタックを1つ保存し、そのスタックの書きかけと、スタックのファイルが無い古い書きかけと新しい書きかけと、関係の無いファイルを置く。
/// 古い書きかけは、最後に書き換えた日時を2日前にする。
fn 書きかけを置く(一時: &一時のライブラリ) {
    let スタック = スタックを作る("stack-1-1", "甲", 一時.動画を置く("a.mp4"));
    一時.ライブラリ.保存する(&スタック).expect("保存");
    let フォルダ = 一時.ライブラリ.フォルダ().パス();
    for 名前 in [
        "stack-1-1.json.tmp",
        "stack-9-9.json.tmp",
        "stack-8-8.json.tmp",
        "memo.txt.tmp",
    ] {
        std::fs::write(フォルダ.join(名前), b"{").expect("置ける");
    }
    let 二日前 = SystemTime::now() - Duration::from_secs(2 * 24 * 60 * 60);
    std::fs::File::options()
        .write(true)
        .open(フォルダ.join("stack-9-9.json.tmp"))
        .and_then(|ファイル| ファイル.set_modified(二日前))
        .expect("書き換えた日時を変えられる");
}

fn 一覧を作らせて待つ(裏で動くライブラリ: &裏で動くライブラリ) {
    裏で動くライブラリ
        .頼む(ライブラリへの頼み::一覧を作る)
        .expect("頼める");
    裏で動くライブラリ.走査も終えるまで待つ();
}

#[test]
fn 書けるアプリはスタックのファイルが無く1日より古い書きかけだけを消す() {
    let 一時 = 一時のライブラリ::作る("置き去りの書きかけを消す");
    書きかけを置く(&一時);
    let 裏で動くライブラリ = 裏で動くライブラリ::起動する(一時.ライブラリ.clone());
    一覧を作らせて待つ(&裏で動くライブラリ);
    assert_eq!(
        一時.ファイル名の一覧(),
        vec![
            "ClipViewer.lock".to_string(),
            "memo.txt.tmp".to_string(),
            "stack-1-1.json".to_string(),
            "stack-1-1.json.tmp".to_string(),
            "stack-8-8.json.tmp".to_string(),
        ]
    );
}

#[test]
fn 読み取り専用のアプリは置き去りの書きかけを消さない() {
    let 一時 = 一時のライブラリ::作る("置き去りの書きかけを消さない");
    書きかけを置く(&一時);
    let _書ける方 = 一時.ライブラリ.フォルダ().錠を取る().expect("錠を取れる");
    let 読むだけ = 裏で動くライブラリ::起動する(一時.ライブラリ.clone());
    一覧を作らせて待つ(&読むだけ);
    assert!(
        一時
            .ファイル名の一覧()
            .contains(&"stack-9-9.json.tmp".to_string())
    );
}

#[test]
fn スタックを削除するとそのスタックの書きかけも消す() {
    let 一時 = 一時のライブラリ::作る("削除で書きかけを消す");
    書きかけを置く(&一時);
    一時
        .ライブラリ
        .削除する(&識別子("stack-1-1"))
        .expect("削除できる");
    let 一覧 = 一時.ファイル名の一覧();
    assert!(!一覧.contains(&"stack-1-1.json".to_string()), "{一覧:?}");
    assert!(
        !一覧.contains(&"stack-1-1.json.tmp".to_string()),
        "{一覧:?}"
    );
    assert!(一覧.contains(&"stack-8-8.json.tmp".to_string()), "{一覧:?}");
}
