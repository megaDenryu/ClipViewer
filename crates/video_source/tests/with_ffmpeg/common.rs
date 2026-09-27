//! 結合試験の共通の道具。FFmpeg を探すことと、時刻と区間を作ることと、条件を待つこと。

use std::path::PathBuf;
use std::time::{Duration, Instant};

use clip_domain::{動画上の区間, 動画上の秒, 時刻};
use video_source::{
    FFmpegの実行ファイル, FFmpegの置き場所の設定, FFmpegを置いたフォルダ, 動画の読み手,
    実行ファイルの検索パス,
};

/// FFmpeg を置いたフォルダを渡す環境変数。無ければ PATH だけを探す。
const 場所を渡す環境変数: &str = "CLIPVIEWER_FFMPEG_DIR";

/// 待つ条件が成り立つまでの期限。デコードは数秒で終わるため、これを超えたら止まっているとみなす。
pub const 待つ期限: Duration = Duration::from_secs(30);

pub fn 実行ファイルを探す() -> FFmpegの実行ファイル {
    let 設定 = match std::env::var_os(場所を渡す環境変数) {
        Some(フォルダ) => FFmpegの置き場所の設定::設定済み(
            FFmpegを置いたフォルダ::作成する(PathBuf::from(フォルダ)),
        ),
        None => FFmpegの置き場所の設定::未設定,
    };
    FFmpegの実行ファイル::探す(&設定, &実行ファイルの検索パス::環境変数から読む())
        .expect("FFmpeg が見つからない")
}

pub fn 読み手() -> 動画の読み手 {
    動画の読み手::作成する(実行ファイルを探す())
}

pub fn 秒(値: f64) -> 動画上の秒 {
    時刻::作成する(値).expect("時刻を作れない")
}

pub fn 区間(開始: f64, 終了: f64) -> 動画上の区間 {
    動画上の区間::作成する(秒(開始), 秒(終了)).expect("区間を作れない")
}

/// 条件が成り立つまで待つ。期限を過ぎたら試験を失敗させる。
pub fn 成り立つまで待つ(説明: &str, mut 条件: impl FnMut() -> bool) {
    let 始め = Instant::now();
    while !条件() {
        assert!(始め.elapsed() < 待つ期限, "{説明}が期限内に成り立たない");
        std::thread::sleep(Duration::from_millis(10));
    }
}

/// RGBA の最初の画素の、最も強い色の成分を返す。0 が赤、1 が緑、2 が青である。
pub fn 最も強い色(画素の並び: &[u8]) -> usize {
    (0..3)
        .max_by_key(|成分| 画素の並び[*成分])
        .expect("画素が無い")
}
