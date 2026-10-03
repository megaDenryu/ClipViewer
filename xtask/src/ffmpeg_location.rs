//! ffmpeg と ffprobe の実行ファイルがそろうフォルダの探索。verify の結合試験と decode-load が同じ規則で探す。

use std::path::{Path, PathBuf};
use std::process::Command;

/// FFmpeg を置いたフォルダを渡す環境変数。video_source の試験も同じ名前を読む。
pub const 場所を渡す環境変数: &str = "CLIPVIEWER_FFMPEG_DIR";

/// FFmpegの置き場所とは、ffmpeg と ffprobe の実行ファイルが同じフォルダにそろう場所のことである。
pub struct FFmpegの置き場所(PathBuf);

impl FFmpegの置き場所 {
    /// 環境変数 CLIPVIEWER_FFMPEG_DIR → PATH の順に、ffmpeg と ffprobe が同じフォルダにそろう最初の場所を探す。
    /// 参照: 探す順と同じフォルダの規則は crates/video_source/src/locate/executables.rs の `FFmpegの実行ファイル::探す` と同じである。
    pub fn 探す() -> Option<Self> {
        let 指定 = std::env::var_os(場所を渡す環境変数).map(PathBuf::from);
        let 検索パス = std::env::var_os("PATH").unwrap_or_default();
        指定
            .into_iter()
            .chain(std::env::split_paths(&検索パス))
            .find(|フォルダ| 両方そろうか(フォルダ))
            .map(Self)
    }

    /// 見つけたフォルダ。
    pub fn フォルダ(&self) -> &Path {
        &self.0
    }

    /// このフォルダの ffmpeg を起動する命令を作る。
    pub fn ffmpegの命令(&self) -> Command {
        Command::new(実行ファイルのパス(&self.0, "ffmpeg"))
    }

    /// `ffmpeg -version` の1行目を返す。読めなければ、読めなかったことを書いた文を返す(表示に使うだけのため失敗にしない)。
    pub fn ffmpegの版を調べる(&self) -> String {
        self.ffmpegの命令()
            .arg("-version")
            .output()
            .ok()
            .and_then(|出力| {
                String::from_utf8_lossy(&出力.stdout)
                    .lines()
                    .next()
                    .map(str::to_owned)
            })
            .unwrap_or_else(|| "(ffmpeg -version を読めなかった)".to_owned())
    }

    /// このフォルダの ffprobe を起動する命令を作る。
    pub fn ffprobeの命令(&self) -> Command {
        Command::new(実行ファイルのパス(&self.0, "ffprobe"))
    }
}

fn 実行ファイルのパス(フォルダ: &Path, 名前: &str) -> PathBuf {
    フォルダ.join(format!("{名前}{}", std::env::consts::EXE_SUFFIX))
}

fn 両方そろうか(フォルダ: &Path) -> bool {
    ["ffmpeg", "ffprobe"]
        .iter()
        .all(|名前| 実行ファイルのパス(フォルダ, 名前).is_file())
}
