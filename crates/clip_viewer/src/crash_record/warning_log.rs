//! 警告の記録係。依存のライブラリ(eframe・winit・glow・rfd 等)が `log` へ出す警告以上の知らせを `warnings.log` へ書く。
//! 起動のたびに前回の記録を `warnings.previous.log` へ移して新しく書き始め、1回の起動で書く量に上限を置く。
//! 前提: 音声のスレッド(cpal が呼ぶ関数)は `log` を呼ばない。cpal は `log` に依存せず、本リポジトリのコードも `log` を使わない。

use std::fs::File;
use std::io::Write;
use std::sync::Mutex;

use super::timestamp::記録の日時;
use super::記録を置くフォルダ;

/// 1回の起動で警告の記録へ書く量の上限(バイト)。同じ警告が毎フレーム出続けてもディスクを埋めないようにする。
const 書く量の上限: u64 = 1024 * 1024;

/// 書いている記録とは、開いた警告の記録のファイルと、今回の起動で書いた量の組のことである。
struct 書いている記録 {
    ファイル: File,
    書いた量: u64,
}

/// 警告の記録係とは、`log` の警告以上の知らせを警告の記録のファイルへ書く係のことである。
pub(crate) struct 警告の記録係(Mutex<書いている記録>);

impl 警告の記録係 {
    /// 前回の記録を移してファイルを開き、`log` の出口として取り付ける。開けなければ取り付けずに理由を返す。
    pub(crate) fn 取り付ける(
        フォルダ: &記録を置くフォルダ
    ) -> Result<(), String> {
        let (今回, 前回) = フォルダ.警告の記録のファイル();
        フォルダ.用意する().map_err(|原因| 原因.to_string())?;
        if 今回.is_file() {
            std::fs::rename(&今回, &前回).map_err(|原因| 原因.to_string())?;
        }
        let ファイル = File::create(&今回).map_err(|原因| 原因.to_string())?;
        let 係 = Box::leak(Box::new(Self(Mutex::new(書いている記録 {
            ファイル,
            書いた量: 0,
        }))));
        log::set_logger(係).map_err(|原因| 原因.to_string())?;
        log::set_max_level(log::LevelFilter::Warn);
        Ok(())
    }
}

impl log::Log for 警告の記録係 {
    fn enabled(&self, 内容: &log::Metadata<'_>) -> bool {
        内容.level() <= log::Level::Warn
    }

    fn log(&self, 記録: &log::Record<'_>) {
        if !self.enabled(記録.metadata()) {
            return;
        }
        let Ok(mut 書いている) = self.0.lock() else {
            return;
        };
        if 書いている.書いた量 >= 書く量の上限 {
            return;
        }
        let 行 = format!(
            "{} {} {}: {}\n",
            記録の日時::今().行の頭に書く形(),
            記録.level(),
            記録.target(),
            記録.args()
        );
        if 書いている.ファイル.write_all(行.as_bytes()).is_ok() {
            書いている.書いた量 += u64::try_from(行.len()).unwrap_or(u64::MAX);
        }
    }

    fn flush(&self) {
        if let Ok(mut 書いている) = self.0.lock() {
            let _書き出し = 書いている.ファイル.flush();
        }
    }
}
