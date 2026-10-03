//! 錠のファイル。フォルダの中の錠のファイル(`ClipViewer.lock`)を OS の排他の錠で開いて持つ。ライブラリのフォルダの錠と
//! 重ね合わせのフォルダ(`overlays`)の錠が、同じ名前と同じ取り方を使う。参照: _doc/設計/ライブラリ.md 判断9、_doc/設計/同時再生.md 3-2

use std::fs::{File, OpenOptions, TryLockError};
use std::io;
use std::path::Path;

/// 錠のファイル名。拡張子が json でないため、一覧の走査の対象にならない。
const 錠のファイル名: &str = "ClipViewer.lock";

/// 錠のファイルとは、フォルダの中の錠のファイルを OS の排他の錠で開いたもののことである。
/// 落とすと錠を放す。アプリが落ちたときは OS が錠を解くため、次に起動したアプリが取れる(ファイルが残っていても取れる)。
#[derive(Debug)]
pub(crate) struct 錠のファイル {
    _開いたファイル: File,
}

/// 錠のファイルを取れない理由とは、別のアプリが錠を持っているか、錠のファイルやフォルダを作れない・開けないかの区別のことである。
#[derive(Debug, thiserror::Error)]
pub enum 錠のファイルを取れない理由 {
    /// 別のアプリ(別の ClipViewer)が錠を持っている。
    #[error("別の ClipViewer が開いている")]
    別のアプリが持っている,
    /// 錠のファイルかフォルダを作れない・開けない。
    #[error("{パス} を読み書きできない: {原因}")]
    読み書きできない {
        /// 読み書きしようとしたパス(表示用)。
        パス: String,
        /// OS が返した理由。
        原因: io::Error,
    },
}

impl 錠のファイルを取れない理由 {
    fn 読み書きのエラー(パス: &Path, 原因: io::Error) -> Self {
        Self::読み書きできない {
            パス: パス.display().to_string(),
            原因,
        }
    }
}

impl 錠のファイル {
    /// フォルダの中の錠のファイルを取る。フォルダが無ければ作る。別のアプリが持っていれば待たずに取れないと返す。
    pub(crate) fn フォルダの中で取る(
        フォルダ: &Path,
    ) -> Result<Self, 錠のファイルを取れない理由> {
        std::fs::create_dir_all(フォルダ).map_err(|原因| {
            錠のファイルを取れない理由::読み書きのエラー(フォルダ, 原因)
        })?;
        let パス = フォルダ.join(錠のファイル名);
        let ファイル = OpenOptions::new()
            .create(true)
            .truncate(false)
            .write(true)
            .open(&パス)
            .map_err(|原因| {
                錠のファイルを取れない理由::読み書きのエラー(&パス, 原因)
            })?;
        match ファイル.try_lock() {
            Ok(()) => Ok(Self {
                _開いたファイル: ファイル,
            }),
            Err(TryLockError::WouldBlock) => {
                Err(錠のファイルを取れない理由::別のアプリが持っている)
            }
            Err(TryLockError::Error(原因)) => {
                Err(錠のファイルを取れない理由::読み書きのエラー(&パス, 原因))
            }
        }
    }
}
