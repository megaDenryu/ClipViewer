//! フレームの時間を測る頼み。`cargo xtask frame-time` が環境変数で渡す結果のファイルの場所を持ち、まとめの文をそこへ書く。
//! 頼みが無い普段の起動では、計測の部品は何もしない。参照: _doc/設計/同時再生.md 5-4

use std::path::PathBuf;
use std::time::Duration;

/// 動画を開いてから測り始めるまでに待つ時間。行の流し読みを開き、倉庫が溜め始めて落ち着くまでのフレームを測らないためである。
pub(crate) const 測る前に待つ時間: Duration = Duration::from_secs(3);
/// 測る長さ。重ね合わせ(長さ5秒)を全体ループで4周する長さである。
pub(crate) const 測る長さ: Duration = Duration::from_secs(20);
/// 起動から動画が開くまで待つ上限。過ぎたら、測れなかったことを書いて閉じる。
pub(crate) const 動画を待つ上限: Duration = Duration::from_secs(60);

/// フレームの時間を測る頼みとは、測ったまとめの文を書く結果のファイルの場所のことである。
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct フレームの時間を測る頼み {
    結果のファイル: PathBuf,
}

impl フレームの時間を測る頼み {
    /// 環境変数で渡された結果のファイルの場所から作る。
    pub(crate) fn 結果のファイルから作る(結果のファイル: PathBuf) -> Self {
        Self {
            結果のファイル
        }
    }

    /// まとめの文を結果のファイルへ書く。書けなければ標準エラーへ出す(測る道具の失敗であり、アプリの振る舞いには関わらない)。
    pub(crate) fn 結果を書く(&self, 文: &str) {
        if let Err(原因) = std::fs::write(&self.結果のファイル, 文) {
            eprintln!(
                "フレームの時間の結果を {} へ書けない: {原因}\n{文}",
                self.結果のファイル.display()
            );
        }
    }
}
