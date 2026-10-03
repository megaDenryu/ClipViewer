//! 同じ動画を n 本の ffmpeg で同時に読み切り、時間と1本が読んだコマ数を測る。

use std::sync::atomic::AtomicBool;
use std::thread::ScopedJoinHandle;
use std::time::Instant;

use super::byte_count::バイト数;
use super::elapsed::読み切る時間;
use super::frame_count::コマ数;
use super::measurement::一回の測定;
use super::one_stream::一本の読み方;
use super::running_ffmpeg::読み出し中のffmpeg;
use super::stream_count::同時に起動するffmpegの数;
use super::stream_failure::一本の読み出しの失敗;
use crate::ffmpeg_location::FFmpegの置き場所;

/// 動画の同時読み出しとは、1本の読み方と、それを n 本起動する ffmpeg の置き場所の組のことである。
pub struct 動画の同時読み出し<'a> {
    pub ffmpegの置き場所: &'a FFmpegの置き場所,
    pub 読み方: 一本の読み方<'a>,
}

impl 動画の同時読み出し<'_> {
    /// n 本を同時に起動して読み切る。読み切る時間は、1本目を起動する直前から、すべてが終わるまでである。
    /// 1本でも失敗すれば残りも止め、理由を付けて失敗にする。本ごとに読んだコマ数が違うときも失敗にする。
    pub fn 同時に読み切る(
        &self,
        数: 同時に起動するffmpegの数,
    ) -> Result<一回の測定, String> {
        let 引数 = self.読み方.引数を並べる();
        let 始め = Instant::now();
        let 読み出し中の並び = 数
            .起動する番号の範囲()
            .map(|_| 読み出し中のffmpeg::ffmpegを起動する(self.ffmpegの置き場所, &引数))
            .collect::<Result<Vec<_>, String>>()?;
        let バイト数の並び = Self::全部の標準出力を読み切る(読み出し中の並び)?;
        let 時間 = 読み切る時間::作成する(始め.elapsed())
            .ok_or_else(|| "読み切る時間が0で速さを求められない".to_owned())?;
        let 一本あたりのコマ数 = self.一本あたりのコマ数を求める(&バイト数の並び)?;
        Ok(一回の測定::作成する(数, 時間, 一本あたりのコマ数))
    }

    // 1本ごとにスレッドで標準出力を読み、全部が終わるのを待つ。1本が失敗したら止める旗で残りを止める。
    fn 全部の標準出力を読み切る(
        読み出し中の並び: Vec<読み出し中のffmpeg>,
    ) -> Result<Vec<バイト数>, String> {
        let 止める旗 = AtomicBool::new(false);
        let 結果の並び = std::thread::scope(|範囲| {
            let 旗 = &止める旗;
            let 読み手の並び: Vec<_> = 読み出し中の並び
                .into_iter()
                .map(|ffmpeg| 範囲.spawn(move || ffmpeg.標準出力を終わりまで読む(旗)))
                .collect();
            読み手の並び
                .into_iter()
                .map(読み手の結果を受け取る)
                .collect()
        });
        一本の読み出しの失敗::結果の並びをまとめる(結果の並び)
    }

    fn 一本あたりのコマ数を求める(
        &self,
        バイト数の並び: &[バイト数],
    ) -> Result<コマ数, String> {
        let 一コマ = self.読み方.一コマのバイト数();
        let 最初 = バイト数の並び.first().copied().unwrap_or(バイト数::ゼロ);
        let そろうか = バイト数の並び.iter().all(|バイト数| *バイト数 == 最初);
        そろうか.then(|| 最初.割り切ってコマ数を求める(一コマ)).flatten().ok_or_else(|| {
            format!(
                "本ごとに読んだバイト数がそろわないか、1コマ({一コマ})の1以上の倍数でない: {バイト数の並び:?}"
            )
        })
    }
}

fn 読み手の結果を受け取る(
    読み手: ScopedJoinHandle<'_, Result<バイト数, 一本の読み出しの失敗>>,
) -> Result<バイト数, 一本の読み出しの失敗> {
    読み手.join().unwrap_or_else(|_| {
        Err(一本の読み出しの失敗::失敗した(
            "読み手のスレッドが止まった".to_owned(),
        ))
    })
}
