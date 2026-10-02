//! `verify` コマンド: 書式検査 → 重ね合わせの作業場の層の依存の向きの検査 → lint → テスト → FFmpeg の結合試験 → 音声出力装置の確認の検証列を順に実行する。

use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::audio_device::{self, 装置の確認の結果};
use crate::ffmpeg_tests::{self, 結合試験の結果};
use crate::overlay_deps;

/// 検証列を実行する。途中の工程が失敗したら、そこで止めて失敗を返す。
pub fn 検証列を実行する() -> Result<(), String> {
    let リポジトリルート = リポジトリルートを求める();
    工程を実行する(&["fmt", "--check"], &リポジトリルート, None)?;
    overlay_deps::依存の向きを検査する(&リポジトリルート)?;
    let 工程一覧: [&[&str]; 2] = [
        &[
            "clippy",
            "--workspace",
            "--all-targets",
            "--",
            "-D",
            "warnings",
        ],
        &["test", "--workspace"],
    ];
    for 引数 in 工程一覧 {
        工程を実行する(引数, &リポジトリルート, None)?;
    }
    let mut 除いた工程 = Vec::new();
    if let 結合試験の結果::流さなかった =
        ffmpeg_tests::結合試験を実行する(&リポジトリルート)?
    {
        除いた工程.push("FFmpeg の結合試験(FFmpeg が見つからない)");
    }
    if let 装置の確認の結果::実行しなかった =
        audio_device::音声出力装置を確かめる(&リポジトリルート)?
    {
        除いた工程.push("音声出力装置の確認(音声出力装置が無い)");
    }
    if 除いた工程.is_empty() {
        println!("検証列はすべて通過した");
    } else {
        println!(
            "検証列は {} を除いて通過した(除いた工程は未検証である)",
            除いた工程.join("と")
        );
    }
    Ok(())
}

/// cargo を1回実行する。環境変数を1つ渡せる。
pub fn 工程を実行する(
    引数: &[&str],
    作業ディレクトリ: &Path,
    環境変数: Option<(&str, OsString)>,
) -> Result<(), String> {
    println!("> cargo {}", 引数.join(" "));
    let mut 命令 = Command::new("cargo");
    命令.args(引数).current_dir(作業ディレクトリ);
    if let Some((名前, 値)) = 環境変数 {
        命令.env(名前, 値);
    }
    let 終了状態 = 命令
        .status()
        .map_err(|原因| format!("cargo の起動に失敗した: {原因}"))?;
    if 終了状態.success() {
        Ok(())
    } else {
        Err(format!("cargo {} が失敗した ({終了状態})", 引数.join(" ")))
    }
}

/// 実行時の作業ディレクトリに依存させないため、xtask自身のマニフェストの位置を起点にする。
/// `xtask/` はリポジトリ直下の子ディレクトリであるため、1階層だけ上がる。`..` を含まない形で返すのは、
/// installer がこのパスを rustc のパスの置き換えの指定に使い、rustc が見るパスと前方一致させるためである。
pub fn リポジトリルートを求める() -> PathBuf {
    let xtaskのフォルダ = Path::new(env!("CARGO_MANIFEST_DIR"));
    xtaskのフォルダ
        .parent()
        .map_or_else(|| xtaskのフォルダ.join(".."), Path::to_path_buf)
}
