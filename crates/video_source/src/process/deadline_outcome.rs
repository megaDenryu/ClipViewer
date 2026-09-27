//! 期限付きの子プロセスを待った結果の型。期限までに終わったときに集めた出力と、集められなかった理由。

use std::io;
use std::process::ExitStatus;

use super::stderr_tail::標準エラーの末尾;

/// 集めた出力とは、期限までに終わった子プロセスの終了状態と、標準出力の全体と、標準エラーの末尾の組のことである。
pub(crate) struct 集めた出力 {
    pub(crate) 終了状態: ExitStatus,
    pub(crate) 標準出力: Vec<u8>,
    pub(crate) 標準エラーの末尾: 標準エラーの末尾,
}

/// 期限付きの待ちの失敗とは、子プロセスの出力を集められなかった理由の区別のことである。
#[derive(Debug)]
pub(crate) enum 期限付きの待ちの失敗 {
    起動できない(io::Error),
    待てない(io::Error),
    出力を読めない(io::Error),
    時間切れ,
    取り消した,
}
