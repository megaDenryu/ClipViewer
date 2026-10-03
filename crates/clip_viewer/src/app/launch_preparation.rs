//! 起動の準備。起動の手順(`launch_plan.rs`)が決めた結果であり、起動の部分(main.rs)がウインドウを作り、
//! 組み立て(`assemble.rs`)が準備からクリップビューアーを組み立てる。参照: _doc/設計/画面.md 判断13

use clip_library::錠を試したライブラリの組;

use video_source::{FFmpegの置き場所の設定, FFmpegを置いたフォルダ};

use super::environment::起動時の環境;
use super::viewer_settings_save::設定の書き方;
use crate::launch::{起動の受け口, 起動の頼み};
use crate::persistence::{FFmpegの置き場所の候補, アプリの設定の保管場所};
use crate::viewer_settings::{ウインドウの記憶, 見る側の設定};

/// 起動の準備とは、ウインドウを作って組み立てるときに使う、起動時の環境・settings.json から読んだ見る側の設定・アプリの設定の保管場所・
/// 錠を試したライブラリの組(置き場所が無ければ無い。スタックのライブラリの錠を試し、持つときだけ続けて `overlays` の錠を取りに行った結果)・
/// 開いた起動の受け口(1つ目でなければ無い)・起動の頼み・起動の途中で利用者へ知らせる文の組のことである。
pub(crate) struct 起動の準備 {
    pub(super) 環境: 起動時の環境,
    pub(super) 見る側: 見る側の設定,
    pub(super) 保管場所: アプリの設定の保管場所,
    pub(super) ライブラリ: Option<錠を試したライブラリの組>,
    pub(super) 受け口: Option<起動の受け口>,
    pub(super) 頼み: 起動の頼み,
    pub(super) 知らせ: Vec<String>,
}

impl 起動の準備 {
    /// ウインドウを作るときの大きさと最大化。settings.json が覚えていなければ最初の大きさである。
    pub(crate) fn ウインドウの記憶(&self) -> ウインドウの記憶 {
        self.見る側.ウインドウ
    }

    /// ライブラリの錠を持つ(1つ目の)アプリか。読み取り専用で起動したアプリと、錠を試せなかった
    /// (ライブラリの置き場所が無い)アプリは持たない。警告の記録は、錠を持つアプリだけが取り付ける。
    pub(crate) fn ライブラリの錠を持つか(&self) -> bool {
        self.ライブラリ
            .as_ref()
            .is_some_and(|組| 組.スタック.書けるか())
    }

    /// 見る側の設定を書くか。ライブラリの錠を持つアプリだけが書く(画面.md 判断17)。
    pub(super) fn 設定の書き方(&self) -> 設定の書き方 {
        if self.ライブラリの錠を持つか() {
            設定の書き方::書く
        } else {
            設定の書き方::書かない
        }
    }

    /// 保存した FFmpeg の置き場所と、環境変数が指す FFmpeg のフォルダから、FFmpeg の置き場所の候補を作る。
    /// 設定を読めなければ未設定として扱い、読めない理由を返す(起動は止めず、通知で知らせる)。
    pub(super) fn ffmpegの置き場所の候補を読む(
        &self,
    ) -> (FFmpegの置き場所の候補, Option<String>) {
        let (保存した置き場所, 読めない理由) = match self.保管場所.ffmpegの置き場所を読む()
        {
            Ok(置き場所) => (置き場所, None),
            Err(理由) => (FFmpegの置き場所の設定::未設定, Some(理由.to_string())),
        };
        let 候補 = FFmpegの置き場所の候補 {
            環境変数のフォルダ: self
                .環境
                .ffmpegのフォルダ
                .clone()
                .map(FFmpegを置いたフォルダ::作成する),
            保存した置き場所,
        };
        (候補, 読めない理由)
    }
}
