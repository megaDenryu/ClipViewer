//! `verify` の最後の工程: 音声出力装置の確認(audio_output の例 device_check)を実行する。
//! 例は既定の出力装置に1本目と2本目の流れを開いて音量0で鳴らし、装置が2本とも音を求めたことと、2本目を開いても1本目の速さが減らないことを確かめる。装置が無ければ決まった終了コードで終わるので、
//! そのときは実行しなかったことを表示して、検証列そのものは失敗させない。xtask は cpal に依存しない。

use std::path::Path;
use std::process::Command;

/// 例が「装置が無いため実行しなかった」ことを示す終了コード。crates/audio_output/examples/device_check/main.rs と同じ値である。
const 装置が無いときの終了コード: i32 = 3;

/// 装置の確認の結果とは、音声出力装置の確認を実行して確かめたか、装置が無く実行しなかったかの区別のことである。
pub enum 装置の確認の結果 {
    確かめた,
    実行しなかった,
}

/// 音声出力装置の確認を実行する。装置が無ければ、実行しなかったことと理由を表示して、実行しなかったと返す。
pub fn 音声出力装置を確かめる(
    リポジトリルート: &Path,
) -> Result<装置の確認の結果, String> {
    let 引数 = [
        "run",
        "-q",
        "-p",
        "audio_output",
        "--example",
        "device_check",
    ];
    println!("> cargo {}", 引数.join(" "));
    let 終了状態 = Command::new("cargo")
        .args(引数)
        .current_dir(リポジトリルート)
        .status()
        .map_err(|原因| format!("cargo の起動に失敗した: {原因}"))?;
    match 終了状態.code() {
        Some(0) => Ok(装置の確認の結果::確かめた),
        Some(装置が無いときの終了コード) => {
            println!(
                "音声出力装置の確認を実行しなかった: 既定の音声出力装置が無い。確かめるには、音声出力装置のある環境で cargo xtask verify を実行し直す。"
            );
            Ok(装置の確認の結果::実行しなかった)
        }
        _ => Err(format!("音声出力装置の確認が失敗した ({終了状態})")),
    }
}
