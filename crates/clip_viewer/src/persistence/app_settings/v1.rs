//! settings.json 第1版の、ファイル上の形そのままの型と、最新との両方向の変換。フィールド名はファイル形式が決めている英語の名前であり、翻訳しない。
//! 第1版は知らない項目を許し、`その他` に持ち回って書くときに戻す(ライブラリのファイルと方針が違う。理由は _doc/設計/ライブラリ.md「settings.json の形式」)。
//! 項目を足すときは、版を上げずに、値が無いときは書かない項目(`Option` と `skip_serializing_if`)として足す。古いアプリはその項目を `その他` として持ち回る。

use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use video_source::{FFmpegの置き場所の設定, FFmpegを置いたフォルダ};

use super::settings::{アプリの設定, 知らない項目};
use super::v1_viewer::第1版の見る側の設定;

/// format に入る、ClipViewer の設定のファイルであることを示す名前。
pub(super) const 形式の名前: &str = "ClipViewer.settings";

/// 第1版が自分の項目として書く名前。第1版の設定の項目と、平らに入れた見る側の設定の項目である。
/// 第0版から変換するときに、知らない項目からこの名前を除く(`v0.rs`)。項目を足したらここにも足す(試験が確かめる)。
pub(super) const 第1版が書く項目の名前: [&str; 10] = [
    "format",
    "version",
    "ffmpegFolder",
    "windowSize",
    "windowMaximized",
    "volume",
    "playbackSpeed",
    "mirrorHorizontally",
    "aspectRatio",
    "displaySize",
];

/// 第1版の version に入る版の番号。
pub(super) const 第1版の番号: u64 = 1;

#[derive(Serialize, Deserialize)]
pub(super) struct 第1版の設定 {
    format: String,
    version: u64,
    #[serde(
        rename = "ffmpegFolder",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    ffmpeg_folder: Option<PathBuf>,
    /// 注意: 知らない項目(その他)より前に置く。平らに入れた型は前から順に自分の項目を取り、その他は残りを受け取るためである。
    #[serde(flatten)]
    見る側: 第1版の見る側の設定,
    #[serde(flatten)]
    その他: Map<String, Value>,
}

impl 第1版の設定 {
    /// 最新のアプリの設定へ変換する。形式の名前と版は読み込みの前に確かめ終えているため、ここでは使わない。
    pub(super) fn 最新へ変換する(self) -> アプリの設定 {
        let Self {
            format: _,
            version: _,
            ffmpeg_folder,
            見る側,
            その他,
        } = self;
        アプリの設定 {
            ffmpegの置き場所: ffmpegの置き場所へ変換する(ffmpeg_folder),
            見る側: 見る側.見る側の設定へ変換する(),
            知らない項目: 知らない項目::作成する(その他),
        }
    }

    /// 最新のアプリの設定から作る。
    pub(super) fn 最新から作る(設定: アプリの設定) -> Self {
        let アプリの設定 {
            ffmpegの置き場所,
            見る側,
            知らない項目,
        } = 設定;
        Self {
            format: 形式の名前.to_string(),
            version: 第1版の番号,
            ffmpeg_folder: match ffmpegの置き場所 {
                FFmpegの置き場所の設定::未設定 => None,
                FFmpegの置き場所の設定::設定済み(フォルダ) => {
                    Some(フォルダ.パス().to_path_buf())
                }
            },
            見る側: 第1版の見る側の設定::見る側の設定から作る(見る側),
            その他: 知らない項目.組にする(),
        }
    }
}

/// ffmpegFolder の値を FFmpeg の置き場所の設定へ変換する。第0版と第1版で同じ表記である。
pub(super) fn ffmpegの置き場所へ変換する(
    フォルダ: Option<PathBuf>,
) -> FFmpegの置き場所の設定 {
    フォルダ.map_or(FFmpegの置き場所の設定::未設定, |フォルダ| {
        FFmpegの置き場所の設定::設定済み(FFmpegを置いたフォルダ::作成する(フォルダ))
    })
}
