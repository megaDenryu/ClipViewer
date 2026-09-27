//! 動画として開ける拡張子の一覧が、インストーラーの関連付けの一覧と同じであることの試験。
#![allow(clippy::expect_used)]

use super::video_extension::動画として開ける拡張子;

/// インストーラーの関連付けの定義。試験の時にだけ読む。
const 関連付けの定義: &str = include_str!("../../../../installer/file_association.iss");

/// `#dim VideoExtensions[N] {".mp4", ...}` の行から、点を除いた拡張子を順に取り出す。
fn インストーラーが関連付ける拡張子() -> Vec<String> {
    let 行 = 関連付けの定義
        .lines()
        .find(|行| 行.starts_with("#dim VideoExtensions"))
        .expect("installer/file_association.iss に VideoExtensions の定義の行がある");
    let 始め = 行.find('{').expect("定義の行に { がある");
    let 終わり = 行.rfind('}').expect("定義の行に } がある");
    行[始め + 1..終わり]
        .split(',')
        .map(|項目| {
            let 引用を除いた = 項目.trim().trim_matches('"');
            引用を除いた
                .strip_prefix('.')
                .expect("拡張子は点から始まる")
                .to_owned()
        })
        .collect()
}

#[test]
fn ファイルダイアログの拡張子はインストーラーの関連付けと同じ一覧である() {
    assert_eq!(
        インストーラーが関連付ける拡張子(),
        動画として開ける拡張子.to_vec()
    );
}
