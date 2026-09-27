//! 落ちたときの記録係。panic の文面・場所・スレッドの名・呼び出しの履歴を `crash-<日時>.log` へ書き、OS のメッセージボックスで利用者へ知らせる。
//! 裏のスレッドの panic も同じ口を通る(panic の処理はプロセスに1つであり、どのスレッドの panic でも呼ばれる)。

use std::backtrace::Backtrace;
use std::panic::PanicHookInfo;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};

use super::timestamp::記録の日時;
use super::記録を置くフォルダ;
use crate::startup_notice::画面の外で見せる知らせ;

/// 落ちたときの記録係とは、panic を受けて記録を書き利用者へ知らせる係のことである。メッセージボックスは1回の起動で1回だけ出す。
pub(crate) struct 落ちたときの記録係 {
    フォルダ: Option<記録を置くフォルダ>,
    知らせたか: AtomicBool,
}

impl 落ちたときの記録係 {
    pub(crate) fn 作成する(フォルダ: Option<記録を置くフォルダ>) -> Self {
        Self {
            フォルダ,
            知らせたか: AtomicBool::new(false),
        }
    }

    /// プロセスの panic の処理として取り付ける。元の処理(標準エラーへの表示)も続けて呼ぶ。
    pub(crate) fn 取り付ける(self) {
        let 元の処理 = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |情報| {
            元の処理(情報);
            self.落ちたことを残して知らせる(情報);
        }));
    }

    fn 落ちたことを残して知らせる(&self, 情報: &PanicHookInfo<'_>) {
        let 日時 = 記録の日時::今();
        let 文面 = 記録の文面(情報, &日時);
        let 残した先 = self.記録を書く(&文面, &日時);
        if self.知らせたか.swap(true, Ordering::SeqCst) {
            return;
        }
        let 案内 = match 残した先 {
            Ok(ファイル) => format!("記録を次のファイルに書いた:\n{}", ファイル.display()),
            Err(理由) => format!("記録を書けなかった: {理由}"),
        };
        let 知らせ = 画面の外で見せる知らせ::作成する(format!(
            "ClipViewer で想定しない失敗が起きた。続けて使えない場合は起動し直す。\n\n{案内}\n\n{}",
            情報
        ));
        // 注意: メッセージボックスは別のスレッドで出して終わるまで待つ。落ちたスレッドが窓のスレッドのとき、
        // そのスレッドでメッセージボックスの受け取りの繰り返しを回すと、窓の処理が panic の途中で呼び戻されるためである。
        let 見せる仕事 = std::thread::spawn(move || 知らせ.利用者へ見せる());
        let _見せ終えた = 見せる仕事.join();
    }

    fn 記録を書く(&self, 文面: &str, 日時: &記録の日時) -> Result<PathBuf, String> {
        let フォルダ = self
            .フォルダ
            .as_ref()
            .ok_or("環境変数 LOCALAPPDATA が無い")?;
        let ファイル = フォルダ.落ちたときの記録のファイル(日時);
        フォルダ
            .用意する()
            .and_then(|()| std::fs::write(&ファイル, 文面))
            .map_err(|原因| format!("{}: {原因}", ファイル.display()))?;
        Ok(ファイル)
    }
}

/// 記録に書く文面。版・スレッドの名・panic の文面と場所・呼び出しの履歴を並べる。
fn 記録の文面(情報: &PanicHookInfo<'_>, 日時: &記録の日時) -> String {
    let スレッド = std::thread::current();
    format!(
        "ClipViewer {}\n日時: {}\nスレッド: {}\n{情報}\n\n呼び出しの履歴:\n{}\n",
        env!("CARGO_PKG_VERSION"),
        日時.本文に書く形(),
        スレッド.name().unwrap_or("(名の無いスレッド)"),
        Backtrace::force_capture()
    )
}
