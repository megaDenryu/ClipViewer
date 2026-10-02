//! settings.json の試験の共有部品。試験ごとに重ならない一時フォルダの保管場所と、ファイルを置く・読む・消す部品と、見本のキーの割り当てを持つ。
#![allow(clippy::expect_used)]

use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

use sengen_egui::{キー, キーの組, 修飾キー};

use super::アプリの設定の保管場所;
use crate::viewer_settings::{キーで行う操作, キーの割り当て};

/// 試験で保存に渡す日時(2026年9月27日ごろの決まった時刻)。
pub(super) fn 試験の日時() -> SystemTime {
    SystemTime::UNIX_EPOCH + Duration::from_secs(1_790_000_000)
}

/// 保管場所のフォルダ(settings.json を置くフォルダ)の中のファイル名を並べる。
pub(super) fn フォルダの中のファイル名(パス: &Path) -> Vec<String> {
    let mut 名前 = std::fs::read_dir(パス.parent().expect("親"))
        .expect("フォルダを読める")
        .map(|項目| {
            項目
                .expect("項目")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect::<Vec<_>>();
    名前.sort();
    名前
}

/// 試験ごとに重ならない保管場所と、その settings.json のパス。前の実行の残りを消してから使う。
pub(super) fn 一時の保管場所(名前: &str) -> (アプリの設定の保管場所, PathBuf) {
    let 一時フォルダ = std::env::temp_dir().join(format!(
        "clip_viewer_設定の版_{名前}_{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&一時フォルダ);
    let 保管場所 =
        アプリの設定の保管場所::アプリのデータのフォルダから決める(
            Some(一時フォルダ.clone()),
        );
    (
        保管場所,
        一時フォルダ.join("ClipViewer").join("settings.json"),
    )
}

pub(super) fn ファイルを置く(パス: &Path, 本文: &str) {
    std::fs::create_dir_all(パス.parent().expect("親")).expect("フォルダを作れる");
    std::fs::write(パス, 本文).expect("書ける");
}

/// settings.json を置いた一時フォルダ(`<一時フォルダ>/ClipViewer/settings.json` の一時フォルダ)を消す。
pub(super) fn 後片付け(パス: &Path) {
    if let Some(一時フォルダ) = パス.parent().and_then(Path::parent) {
        let _ = std::fs::remove_dir_all(一時フォルダ);
    }
}

pub(super) fn 読み直す(パス: &Path) -> serde_json::Value {
    serde_json::from_str(&std::fs::read_to_string(パス).expect("読める")).expect("JSON")
}

/// 見本の見る側の設定のキーの割り当て。再生と停止を P に、シアターと編集の切り替えを Ctrl+Shift+T に変えてある。
pub(super) fn 見本のキーの割り当て() -> キーの割り当て {
    let コマンドとシフト = 修飾キー {
        shift: true,
        ..修飾キー::COMMAND
    };
    キーの割り当て::既定()
        .キーを変えた(
            キーで行う操作::再生と停止を切り替える,
            キーの組::単独(キー::P),
        )
        .and_then(|割り当て| {
            割り当て.キーを変えた(
                キーで行う操作::シアターと編集を切り替える,
                キーの組::生成する(コマンドとシフト, キー::T),
            )
        })
        .expect("空いたキーへは変えられる")
}
