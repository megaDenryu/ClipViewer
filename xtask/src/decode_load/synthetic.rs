//! 動画を渡されなかったときに測る、ffmpeg の合成画像の動画。一時フォルダに作り、測り終えたらフォルダごと消す。

use std::path::{Path, PathBuf};

use super::read_length::読む秒数;
use crate::ffmpeg_location::FFmpegの置き場所;

/// 合成画像の動画とは、testsrc2 を 1920×1080・30コマ/秒で H.264(libx264 の既定の設定・キーフレームの間隔60コマ・yuv420p)へ
/// 符号化した、一時フォルダの中の動画のことである。値を捨てると一時フォルダを消す。
pub struct 合成画像の動画 {
    一時フォルダ: PathBuf,
    動画: PathBuf,
}

impl 合成画像の動画 {
    /// OS の一時フォルダの下に、このプロセスだけが使うフォルダを作り、指定の秒数の動画を符号化する。
    pub fn 作る(
        ffmpegの置き場所: &FFmpegの置き場所, 長さ: 読む秒数
    ) -> Result<Self, String> {
        let 一時フォルダ =
            std::env::temp_dir().join(format!("clipviewer-decode-load-{}", std::process::id()));
        std::fs::create_dir_all(&一時フォルダ).map_err(|原因| {
            format!("一時フォルダ {} を作れない: {原因}", 一時フォルダ.display())
        })?;
        let 作ったもの = Self {
            動画: 一時フォルダ.join("testsrc2.mp4"),
            一時フォルダ,
        };
        println!(
            "合成画像の動画を作る({}秒、1920x1080、30コマ/秒): {}",
            長さ.値(),
            作ったもの.動画.display()
        );
        let 入力 = format!("testsrc2=size=1920x1080:rate=30:duration={}", 長さ.値());
        let 終了状態 = ffmpegの置き場所
            .ffmpegの命令()
            .args([
                "-hide_banner",
                "-nostdin",
                "-loglevel",
                "error",
                "-y",
                "-f",
                "lavfi",
                "-i",
            ])
            .arg(入力)
            .args(["-c:v", "libx264", "-g", "60", "-pix_fmt", "yuv420p"])
            .arg(&作ったもの.動画)
            .status()
            .map_err(|原因| format!("ffmpeg の起動に失敗した: {原因}"))?;
        if !終了状態.success() {
            return Err(format!("合成画像の動画を作れなかった ({終了状態})"));
        }
        Ok(作ったもの)
    }

    /// 作った動画のファイル。
    pub fn パス(&self) -> &Path {
        &self.動画
    }
}

impl Drop for 合成画像の動画 {
    fn drop(&mut self) {
        if let Err(原因) = std::fs::remove_dir_all(&self.一時フォルダ) {
            eprintln!(
                "一時フォルダ {} を消せなかった: {原因}",
                self.一時フォルダ.display()
            );
        }
    }
}
