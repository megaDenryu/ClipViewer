//! クリップビューアーを落とすときの、錠を放す順の試験。`Drop` が呼ぶ `重ね合わせのライブラリを放す` の後は、`overlays` の錠が放れていて、
//! 状態が持つスタックのライブラリの錠はまだ持っていることと、落とし終えると両方が放れていることを確かめる。
//! 錠が放れているかは、同じ錠のファイルを別に開いて排他の錠を取れるかで確かめる。利用者の %APPDATA% を使わず、一時フォルダを置き場所にする。
#![allow(clippy::expect_used)]

use std::fs::{OpenOptions, TryLockError};
use std::path::Path;

use clip_library::{
    スタックのライブラリ, ライブラリのフォルダ, 裏で動く重ね合わせのライブラリ
};

use super::launch_requests_test_support::試験のビューアー;
use crate::state::library::ライブラリの接続;

/// フォルダの錠のファイルを別に開いて、排他の錠を取れるか(ほかの持ち手が放しているか)。
fn 錠が放れているか(フォルダ: &Path) -> bool {
    let ファイル = OpenOptions::new()
        .write(true)
        .open(フォルダ.join("ClipViewer.lock"))
        .expect("錠のファイルを開ける");
    match ファイル.try_lock() {
        Ok(()) => true,
        Err(TryLockError::WouldBlock) => false,
        Err(TryLockError::Error(原因)) => panic!("錠を試せない: {原因}"),
    }
}

#[test]
fn クリップビューアーはoverlaysの錠をスタックのライブラリの錠より先に放す() {
    let 一時 = std::env::temp_dir().join(format!("clip_viewer_錠を放す順_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&一時);
    let フォルダ = ライブラリのフォルダ::作成する(一時.join("library"));
    let 重ね合わせのフォルダ = フォルダ.重ね合わせのフォルダ();
    let 組 = スタックのライブラリ::作成する(フォルダ.clone())
        .錠を試す()
        .重ね合わせの錠も試す();
    let mut ビューアー = 試験のビューアー();
    ビューアー
        .前に出ている作業場
        .重ね合わせのライブラリを持たせる(
            裏で動く重ね合わせのライブラリ::錠を試した後で起動する(
                組.重ね合わせ,
            ),
        );
    ビューアー.状態.ライブラリ.接続 =
        ライブラリの接続::錠を試したライブラリから起動する(
            Some(組.スタック),
        );
    assert!(
        !錠が放れているか(重ね合わせのフォルダ.パス())
            && !錠が放れているか(フォルダ.パス())
    );
    ビューアー.前に出ている作業場.重ね合わせの側を放す();
    assert!(錠が放れているか(重ね合わせのフォルダ.パス()));
    assert!(!錠が放れているか(フォルダ.パス()));
    drop(ビューアー);
    assert!(錠が放れているか(フォルダ.パス()));
    let _ = std::fs::remove_dir_all(&一時);
}
