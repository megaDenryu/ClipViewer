//! 同じ動画を n 本の ffmpeg で同時に読み切り、時間と1本が読んだコマ数を測る。

use std::ffi::OsString;
use std::io::Read;
use std::process::{Child, Stdio};
use std::time::Instant;

use super::measurement::一回の測定;
use super::one_stream::一本の読み方;
use super::stream_count::同時に読む本数;
use crate::ffmpeg_location::FFmpegの置き場所;

/// 標準出力を読む1回の大きさ。1コマ(1280×720 で約3.7MB)より小さくてよく、読む回数を抑える大きさにする。
const 読む塊のバイト数: usize = 1 << 20;

/// 同時の読み出しとは、1本の読み方と、それを n 本起動する ffmpeg の場所の組のことである。
pub struct 同時の読み出し<'a> {
    pub 場所: &'a FFmpegの置き場所,
    pub 読み方: 一本の読み方<'a>,
}

impl 同時の読み出し<'_> {
    /// n 本を同時に起動して読み切る。読み切る時間は、1本目を起動する直前から、すべてが終わるまでである。
    /// どれかが失敗するか、本ごとに読んだコマ数が違えば、理由を付けて失敗にする。
    pub fn 同時に読み切る(
        &self, 本数: 同時に読む本数
    ) -> Result<一回の測定, String> {
        let 引数 = self.読み方.引数を並べる();
        let 始め = Instant::now();
        let プロセスの並び = (0..本数.値())
            .map(|_| self.起動する(&引数))
            .collect::<Result<Vec<Child>, String>>()?;
        let バイト数の並び = std::thread::scope(|範囲| {
            let 読み手の並び: Vec<_> = プロセスの並び
                .into_iter()
                .map(|プロセス| 範囲.spawn(move || 終わりまで読む(プロセス)))
                .collect();
            読み手の並び
                .into_iter()
                .map(|読み手| {
                    読み手
                        .join()
                        .unwrap_or_else(|_| Err("読み手のスレッドが止まった".into()))
                })
                .collect::<Result<Vec<u64>, String>>()
        })?;
        let 読み切る時間 = 始め.elapsed();
        let 一本あたりのコマ数 = self.一本あたりのコマ数を求める(&バイト数の並び)?;
        一回の測定::作る(本数, 読み切る時間, 一本あたりのコマ数)
    }

    fn 起動する(&self, 引数: &[OsString]) -> Result<Child, String> {
        self.場所
            .ffmpegの命令()
            .args(引数)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(|原因| format!("ffmpeg の起動に失敗した: {原因}"))
    }

    fn 一本あたりのコマ数を求める(
        &self,
        バイト数の並び: &[u64],
    ) -> Result<u32, String> {
        let 一コマ = self.読み方.一コマのバイト数();
        let 最初 = バイト数の並び.first().copied().unwrap_or_default();
        let そろうか = バイト数の並び.iter().all(|バイト数| *バイト数 == 最初);
        if !そろうか || !最初.is_multiple_of(一コマ) {
            return Err(format!(
                "本ごとに読んだバイト数がそろわないか、1コマ({一コマ}バイト)の倍数でない: {バイト数の並び:?}"
            ));
        }
        u32::try_from(最初 / 一コマ).map_err(|_| format!("読んだコマ数が多すぎる({最初}バイト)"))
    }
}

/// 標準出力を終わりまで読んで捨て、読んだバイト数を返す。ffmpeg が失敗して終わったら失敗にする。
fn 終わりまで読む(mut プロセス: Child) -> Result<u64, String> {
    let mut 読んだバイト数 = 0_u64;
    if let Some(mut 標準出力) = プロセス.stdout.take() {
        let mut 塊 = vec![0_u8; 読む塊のバイト数];
        loop {
            let 読んだ = 標準出力
                .read(&mut 塊)
                .map_err(|原因| format!("ffmpeg の出力を読めない: {原因}"))?;
            if 読んだ == 0 {
                break;
            }
            読んだバイト数 += u64::try_from(読んだ).unwrap_or(u64::MAX);
        }
    }
    let 終了状態 = プロセス
        .wait()
        .map_err(|原因| format!("ffmpeg の終了を待てない: {原因}"))?;
    if 終了状態.success() {
        Ok(読んだバイト数)
    } else {
        Err(format!("ffmpeg が失敗した ({終了状態})"))
    }
}
