//! ライブラリの操作の試験の道具。一時フォルダをライブラリとサムネイルのキャッシュの置き場所にした適用係と接続と、登録済みのスタックを開いた状態を作る。
#![allow(clippy::expect_used)]

use std::path::PathBuf;

use clip_domain::{
    クリップ, スタックの名前, スタックの識別子, ライブラリのスタック, ライブラリの日時,
    入力された動画パス,
};
use clip_library::{スタックのライブラリ, ライブラリのフォルダ};
use eframe::egui;
use video_source::実行ファイルの検索パス;

use super::クリップを並べた状態;
use crate::command::操作の適用係;
use crate::persistence::アプリの設定の保管場所;
use crate::state::library::{
    ライブラリの接続, 登録済みのスタック, 開いているスタック
};
use crate::state::アプリの状態;
use crate::thumbnail_feed::一覧のサムネイル;

/// 試験のライブラリとは、試験ごとに重ならない一時フォルダをアプリのデータのフォルダとサムネイルのキャッシュの置き場所にした適用係のことである。
/// %APPDATA% と %LOCALAPPDATA% の本物のフォルダを使わない。落とすと一時フォルダを消す。
pub(super) struct 試験のライブラリ {
    pub(super) 一時フォルダ: PathBuf,
    pub(super) フォルダ: ライブラリのフォルダ,
    pub(super) 適用係: 操作の適用係,
}

impl 試験のライブラリ {
    pub(super) fn 作る(名前: &str) -> Self {
        let 一時フォルダ = std::env::temp_dir().join(format!(
            "clip_viewer_ライブラリ_{名前}_{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&一時フォルダ);
        let 保管場所 =
            アプリの設定の保管場所::アプリのデータのフォルダから決める(
                Some(一時フォルダ.clone()),
            );
        let フォルダ = 保管場所.ライブラリのフォルダ().expect("フォルダ");
        let 適用係 = 操作の適用係::作成する(
            保管場所,
            実行ファイルの検索パス::作成する(Vec::new()),
            egui::Context::default(),
        );
        Self {
            一時フォルダ,
            フォルダ,
            適用係,
        }
    }

    /// クリップを並べ、このライブラリへ接続した状態(錠を取って書ける)。サムネイルは一時フォルダのキャッシュを使う。
    pub(super) fn 接続した状態(
        &self, クリップ一覧: Vec<クリップ>
    ) -> アプリの状態 {
        let mut 状態 = クリップを並べた状態(クリップ一覧);
        状態.ライブラリ.接続 =
            ライブラリの接続::錠を試したライブラリから起動する(Some(
                スタックのライブラリ::作成する(self.フォルダ.clone()).錠を試す(),
            ));
        状態.ライブラリ.サムネイル =
            一覧のサムネイル::作成する(egui::Context::default(), self.キャッシュ());
        状態
    }

    /// ファイルから読む。無ければ無い。
    pub(super) fn ファイルを読む(
        &self,
        識別子: &スタックの識別子,
    ) -> Option<ライブラリのスタック> {
        スタックのライブラリ::作成する(self.フォルダ.clone())
            .読む(識別子)
            .ok()
    }

    /// 今の並びを名前のスタックとしてファイルへ保存し、そのスタックを開いた登録済みの状態にする。
    pub(super) fn 登録済みにする(
        &self,
        状態: &mut アプリの状態,
        識別子: &str,
        名前: &str,
    ) -> スタックの識別子 {
        let スタック = ライブラリのスタック::新しく作る(
            スタックの識別子::文字列から作成する(識別子.to_string()).expect("識別子"),
            スタックの名前::作成する(名前.to_string()).expect("名前"),
            入力された動画パス::作成する(
                self.一時フォルダ.join("無い動画.mp4").display().to_string(),
            )
            .正規化する(),
            状態.並び.clone(),
            ライブラリの日時::紀元からのミリ秒で作成する(1_000).expect("日時"),
        );
        スタックのライブラリ::作成する(self.フォルダ.clone())
            .保存する(&スタック)
            .expect("保存できる");
        状態.ライブラリ.開いている = 開いているスタック::登録済み(
            登録済みのスタック::ファイルから開いた(スタック.clone()),
        );
        スタック.識別子().clone()
    }
}

impl Drop for 試験のライブラリ {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.一時フォルダ);
    }
}
