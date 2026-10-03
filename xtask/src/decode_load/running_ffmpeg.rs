//! 読み出し中の ffmpeg のプロセス。値を捨てるときに、終わっていなければ止めて終わりを待つ。

use std::ffi::OsString;
use std::io::Read;
use std::process::{Child, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};

use super::byte_count::バイト数;
use super::stream_failure::一本の読み出しの失敗;
use crate::ffmpeg_location::FFmpegの置き場所;

/// 標準出力を読む1回の大きさ。1コマ(1280×720 で約3.7MB)より小さくてよく、読む回数を抑える大きさにする。
const 読む塊のバイト数: usize = 1 << 20;

/// 読み出し中のffmpegとは、起動して標準出力を読んでいる1本の ffmpeg のプロセスのことである。
/// 終わりを待つ前に値を捨てると、プロセスを止めて(kill)終わりを待つ(wait)。起動に失敗した後の残りや、ほかの1本の失敗で
/// 止めたものが、走り続けたりゾンビとして残ったりしないためである。
pub struct 読み出し中のffmpeg {
    プロセス: Child,
    終わりを待ったか: bool,
}

impl 読み出し中のffmpeg {
    /// ffmpeg を起動し、標準出力を読めるようにする。
    pub fn ffmpegを起動する(
        ffmpegの置き場所: &FFmpegの置き場所,
        引数: &[OsString],
    ) -> Result<Self, String> {
        let プロセス = ffmpegの置き場所
            .ffmpegの命令()
            .args(引数)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(|原因| format!("ffmpeg の起動に失敗した: {原因}"))?;
        Ok(Self {
            プロセス,
            終わりを待ったか: false,
        })
    }

    /// 標準出力を終わりまで読んで捨て、読んだバイト数を返す。止める旗が立ったら読むのをやめて止める。
    /// 自分が失敗したときは止める旗を立て、同時に読んでいるほかの ffmpeg を止めさせる。
    pub fn 標準出力を終わりまで読む(
        mut self,
        止める旗: &AtomicBool,
    ) -> Result<バイト数, 一本の読み出しの失敗> {
        let 結果 = self.読んで終わりを待つ(止める旗);
        if let Err(一本の読み出しの失敗::失敗した(_)) = &結果 {
            止める旗.store(true, Ordering::Relaxed);
        }
        結果
    }

    fn 読んで終わりを待つ(
        &mut self,
        止める旗: &AtomicBool,
    ) -> Result<バイト数, 一本の読み出しの失敗> {
        let 失敗した = |理由: String| 一本の読み出しの失敗::失敗した(理由);
        let mut 読んだバイト数 = バイト数::ゼロ;
        let mut 標準出力 = self
            .プロセス
            .stdout
            .take()
            .ok_or_else(|| 失敗した("標準出力を受け取れない".into()))?;
        let mut 塊 = vec![0_u8; 読む塊のバイト数];
        loop {
            if 止める旗.load(Ordering::Relaxed) {
                return Err(一本の読み出しの失敗::ほかの失敗を受けて止めた);
            }
            let 読んだ = 標準出力
                .read(&mut 塊)
                .map_err(|原因| 失敗した(format!("ffmpeg の出力を読めない: {原因}")))?;
            if 読んだ == 0 {
                break;
            }
            読んだバイト数 = 読んだバイト数.読んだ分を足す(読んだ);
        }
        let 終了状態 = self
            .プロセス
            .wait()
            .map_err(|原因| 失敗した(format!("ffmpeg の終了を待てない: {原因}")))?;
        self.終わりを待ったか = true;
        if 終了状態.success() {
            Ok(読んだバイト数)
        } else {
            Err(失敗した(format!("ffmpeg が失敗した ({終了状態})")))
        }
    }
}

impl Drop for 読み出し中のffmpeg {
    fn drop(&mut self) {
        if !self.終わりを待ったか {
            // kill の失敗は、プロセスが既に終わっていたときにも起きるため捨てる。wait は終わりを回収するために必ず呼ぶ。
            let _ = self.プロセス.kill();
            let _ = self.プロセス.wait();
        }
    }
}
