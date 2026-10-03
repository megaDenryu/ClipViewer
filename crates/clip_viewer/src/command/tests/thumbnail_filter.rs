//! 絞り込みとサムネイルの描き直しの試験。絞り込みで隠れた行はキャッシュを見る仕事が回らないため、描き直しを早める判定に数えない。
#![allow(clippy::expect_used)]

use super::library_support::試験のライブラリ;
use super::library_thumbnail_list::一覧を読んだ状態;
use super::thumbnail_place::動画のあるスタックを置く;
use crate::library_common::list::絞り込みの語;

#[test]
fn 絞り込みで隠れた行はキャッシュを見る前でも描き直しを早める判定に数えない() {
    let 試験 = 試験のライブラリ::作る("絞り込みで隠れた行");
    動画のあるスタックを置く(&試験, "stack-1-1", None);
    let mut 状態 = 一覧を読んだ状態(&試験);
    assert!(状態.見せる行にキャッシュを見る前の行があるか());
    状態
        .ライブラリ
        .一覧
        .絞り込みを変える(絞り込みの語::作成する("一致しない名前".into()));
    assert!(!状態.見せる行にキャッシュを見る前の行があるか());
}
