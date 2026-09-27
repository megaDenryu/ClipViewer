//! デコードの失敗の型。

use std::io;
use std::process::ExitStatus;

use super::stderr_tail::標準エラーの末尾;

/// デコードの失敗とは、ffmpeg でコマを読み出せなかった理由のことである。
#[derive(Debug, thiserror::Error)]
pub enum デコードの失敗 {
    /// ffmpeg を起動できなかった。
    #[error("ffmpeg を起動できない: {0}")]
    起動できない(#[source] io::Error),
    /// ffmpeg の出力を読む途中で失敗した。
    #[error("ffmpeg の出力を読めない: {原因}。{標準エラーの末尾}")]
    出力を読めない {
        /// 読み取りの失敗。
        #[source]
        原因: io::Error,
        /// ffmpeg の標準エラーの末尾。
        標準エラーの末尾: 標準エラーの末尾,
    },
    /// ffmpeg が失敗の終了状態で終わった。
    #[error("ffmpeg が異常終了した({終了状態})。{標準エラーの末尾}")]
    異常終了 {
        /// ffmpeg の終了状態。
        終了状態: ExitStatus,
        /// ffmpeg の標準エラーの末尾。
        標準エラーの末尾: 標準エラーの末尾,
    },
    /// ffmpeg が正常に終わったが、コマを1つも出さなかった。区間が動画の終わりより後ろにある等。
    #[error("ffmpeg がコマを1つも出さなかった。{標準エラーの末尾}")]
    コマが無い {
        /// ffmpeg の標準エラーの末尾。
        標準エラーの末尾: 標準エラーの末尾,
    },
}
