//! 保存に失敗したときの試験。失敗した並びを捨てずに間隔を置いて頼み直す。
//! 保存を失敗させるため、スタックのファイルを排他で開いて置き換えられなくする(Windows だけで流す)。
#![cfg(windows)]
#![allow(clippy::expect_used)]

use std::time::Duration;

use super::library_check::{
    ファイルのクリップの名前, 終えるまで待って当てる
};
use super::library_failure::{保存に失敗した状態, 段階};
use super::library_support::試験のライブラリ;
use crate::state::library::保存の段階;

#[test]
fn 保存に失敗したら間隔を置いて頼み直し_書けたら保存済みになる() {
    let 試験 = 試験のライブラリ::作る("頼み直し");
    let (mut 状態, 識別子, 排他, 始め) = 保存に失敗した状態(&試験);
    assert!(matches!(段階(&状態), Some(保存の段階::保存に失敗(_))));
    drop(排他);
    assert_eq!(
        ファイルのクリップの名前(&試験.ファイルを読む(&識別子).expect("ある")),
        ["甲"],
        "失敗した保存はファイルを変えない"
    );
    状態.ライブラリの書き込みを進める(始め + Duration::from_millis(700));
    assert_eq!(
        状態.ライブラリ.次に並びを見るまでの時間,
        Some(Duration::from_millis(3_000)),
        "失敗を受けて最初に見たときに頼み直す時刻を決め、それまで描き直しを待てる"
    );
    状態.ライブラリの書き込みを進める(始め + Duration::from_millis(1_000));
    assert!(
        matches!(段階(&状態), Some(保存の段階::保存に失敗(_))),
        "間隔が過ぎるまでは頼み直さない"
    );
    assert_eq!(
        状態.ライブラリ.次に並びを見るまでの時間,
        Some(Duration::from_millis(2_700))
    );
    状態.ライブラリの書き込みを進める(始め + Duration::from_millis(3_800));
    終えるまで待って当てる(&mut 状態);
    assert!(matches!(段階(&状態), Some(保存の段階::保存済み)));
    状態.ライブラリの書き込みを進める(始め + Duration::from_millis(3_900));
    assert_eq!(
        状態.ライブラリ.次に並びを見るまでの時間, None,
        "書けたら描き直しを予約しない"
    );
    assert_eq!(
        ファイルのクリップの名前(&試験.ファイルを読む(&識別子).expect("ある")),
        ["変えた"]
    );
}
