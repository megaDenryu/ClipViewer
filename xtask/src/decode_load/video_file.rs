//! 測る動画のファイル。引数で渡された動画と、一時フォルダに作った合成画像の動画を1つの型で扱う。

use std::ffi::OsStr;
use std::fmt;
use std::path::PathBuf;

use super::synthetic::合成画像の動画;

/// 測る動画のファイルとは、decode-load が ffprobe と ffmpeg に読ませる、実在する動画のファイルのことである。
/// 合成画像の動画は、この値を捨てると一時フォルダごと消える。
pub enum 測る動画のファイル {
    /// 利用者が引数で渡したファイル。
    与えられたファイル(PathBuf),
    合成画像(合成画像の動画),
}

impl 測る動画のファイル {
    /// ffprobe と ffmpeg の引数へ渡すパスの表記。生のパスへ戻すのはこの口だけである。
    pub fn 引数の表記(&self) -> &OsStr {
        match self {
            Self::与えられたファイル(パス) => パス.as_os_str(),
            Self::合成画像(動画) => 動画.引数の表記(),
        }
    }
}

impl fmt::Display for 測る動画のファイル {
    fn fmt(&self, 書き先: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(書き先, "{}", self.引数の表記().display())
    }
}
