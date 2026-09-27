//! 期限付きの子プロセスの試験。期限までに終わった子プロセスの出力を集め、期限を過ぎたものと取り消したものは止めて失敗にする。
//! 固まる ffmpeg の代わりに、Windows に必ずある ping.exe を長く待たせて使う(止めなければ約30秒かかる)。
#![cfg(windows)]
#![allow(clippy::expect_used)]

use std::process::Command;
use std::time::{Duration, Instant};

use super::cancel::取り消しの合図;
use super::deadline::期限付きの子プロセス;
use super::deadline_outcome::期限付きの待ちの失敗;

/// 止めなければ約30秒かかる命令。
fn 長くかかる命令() -> Command {
    let mut 命令 = Command::new("ping");
    命令.args(["-n", "30", "127.0.0.1"]);
    命令
}

#[test]
fn 期限までに終わった子プロセスの標準出力を集める() {
    let mut 命令 = Command::new("cmd");
    命令.args(["/C", "echo thumbnail"]);
    let 出力 = 期限付きの子プロセス::起動する(命令)
        .expect("起動できる")
        .終わるまで待つ(Duration::from_secs(10), &取り消しの合図::default())
        .expect("集められる");
    assert!(出力.終了状態.success());
    assert_eq!(String::from_utf8_lossy(&出力.標準出力).trim(), "thumbnail");
}

#[test]
fn 期限を過ぎた子プロセスは止めて時間切れにする() {
    let 始め = Instant::now();
    let 結果 = 期限付きの子プロセス::起動する(長くかかる命令())
        .expect("起動できる")
        .終わるまで待つ(Duration::from_millis(300), &取り消しの合図::default());
    let かかった = 始め.elapsed();
    assert!(
        matches!(結果, Err(期限付きの待ちの失敗::時間切れ)),
        "{:?}",
        結果.err()
    );
    // 止めずに待てば約30秒かかる。読むスレッドの終わりまで含めて短く終わったことが、止めた証拠である。
    assert!(かかった < Duration::from_secs(5), "{かかった:?}");
}

#[test]
fn 取り消しの合図を受けた子プロセスはすぐ止めて取り消したにする() {
    let 合図 = 取り消しの合図::default();
    let 別の合図 = 合図.clone();
    let 取り消す係 = std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(100));
        別の合図.取り消す();
    });
    let 始め = Instant::now();
    let 結果 = 期限付きの子プロセス::起動する(長くかかる命令())
        .expect("起動できる")
        .終わるまで待つ(Duration::from_secs(60), &合図);
    取り消す係.join().expect("取り消す係が終わる");
    assert!(
        matches!(結果, Err(期限付きの待ちの失敗::取り消した)),
        "{:?}",
        結果.err()
    );
    assert!(始め.elapsed() < Duration::from_secs(5));
}
