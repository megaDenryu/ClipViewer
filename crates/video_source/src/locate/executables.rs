//! 見つけた FFmpeg の実行ファイルの組と、それを探す手順と、起動の命令の組み立て。

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use super::error::{FFmpegが見つからないエラー, FFmpegの道具, 探した場所};
use super::place::{FFmpegの置き場所の設定, 実行ファイルの検索パス};

/// Windows でコンソールのウインドウを開かずに子プロセスを起動する指定(CREATE_NO_WINDOW)。
#[cfg(windows)]
const ウインドウを開かない指定: u32 = 0x0800_0000;

/// FFmpegの実行ファイルとは、見つけた ffmpeg と ffprobe の実行ファイルのパスの組のことである。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FFmpegの実行ファイル {
    変換のパス: PathBuf,
    調査のパス: PathBuf,
}

impl FFmpegの実行ファイル {
    /// 設定の場所 → PATH の順に、ffmpeg と ffprobe の実行ファイルが両方そろうフォルダを探す。
    /// 版の違う組を選ばないため、2つを別々のフォルダから選ばない。見つからなければ、探した場所を並べたエラーを返す。
    /// 参照: xtask/src/ffmpeg_location.rs は同じ探す順と同じフォルダの規則で探し、verify(結合試験を流すかの判定)と decode-load が共有する。
    pub fn 探す(
        設定: &FFmpegの置き場所の設定,
        検索パス: &実行ファイルの検索パス,
    ) -> Result<Self, FFmpegが見つからないエラー> {
        let 場所の列 = 探す場所を並べる(設定, 検索パス);
        let 道具 = [FFmpegの道具::変換, FFmpegの道具::調査];
        let そろう場所 = 場所の列
            .iter()
            .map(探した場所::フォルダ)
            .find(|フォルダ| 道具.iter().all(|一つ| 道具があるか(フォルダ, *一つ)));
        if let Some(フォルダ) = そろう場所 {
            let 変換のパス = フォルダ.join(FFmpegの道具::変換.ファイル名());
            let 調査のパス = フォルダ.join(FFmpegの道具::調査.ファイル名());
            return Ok(Self {
                変換のパス,
                調査のパス,
            });
        }
        let 見つからない道具 = 道具
            .into_iter()
            .filter(|一つ| {
                !場所の列
                    .iter()
                    .any(|場所| 道具があるか(場所.フォルダ(), *一つ))
            })
            .collect();
        Err(FFmpegが見つからないエラー {
            見つからない道具,
            探した場所: 場所の列,
        })
    }

    /// ffmpeg の実行ファイルのパス。
    pub fn 変換のパス(&self) -> &Path {
        &self.変換のパス
    }

    /// ffprobe の実行ファイルのパス。
    pub fn 調査のパス(&self) -> &Path {
        &self.調査のパス
    }

    /// ffmpeg を起動する命令を作る。標準入力は閉じ、Windows ではウインドウを開かない。
    pub(crate) fn 変換の命令(&self) -> Command {
        ウインドウを開かない命令(&self.変換のパス)
    }

    /// ffprobe を起動する命令を作る。標準入力は閉じ、Windows ではウインドウを開かない。
    pub(crate) fn 調査の命令(&self) -> Command {
        ウインドウを開かない命令(&self.調査のパス)
    }
}

fn ウインドウを開かない命令(実行ファイル: &Path) -> Command {
    let mut 命令 = Command::new(実行ファイル);
    命令.stdin(Stdio::null());
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        命令.creation_flags(ウインドウを開かない指定);
    }
    命令
}

fn 探す場所を並べる(
    設定: &FFmpegの置き場所の設定,
    検索パス: &実行ファイルの検索パス,
) -> Vec<探した場所> {
    let 設定の場所 = match 設定 {
        FFmpegの置き場所の設定::未設定 => None,
        FFmpegの置き場所の設定::設定済み(フォルダ) => Some(
            探した場所::設定の場所(フォルダ.パス().to_path_buf()),
        ),
    };
    let 検索パスの場所 = 検索パス
        .フォルダの列()
        .iter()
        .map(|フォルダ| 探した場所::PATHの中(フォルダ.clone()));
    設定の場所.into_iter().chain(検索パスの場所).collect()
}

/// フォルダに道具の実行ファイルがあるか。ファイルの有無を調べる境界はここ1箇所である。
fn 道具があるか(フォルダ: &Path, 道具: FFmpegの道具) -> bool {
    フォルダ.join(道具.ファイル名()).is_file()
}
