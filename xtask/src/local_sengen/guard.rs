//! Cargo.lock の見張り役。差し替えた cargo を実行する前の Cargo.lock の中身を覚え、実行が終わったら書き戻す。
//! 見張り役とは、xtask が自分の実行ファイルを別のプロセスとして起こしたもののことである。本体が Ctrl+C やプロセスの強制終了で
//! 止まっても書き戻せるように、見張り役は別のプロセスグループに置き(Ctrl+C が届かない)、本体とつないだ標準入力が閉じたこと
//! (本体が終わったこと)を合図に書き戻す。本体と同じプロセスの中で書き戻すと、Ctrl+C で本体ごと止まったときに戻せない。

use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;
use std::process::{Child, ChildStdin, Command, Stdio};

/// 見張り役を起こすときに渡す xtask のコマンド名。command.rs の語彙と一致させる。
pub const 見張り役のコマンド名: &str = "local-sengen-guard";

/// 見張り役が Cargo.lock の中身を覚えたことを本体へ知らせる行。本体はこの行を読んでから cargo を起こす。
const 覚えた知らせ: &str = "Cargo.lock を覚えた";

/// 依存の固定ファイルの見張りとは、Cargo.lock を書き戻させるために本体の側が持つ、起こした見張り役とその標準入力(閉じると書き戻しの合図になる)の組のことである。
pub struct 依存の固定ファイルの見張り {
    見張り役: Child,
    合図の口: ChildStdin,
}

impl 依存の固定ファイルの見張り {
    /// 見張り役を起こし、見張り役が Cargo.lock の中身を覚え終えるまで待つ。
    pub fn 起こす() -> Result<Self, String> {
        let 実行ファイル = std::env::current_exe()
            .map_err(|原因| format!("xtask の実行ファイルの場所が分からない: {原因}"))?;
        let mut 命令 = Command::new(実行ファイル);
        命令
            .arg(見張り役のコマンド名)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped());
        別のプロセスグループに置く(&mut 命令);
        let mut 見張り役 = 命令
            .spawn()
            .map_err(|原因| format!("Cargo.lock の見張り役を起こせない: {原因}"))?;
        let 見張り役の標準入出力 = (見張り役.stdin.take(), 見張り役.stdout.take());
        let (Some(合図の口), Some(知らせの口)) = 見張り役の標準入出力 else {
            return Err("見張り役の標準入出力をつなげない".to_string());
        };
        let mut 知らせ = String::new();
        let 読めた = BufReader::new(知らせの口).read_line(&mut 知らせ);
        if 読めた.is_err() || 知らせ.trim_end() != 覚えた知らせ {
            return Err(format!(
                "見張り役が Cargo.lock を覚えられなかった(知らせ: {知らせ:?})"
            ));
        }
        Ok(Self {
            見張り役, 合図の口
        })
    }

    /// 合図の口を閉じて見張り役に書き戻させ、書き戻しが終わるまで待つ。
    pub fn 書き戻させる(self) -> Result<(), String> {
        let Self {
            mut 見張り役,
            合図の口,
        } = self;
        drop(合図の口);
        let 終了状態 = 見張り役
            .wait()
            .map_err(|原因| format!("見張り役を待てない: {原因}"))?;
        if 終了状態.success() {
            Ok(())
        } else {
            Err(format!(
                "見張り役が Cargo.lock を戻せなかった ({終了状態})。git checkout -- Cargo.lock で戻す"
            ))
        }
    }
}

/// 見張り役のプロセスの中身。Cargo.lock を覚えて本体へ知らせ、標準入力が閉じるまで待ち、中身が変わっていれば書き戻す。
pub fn 見張り役として待って書き戻す(
    リポジトリルート: &Path
) -> Result<(), String> {
    let パス = リポジトリルート.join("Cargo.lock");
    let 前の中身 =
        std::fs::read(&パス).map_err(|原因| format!("{} を読めない: {原因}", パス.display()))?;
    let mut 知らせの口 = std::io::stdout();
    writeln!(知らせの口, "{覚えた知らせ}")
        .and_then(|()| 知らせの口.flush())
        .map_err(|原因| format!("本体へ知らせられない: {原因}"))?;
    // 読み取りの失敗も本体が居なくなったことを示すため、成否を問わず書き戻しへ進む。
    let _本体が閉じた = std::io::stdin().read_to_end(&mut Vec::new());
    if std::fs::read(&パス).ok().as_ref() != Some(&前の中身) {
        std::fs::write(&パス, &前の中身)
            .map_err(|原因| format!("{} を戻せない: {原因}", パス.display()))?;
        eprintln!("Cargo.lock を実行の前の中身に戻した");
    }
    Ok(())
}

#[cfg(windows)]
fn 別のプロセスグループに置く(命令: &mut Command) {
    use std::os::windows::process::CommandExt;
    /// Windows の CREATE_NEW_PROCESS_GROUP。このプロセスグループには Ctrl+C が届かない。
    const 新しいプロセスグループを作る: u32 = 0x0000_0200;
    命令.creation_flags(新しいプロセスグループを作る);
}

#[cfg(unix)]
fn 別のプロセスグループに置く(命令: &mut Command) {
    use std::os::unix::process::CommandExt;
    命令.process_group(0);
}
