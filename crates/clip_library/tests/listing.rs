//! 一覧の走査の試験。読めないファイルを理由付きで出し、スタックのファイルでないものを含めず、動画の有無を確かめる。
#![allow(clippy::expect_used)]

mod support;

use clip_domain::入力された動画パス;
use clip_library::{一覧の項目, 動画の有無, 読めないファイル};
use support::{スタックを作る, 一時のライブラリ};

fn 読めない理由(項目: &[一覧の項目], ファイル名: &str) -> String {
    項目
        .iter()
        .find_map(|項目| match 項目 {
            一覧の項目::読めない(読めないファイル {
                ファイル名: 名前,
                理由,
            }) if 名前.文字列() == ファイル名 => Some(理由.clone()),
            _ => None,
        })
        .unwrap_or_else(|| panic!("{ファイル名} が読めない項目に無い"))
}

#[test]
fn フォルダがまだ無ければ一覧は空である() {
    let 一時 = 一時のライブラリ::作る("無いフォルダ");
    assert!(
        一時
            .ライブラリ
            .一覧を作る()
            .expect("作れる")
            .項目
            .is_empty()
    );
}

#[test]
fn 読めないファイルを理由付きで出し_書きかけとほかの拡張子は含めない() {
    let 一時 = 一時のライブラリ::作る("読めない");
    let 動画 = 一時.動画を置く("a.mp4");
    一時
        .ライブラリ
        .保存する(&スタックを作る("stack-1-1", "甲", 動画.clone()))
        .expect("保存");
    一時
        .ライブラリ
        .保存する(&スタックを作る("stack-1-2", "乙", 動画))
        .expect("保存");
    let フォルダ = 一時.ライブラリ.フォルダ().パス().to_path_buf();
    std::fs::write(フォルダ.join("stack-9-9.json"), "{壊れた").expect("書ける");
    std::fs::copy(
        フォルダ.join("stack-1-1.json"),
        フォルダ.join("stack-7-7.json"),
    )
    .expect("写せる");
    std::fs::write(フォルダ.join("Bad Name.json"), "{}").expect("書ける");
    std::fs::write(フォルダ.join("stack-1-1.json.tmp"), "{途中").expect("書ける");
    std::fs::write(フォルダ.join("メモ.txt"), "x").expect("書ける");
    let 項目 = 一時.ライブラリ.一覧を作る().expect("作れる").項目;
    assert_eq!(項目.len(), 5, "{項目:#?}");
    let 読めた数 = 項目
        .iter()
        .filter(|項目| matches!(項目, 一覧の項目::読めた(_)))
        .count();
    assert_eq!(読めた数, 2);
    assert!(読めない理由(&項目, "stack-9-9.json").contains("JSON"));
    assert!(読めない理由(&項目, "stack-7-7.json").contains("stack-1-1"));
    assert!(読めない理由(&項目, "Bad Name.json").contains("識別子"));
}

#[test]
fn 動画のファイルが無いスタックに見つからない印を付ける() {
    let 一時 = 一時のライブラリ::作る("動画の有無");
    let ある = 一時.動画を置く("a.mp4");
    let 無い = 入力された動画パス::作成する(
        一時.一時フォルダ.join("消した.mp4").display().to_string(),
    )
    .正規化する();
    一時
        .ライブラリ
        .保存する(&スタックを作る("stack-1-1", "甲", ある))
        .expect("保存");
    一時
        .ライブラリ
        .保存する(&スタックを作る("stack-1-2", "乙", 無い))
        .expect("保存");
    let 項目 = 一時.ライブラリ.一覧を作る().expect("作れる").項目;
    let 有無 = |名前: &str| {
        項目
            .iter()
            .find_map(|項目| match 項目 {
                一覧の項目::読めた(読めた) if 読めた.スタック.名前().文字列() == 名前 => {
                    Some(読めた.動画)
                }
                _ => None,
            })
            .expect("読めた")
    };
    assert_eq!(有無("甲"), 動画の有無::ある);
    assert_eq!(有無("乙"), 動画の有無::見つからない);
}
