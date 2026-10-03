//! 起動時の環境。環境変数と起動の引数を読む境界はここ1箇所であり、読んだ値を配線へ渡す。

use std::ffi::OsString;
use std::path::PathBuf;

use video_source::実行ファイルの検索パス;

use crate::frame_time::フレームの時間を測る頼み;

/// FFmpeg を置いたフォルダを渡す環境変数(verify と video_source の結合試験と同じ名前)。
const FFMPEGの環境変数: &str = "CLIPVIEWER_FFMPEG_DIR";

/// アプリのデータを置くフォルダを指す環境変数(Windows の %APPDATA%)。
const アプリのデータの環境変数: &str = "APPDATA";

/// `cargo xtask frame-time` が、1フレームの時間のまとめを書く結果のファイルを渡す環境変数。普段の起動では無い。
const フレームの時間の結果の環境変数: &str = "CLIPVIEWER_FRAME_TIME_RESULT";

/// この計算機だけのアプリのデータ(捨ててよいキャッシュ)を置くフォルダを指す環境変数(Windows の %LOCALAPPDATA%)。
const ローカルのアプリのデータの環境変数: &str = "LOCALAPPDATA";

/// 起動時の環境とは、起動したときに環境変数から読んだ、アプリのデータのフォルダとローカルのアプリのデータのフォルダと FFmpeg のフォルダと PATH と、
/// 起動の引数(実行ファイルの名前を除いたもの。エクスプローラーが渡す動画のパス)と、フレームの時間を測る頼み(`cargo xtask frame-time` が起動したときだけある)の組のことである。
#[derive(Debug, Clone)]
pub(crate) struct 起動時の環境 {
    pub(crate) アプリのデータのフォルダ: Option<PathBuf>,
    pub(crate) ローカルのアプリのデータのフォルダ: Option<PathBuf>,
    pub(crate) ffmpegのフォルダ: Option<PathBuf>,
    pub(crate) 検索パス: 実行ファイルの検索パス,
    pub(crate) 起動の引数: Vec<OsString>,
    pub(crate) フレームの時間を測る頼み: Option<フレームの時間を測る頼み>,
}

impl 起動時の環境 {
    /// 今のプロセスの環境変数と起動の引数から読む。
    pub(crate) fn 今のプロセスから読む() -> Self {
        Self {
            アプリのデータのフォルダ: std::env::var_os(アプリのデータの環境変数)
                .map(PathBuf::from),
            ローカルのアプリのデータのフォルダ: std::env::var_os(
                ローカルのアプリのデータの環境変数,
            )
            .map(PathBuf::from),
            ffmpegのフォルダ: std::env::var_os(FFMPEGの環境変数).map(PathBuf::from),
            検索パス: 実行ファイルの検索パス::環境変数から読む(),
            起動の引数: std::env::args_os().skip(1).collect(),
            フレームの時間を測る頼み: std::env::var_os(フレームの時間の結果の環境変数)
                .map(|値| {
                    フレームの時間を測る頼み::結果のファイルから作る(
                        PathBuf::from(値),
                    )
                }),
        }
    }
}
