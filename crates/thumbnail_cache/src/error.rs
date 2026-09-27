//! キャッシュの読み書きのエラー。

use std::io;
use std::path::Path;

/// キャッシュの読み書きエラーとは、サムネイルのキャッシュのファイルかフォルダを読み書きできなかった理由のことである。
#[derive(Debug, thiserror::Error)]
#[error("サムネイルのキャッシュ {パス} を読み書きできない: {原因}")]
pub struct キャッシュの読み書きエラー {
    /// 読み書きしようとしたパス(表示用)。
    pub パス: String,
    /// OS が返した理由。
    #[source]
    pub 原因: io::Error,
}

impl キャッシュの読み書きエラー {
    /// io のエラーを、どのパスで起きたかと組にする。
    pub(crate) fn 作る(パス: &Path, 原因: io::Error) -> Self {
        Self {
            パス: パス.display().to_string(),
            原因,
        }
    }
}
