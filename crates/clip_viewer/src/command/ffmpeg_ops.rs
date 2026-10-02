//! FFmpeg の置き場所の操作。FFmpeg を探して動画の開き手へ読み手を持たせ、見つからなければ入力されたフォルダで探し直して保存する。

use std::time::SystemTime;

use video_source::{
    FFmpegの実行ファイル, FFmpegの置き場所の設定, 動画の読み手
};

use super::applier::操作の適用係;
use crate::persistence::アプリの設定の保存の結果;
use crate::state::{FFmpegの状況, アプリの状態, 入力中のフォルダ};

/// FFmpegの操作とは、FFmpeg が見つからないときに画面の上部の欄が発する操作の区別のことである。
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum FFmpegの操作 {
    フォルダの入力を書き換える(String),
    フォルダを保存して探し直す,
}

impl 操作の適用係 {
    /// 置き場所の設定 → PATH の順に探す。見つかれば動画の開き手に読み手を持たせる。
    pub(crate) fn ffmpegを探す(
        &mut self, 設定: &FFmpegの置き場所の設定
    ) -> FFmpegの状況 {
        match FFmpegの実行ファイル::探す(設定, &self.検索パス) {
            Ok(実行ファイル) => {
                self.開き手
                    .読み手を持たせる(動画の読み手::作成する(実行ファイル.clone()));
                FFmpegの状況::見つかった(実行ファイル)
            }
            Err(理由) => FFmpegの状況::見つからない {
                理由,
                入力中のフォルダ: 入力中のフォルダ::default(),
            },
        }
    }

    /// 動画の開き手が持っている動画の読み手を貸す。FFmpeg が見つかっていなければ無い。重ね合わせの作業場が動画を開くときに、配線が借りて渡す。
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "第8段階で「今のスタックから並べる」が重ね合わせを開くときに配線が呼ぶ。それまでは試験からだけ呼ぶ(同時再生.md 6節)"
        )
    )]
    pub(crate) fn 動画の読み手(&self) -> Option<&動画の読み手> {
        self.開き手.読み手()
    }

    pub(super) fn ffmpegの操作を適用する(
        &mut self,
        状態: &mut アプリの状態,
        操作: FFmpegの操作,
    ) {
        let FFmpegの状況::見つからない {
            入力中のフォルダ: 入力,
            ..
        } = &mut 状態.ffmpegの状況
        else {
            return;
        };
        match 操作 {
            FFmpegの操作::フォルダの入力を書き換える(文字列) => {
                *入力 = 入力中のフォルダ::作成する(文字列)
            }
            FFmpegの操作::フォルダを保存して探し直す => {
                let 入力 = 入力.clone();
                self.入力したフォルダで探し直す(状態, 入力);
            }
        }
    }

    /// 入力したフォルダで探し直す。見つかったときだけ、そのフォルダをアプリの設定へ保存する。
    fn 入力したフォルダで探し直す(
        &mut self,
        状態: &mut アプリの状態,
        入力: 入力中のフォルダ,
    ) {
        let Some(フォルダ) = 入力.フォルダとして読む() else {
            return 状態.通知.出す("FFmpeg を置いたフォルダを入力してください");
        };
        match self.ffmpegを探す(&FFmpegの置き場所の設定::設定済み(
            フォルダ.clone(),
        )) {
            FFmpegの状況::見つかった(実行ファイル) => {
                let 知らせ = match self
                    .保管場所
                    .ffmpegの置き場所を保存する(&フォルダ, SystemTime::now())
                {
                    Ok(アプリの設定の保存の結果::書き直した) => {
                        "FFmpeg が見つかり、置き場所を保存しました".to_string()
                    }
                    Ok(アプリの設定の保存の結果::壊れたファイルを移して書いた(移し先)) => format!(
                        "FFmpeg が見つかり、置き場所を保存しました。壊れていて読めなかったアプリの設定は {移し先} へ移しました"
                    ),
                    Err(理由) => format!("FFmpeg は見つかったが、置き場所を保存できない: {理由}"),
                };
                状態.ffmpegの状況 = FFmpegの状況::見つかった(実行ファイル);
                状態.通知.出す(知らせ);
            }
            FFmpegの状況::見つからない { 理由, .. } => {
                状態.ffmpegの状況 = FFmpegの状況::見つからない {
                    理由,
                    入力中のフォルダ: 入力,
                };
                状態
                    .通知
                    .出す("入力したフォルダと PATH に ffmpeg と ffprobe がそろっていない");
            }
        }
    }
}
