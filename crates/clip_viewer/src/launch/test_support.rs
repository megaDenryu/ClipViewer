//! 起動の受け渡しの試験の支え。案内のファイルは試験ごとの一時フォルダに置き、利用者のアプリの受け口と混ざらないようにする。

use std::path::PathBuf;

use clip_domain::入力された動画パス;

use super::request::開かなかった動画の数;
use super::{受け口の案内ファイル, 起動の頼み};

/// 試験ごとに重ならない一時フォルダと、その中の案内ファイル。
pub(super) fn 一時の案内ファイル(
    名前: &str
) -> (PathBuf, 受け口の案内ファイル) {
    let フォルダ = std::env::temp_dir().join(format!(
        "clip_viewer_起動の試験_{名前}_{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&フォルダ);
    let ファイル = 受け口の案内ファイル::作成する(フォルダ.join("launch.json"));
    (フォルダ, ファイル)
}

pub(super) fn 動画を開く頼み(パス: &str, 開かなかった数: usize) -> 起動の頼み {
    起動の頼み::動画を開く {
        動画: 入力された動画パス::作成する(パス.to_string()),
        開かなかった数: 開かなかった動画の数::作成する(開かなかった数),
    }
}
