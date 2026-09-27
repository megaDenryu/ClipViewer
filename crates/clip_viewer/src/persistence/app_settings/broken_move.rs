//! 壊れた settings.json の退避。書き直す前に、壊れたファイルを上書きしない別の名前へ移す。
//! 移し先の名前にはプロセスの番号を入れる。settings.json はライブラリの錠の外にあり、2つのアプリがほぼ同時に保存しうるため、
//! 先に移した写しを後のアプリが上書きしないようにする。

use std::path::PathBuf;
use std::time::SystemTime;

use chrono::{DateTime, Local};

use super::error::アプリの設定のエラー;
use super::file::{設定のファイル, 設定のファイル名};
use super::rewrite::{壊れた設定の移し先, 読み直した設定};

/// 移し先の名前が既にあるときに、末尾の番号を変えて試す回数の上限。上限に達したら移さずに失敗を返す。
const 移し先を試す回数: u32 = 100;

impl 設定のファイル {
    /// 移す直前にもう一度読み、まだ壊れていれば移す。1回目に読んでから今までの間に、もう一方のアプリが正しい settings.json を
    /// 書いていれば、それを移さずにそのまま使う。
    pub(super) fn まだ壊れていれば移す(
        &self,
        今: SystemTime,
    ) -> Result<読み直した設定, アプリの設定のエラー> {
        match self.読む() {
            Ok(設定) => Ok(読み直した設定::読めた(設定)),
            Err(アプリの設定のエラー::壊れている { .. }) => {
                let 移し先 = self.空いている移し先(今)?;
                std::fs::rename(self.パス(), &移し先).map_err(|原因| {
                    アプリの設定のエラー::読み書きできないときのエラーを作る(&移し先, 原因)
                })?;
                Ok(読み直した設定::壊れていたので移した(
                    壊れた設定の移し先::作成する(移し先),
                ))
            }
            Err(その他) => Err(その他),
        }
    }

    /// まだ無い移し先のパス(`settings.json.broken-<年月日>-<時分秒>-<プロセスの番号>`、あれば末尾に `-2` から順に番号を足す)。
    /// 日時はこの計算機の時間帯で書く。rename は既にあるファイルを置き換えるため、無い名前を選んでから移す。
    fn 空いている移し先(
        &self,
        今: SystemTime,
    ) -> Result<PathBuf, アプリの設定のエラー> {
        let 日時 = DateTime::<Local>::from(今).format("%Y%m%d-%H%M%S");
        let 番号無しの名前 = format!("{設定のファイル名}.broken-{日時}-{}", std::process::id());
        let 最初 = self.フォルダ.join(&番号無しの名前);
        std::iter::once(最初.clone())
            .chain(
                (2..=移し先を試す回数)
                    .map(|番号| self.フォルダ.join(format!("{番号無しの名前}-{番号}"))),
            )
            .find(|候補| !候補.exists())
            .ok_or_else(|| {
                アプリの設定のエラー::読み書きできないときのエラーを作る(
                    &最初,
                    std::io::Error::new(
                        std::io::ErrorKind::AlreadyExists,
                        "壊れたファイルの移し先の名前が空いていない",
                    ),
                )
            })
    }
}
