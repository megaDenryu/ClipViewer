//! 受け口の案内。1つ目のアプリが待っている受け口のポートと合言葉を、アプリのデータのフォルダのファイルへ書き、2つ目のアプリが読む。
//! 合言葉は、この計算機の別の利用者やほかのプログラムが受け口へ頼みを送れないようにするためのものである
//! (案内のファイルは利用者ごとのフォルダ %APPDATA% にあり、ほかの利用者は読めない)。

use std::hash::{BuildHasher, Hasher, RandomState};
use std::io;
#[cfg(test)]
use std::net::{Ipv4Addr, SocketAddr};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// 合言葉とは、受け口が頼みを受け付けるかを決める、推測できない文字の並びのことである。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub(super) struct 合言葉(String);

impl 合言葉 {
    /// OS の乱数で乱数の種を決めた標準ライブラリのハッシュの出力を2つ並べ、128ビットの合言葉を作る。乱数を読む境界はここ1箇所である。
    pub(super) fn 作る() -> Self {
        let 半分 = || RandomState::new().build_hasher().finish();
        Self(format!("{:016x}{:016x}", 半分(), 半分()))
    }
}

/// 受け口の案内とは、1つ目のアプリの受け口のポートと合言葉の組のことである。受け口はこの計算機の中(127.0.0.1)だけで待つ。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(super) struct 受け口の案内 {
    #[serde(rename = "port")]
    pub(super) ポート: u16,
    #[serde(rename = "passphrase")]
    pub(super) 合言葉: 合言葉,
}

#[cfg(test)]
impl 受け口の案内 {
    /// 受け口の番地(この計算機の中だけの番地とポート)。
    pub(super) fn 番地(&self) -> SocketAddr {
        SocketAddr::from((Ipv4Addr::LOCALHOST, self.ポート))
    }
}

/// 受け口の案内ファイルとは、受け口の案内を書くファイルのパスのことである。ライブラリの錠と同じアプリのデータのフォルダに置き、
/// 錠を取れたアプリだけが書く。試験では一時フォルダのパスを渡し、利用者のアプリの受け口と混ざらないようにする。
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct 受け口の案内ファイル(PathBuf);

impl 受け口の案内ファイル {
    pub(crate) fn 作成する(パス: PathBuf) -> Self {
        Self(パス)
    }

    /// 案内を書く。フォルダが無ければ作る。
    pub(super) fn 書く(&self, 案内: &受け口の案内) -> io::Result<()> {
        if let Some(フォルダ) = self.0.parent() {
            std::fs::create_dir_all(フォルダ)?;
        }
        let 本文 = serde_json::to_string(案内).map_err(io::Error::other)?;
        std::fs::write(&self.0, 本文)
    }

    /// 案内を読む。ファイルが無い・書きかけで形が不正なら、読めない理由を返す。
    #[cfg(test)]
    pub(super) fn 読む(&self) -> io::Result<受け口の案内> {
        let 本文 = std::fs::read_to_string(&self.0)?;
        serde_json::from_str(&本文).map_err(|原因| io::Error::new(io::ErrorKind::InvalidData, 原因))
    }
}
