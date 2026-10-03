//! `decode-load` コマンド: 同じ動画を 1本・2本・4本・8本の ffmpeg で同時に読み切る時間を測り、1本あたりのコマ/秒と
//! 再生の速さに対する倍率を表で出す。参照: _doc/設計/同時再生.md 5-4「デコードの速さ」
//! 時間のかかる測定であり、結果が計算機の負荷で揺れるため、verify には入れない。

mod arguments;
mod measurement;
mod one_stream;
mod read_length;
mod reading;
mod stream_count;
mod synthetic;
mod table;
mod video_shape;

#[cfg(test)]
mod arguments_tests;
#[cfg(test)]
mod one_stream_tests;
#[cfg(test)]
mod table_tests;
#[cfg(test)]
mod video_shape_tests;

use std::path::Path;

use arguments::{測る動画, 測定の指定};
use one_stream::一本の読み方;
use reading::動画の同時読み出し;
use synthetic::合成画像の動画;
use table::負荷の表;
use video_shape::動画の形;

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

/// 負荷の測定とは、見つけた FFmpeg の場所と、引数から決まった測定の指定の組のことである。
struct 負荷の測定 {
    ffmpegの置き場所: FFmpegの置き場所,
    指定: 測定の指定,
}

impl 負荷の測定 {
    fn 測る(&self) -> Result<(), String> {
        println!(
            "FFmpeg の場所: {}",
            self.ffmpegの置き場所.フォルダ().display()
        );
        println!(
            "FFmpeg の版: {}",
            self.ffmpegの置き場所.ffmpegの版を調べる()
        );
        match &self.指定.動画 {
            測る動画::合成画像で作る => {
                let 動画 =
                    合成画像の動画::作る(&self.ffmpegの置き場所, self.指定.読む長さ)?;
                self.動画を測る(動画.パス())
            }
            測る動画::与えられた動画(パス) => self.動画を測る(パス.パス()),
        }
    }

    fn 動画を測る(&self, 動画: &Path) -> Result<(), String> {
        let 形 = 動画の形::調べる(&self.ffmpegの置き場所, 動画)?;
        let 出力の寸法 = 形.元の寸法.長辺を上限へ縮める();
        let 論理cpuの数 = std::thread::available_parallelism()
            .map_or_else(|_| "不明".to_owned(), |数| 数.to_string());
        println!(
            "測る動画: {}({}x{}、{}コマ/秒、先頭から{}秒) → {}x{} の RGBA で読み出す。論理 CPU の数: {論理cpuの数}",
            動画.display(),
            形.元の寸法.幅,
            形.元の寸法.高さ,
            形.コマの速さ.毎秒のコマ数の表記(),
            self.指定.読む長さ.値(),
            出力の寸法.幅,
            出力の寸法.高さ,
        );
        let 読み出し = 動画の同時読み出し {
            ffmpegの置き場所: &self.ffmpegの置き場所,
            読み方: 一本の読み方 {
                動画,
                形,
                読む長さ: self.指定.読む長さ,
            },
        };
        let mut 測定の並び = Vec::new();
        for ffmpegの数 in &self.指定.ffmpegの数の並び {
            let 測定 = 読み出し.同時に読み切る(*ffmpegの数)?;
            println!(
                "  {}本: {:.1}秒(1本あたり{}コマ)",
                ffmpegの数.値(),
                測定.読み切る時間().as_secs_f64(),
                測定.一本あたりのコマ数()
            );
            測定の並び.push(測定);
        }
        println!();
        print!(
            "{}",
            負荷の表::組み立てる(&測定の並び, 形.コマの速さ).markdownの表として書く()
        );
        Ok(())
    }
}
