//! `frame-time` コマンド: アプリを release でビルドして測る起動をし、重ね合わせの作業場で8行を映して再生している間の1フレームの時間
//! (GPU へ載せる時間を含む)を、アプリの中の計測(`crates/clip_viewer/src/frame_time/`)に測らせて表示する。参照: _doc/設計/同時再生.md 5-4
//! アプリのウインドウが開いて約25秒で自分で閉じる。時間がかかり結果が計算機の負荷で揺れるため、verify には入れない。

mod data_folder;

use std::ffi::OsString;
use std::process::Command;

use data_folder::測るための一時のデータのフォルダ;

use crate::decode_load::{合成画像の動画, 読む秒数};
use crate::ffmpeg_location::{FFmpegの置き場所, 場所を渡す環境変数};
use crate::verify::リポジトリルートを求める;

/// アプリに結果のファイルを渡す環境変数。参照: crates/clip_viewer/src/app/environment.rs の同じ名前の定数と一致させる(xtask はアプリのクレートに依存しないため、機械では確かめていない)。
const 結果を渡す環境変数: &str = "CLIPVIEWER_FRAME_TIME_RESULT";

/// 動画を渡されなかったときに作る合成画像の動画の長さ(秒)。アプリは5秒の区間を8つ並べるため、40秒以上が要る。
const 合成画像の秒数: u32 = 40;

/// 引数(省けば合成画像を作る動画のパス1つ)を読み、アプリに測らせて結果を表示する。
pub fn 八行を映したときの一フレームの時間を測る(
    引数一覧: &[String],
) -> Result<(), String> {
    let 渡された動画 = match 引数一覧 {
        [] => None,
        [パス] => Some(OsString::from(パス)),
        _ => return Err("frame-time が受け取る引数は動画のパス1つまで".to_string()),
    };
    let ffmpegの置き場所 = FFmpegの置き場所::探す().ok_or_else(|| {
        format!("ffmpeg と ffprobe が同じフォルダにそろう場所が、環境変数 {場所を渡す環境変数} にも PATH にも無い")
    })?;
    let 合成画像 = match 渡された動画 {
        Some(_) => None,
        None => {
            let 長さ = 読む秒数::作成する(合成画像の秒数).ok_or("合成画像の秒数が範囲の外")?;
            Some(合成画像の動画::作る(&ffmpegの置き場所, 長さ)?)
        }
    };
    let 動画 = 渡された動画
        .or_else(|| {
            合成画像
                .as_ref()
                .map(|作った| 作った.引数の表記().to_os_string())
        })
        .ok_or("測る動画が無い")?;
    let 一時 = 測るための一時のデータのフォルダ::作る()?;
    println!("ffmpeg: {}", ffmpegの置き場所.ffmpegの版を調べる());
    println!("アプリを release でビルドして起動する。ウインドウは約25秒で自分で閉じる");
    let 終了状態 = Command::new("cargo")
        .args(["run", "--release", "--package", "clip_viewer", "--"])
        .arg(&動画)
        .current_dir(リポジトリルートを求める())
        .env(場所を渡す環境変数, ffmpegの置き場所.フォルダ())
        .env("APPDATA", 一時.アプリのデータのフォルダ())
        .env("LOCALAPPDATA", 一時.ローカルのアプリのデータのフォルダ())
        .env(結果を渡す環境変数, 一時.結果のファイル())
        .status()
        .map_err(|原因| format!("cargo の起動に失敗した: {原因}"))?;
    let 結果 = std::fs::read_to_string(一時.結果のファイル()).map_err(|原因| {
        format!("アプリが結果を書かなかった(アプリの終了状態 {終了状態}): {原因}")
    })?;
    println!("{結果}");
    if 結果.starts_with("測れなかった") {
        return Err("測れなかった".to_string());
    }
    Ok(())
}
