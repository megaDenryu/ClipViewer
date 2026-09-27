//! 受け渡しの本文。送り手が1行の JSON で書き、受け口が読む形を定める。受け口は読み終えたら「受け取った」の1行を返す。

use clip_domain::入力された動画パス;
use serde::{Deserialize, Serialize};

use super::guide::合言葉;
use super::request::{起動の頼み, 開かなかった動画の数};

/// 受け取ったときに受け口が返す1行。
pub(super) const 受け取った返事: &str = "received";

/// 1行の本文の長さの上限(バイト)。上限を超えて送り続ける相手に受け口のスレッドを使い続けさせないためである。
pub(super) const 本文の長さの上限: u64 = 64 * 1024;

/// 受け渡す頼みとは、起動の頼みを JSON へ写した形のことである。
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind")]
enum 受け渡す頼み {
    #[serde(rename = "focus")]
    前に出る,
    #[serde(rename = "open")]
    動画を開く {
        #[serde(rename = "path")]
        パス: String,
        #[serde(rename = "skipped")]
        開かなかった数: usize,
    },
}

/// 受け渡す本文とは、合言葉と受け渡す頼みの組を1行の JSON にしたもののことである。
#[derive(Debug, Serialize, Deserialize)]
pub(super) struct 受け渡す本文 {
    #[serde(rename = "passphrase")]
    合言葉: 合言葉,
    #[serde(rename = "request")]
    頼み: 受け渡す頼み,
}

impl 受け渡す本文 {
    pub(super) fn 作成する(合言葉: 合言葉, 頼み: &起動の頼み) -> Self {
        let 頼み = match 頼み {
            起動の頼み::前に出る => 受け渡す頼み::前に出る,
            起動の頼み::動画を開く {
                動画,
                開かなかった数,
            } => 受け渡す頼み::動画を開く {
                パス: 動画.文字列().to_string(),
                開かなかった数: 開かなかった数.数(),
            },
        };
        Self { 合言葉, 頼み }
    }

    /// 合言葉が合えば起動の頼みを返す。合わなければ無い。
    pub(super) fn 合言葉を確かめて取り出す(
        self,
        期待する: &合言葉,
    ) -> Option<起動の頼み> {
        if &self.合言葉 != 期待する {
            return None;
        }
        Some(match self.頼み {
            受け渡す頼み::前に出る => 起動の頼み::前に出る,
            受け渡す頼み::動画を開く {
                パス,
                開かなかった数,
            } => 起動の頼み::動画を開く {
                動画: 入力された動画パス::作成する(パス),
                開かなかった数: 開かなかった動画の数::作成する(開かなかった数),
            },
        })
    }
}
