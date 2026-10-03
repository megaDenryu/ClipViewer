//! 錠のファイル。フォルダの中の錠のファイル(`ClipViewer.lock`)を OS の排他の錠で開いて持つ。ライブラリのフォルダの錠と
//! 重ね合わせのフォルダ(`overlays`)の錠が、同じ名前と同じ取り方を使う。参照: _doc/設計/ライブラリ.md 判断9、_doc/設計/同時再生.md 3-2

use std::fs::{File, OpenOptions, TryLockError};
use std::io;
use std::path::{Path, PathBuf};

/// 錠のファイル名。拡張子が json でないため、一覧の走査の対象にならない。
const 錠のファイル名: &str = "ClipViewer.lock";

/// 錠のファイルとは、フォルダの中の錠のファイルを OS の排他の錠で開いたもののことである。
/// 落とすと錠を放す。アプリが落ちたときは OS が錠を解くため、次に起動したアプリが取れる(ファイルが残っていても取れる)。
#[derive(Debug)]
pub(crate) struct 錠のファイル {
    _開いたファイル: File,
}

/// 錠のファイルを取れない理由とは、別のアプリが錠を持っているか、錠のファイルやフォルダを作れない・開けないかの区別のことである。
#[derive(Debug)]
pub(crate) enum 錠のファイルを取れない理由 {
    別のアプリが持っている,
    読み書きできない { パス: PathBuf, 原因: io::Error },
}

impl 錠のファイル {
    /// フォルダの中の錠のファイルを取る。フォルダが無ければ作る。別のアプリが持っていれば待たずに取れないと返す。
    pub(crate) fn フォルダの中で取る(
        フォルダ: &Path,
    ) -> Result<Self, 錠のファイルを取れない理由> {
        std::fs::create_dir_all(フォルダ).map_err(|原因| {
            錠のファイルを取れない理由::読み書きできない {
                パス: フォルダ.to_path_buf(),
                原因,
            }
        })?;
        let パス = フォルダ.join(錠のファイル名);
        let ファイル = match OpenOptions::new()
            .create(true)
            .truncate(false)
            .write(true)
            .open(&パス)
        {
            Ok(ファイル) => ファイル,
            Err(原因) => {
                return Err(錠のファイルを取れない理由::読み書きできない {
                    パス,
                    原因,
                });
            }
        };
        match ファイル.try_lock() {
            Ok(()) => Ok(Self {
                _開いたファイル: ファイル,
            }),
            Err(TryLockError::WouldBlock) => {
                Err(錠のファイルを取れない理由::別のアプリが持っている)
            }
            Err(TryLockError::Error(原因)) => {
                Err(錠のファイルを取れない理由::読み書きできない {
                    パス,
                    原因,
                })
            }
        }
    }
}
