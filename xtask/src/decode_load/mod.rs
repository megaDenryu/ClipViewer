//! `decode-load` コマンド: 同じ動画を 1本・2本・4本・8本の ffmpeg で同時に読み切る時間を測り、1本あたりの読み出しの速さと
//! 再生の速さに対する倍率を表で出す。参照: _doc/設計/同時再生.md 5-4「デコードの速さ」
//! 時間のかかる測定であり、結果が計算機の負荷で揺れるため、verify には入れない。

mod arguments;
mod byte_count;
mod elapsed;
mod frame_count;
mod frame_rate;
mod frame_size;
mod load_measurement;
mod measurement;
mod one_stream;
mod playback_ratio;
mod probed_stream;
mod read_length;
mod read_speed;
mod reading;
mod running_ffmpeg;
mod stream_count;
mod stream_failure;
mod synthetic;
mod table;
mod video_file;
mod video_origin;
mod video_shape;

#[cfg(test)]
mod arguments_tests;
#[cfg(test)]
mod frame_size_tests;
#[cfg(test)]
mod measurement_tests;
#[cfg(test)]
mod one_stream_tests;
#[cfg(test)]
mod read_length_tests;
#[cfg(test)]
mod stream_count_tests;
#[cfg(test)]
mod stream_failure_tests;
#[cfg(test)]
mod table_tests;
#[cfg(test)]
mod video_shape_tests;

pub use read_length::読む秒数;
pub use synthetic::合成画像の動画;

use arguments::測定の指定;
use load_measurement::負荷の測定;

use crate::ffmpeg_location::{FFmpegの置き場所, 場所を渡す環境変数};

/// 引数を解釈して測り、表を標準出力へ出す。FFmpeg が見つからないときは、探した場所を付けて失敗にする。
pub fn 動画の同時読み出しの負荷を測る(
    引数一覧: &[String]
) -> Result<(), String> {
    let 指定 = 測定の指定::引数から解釈する(引数一覧)?;
    let ffmpegの置き場所 = FFmpegの置き場所::探す().ok_or_else(|| {
        format!("ffmpeg と ffprobe が同じフォルダにそろう場所が、環境変数 {場所を渡す環境変数} にも PATH にも無い")
    })?;
    負荷の測定 {
        ffmpegの置き場所,
        指定,
    }
    .測る()
}
