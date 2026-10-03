//! 負荷の測定の流れ: 測る動画のファイルを用意し、形を調べ、ffmpeg の数ごとに同時に読み切り、表を出す。

use super::arguments::測定の指定;
use super::one_stream::一本の読み方;
use super::reading::動画の同時読み出し;
use super::table::負荷の表;
use super::video_file::測る動画のファイル;
use super::video_shape::動画の形;
use crate::ffmpeg_location::FFmpegの置き場所;

/// 負荷の測定とは、見つけた FFmpeg の置き場所と、引数から決まった測定の指定の組のことである。
pub struct 負荷の測定 {
    pub ffmpegの置き場所: FFmpegの置き場所,
    pub 指定: 測定の指定,
}

impl 負荷の測定 {
    /// 測って、途中の経過と表を標準出力へ出す。合成画像の動画は、測り終えたら(失敗したときも)一時フォルダごと消える。
    pub fn 測る(&self) -> Result<(), String> {
        println!(
            "FFmpeg の場所: {}",
            self.ffmpegの置き場所.フォルダ().display()
        );
        println!(
            "FFmpeg の版: {}",
            self.ffmpegの置き場所.ffmpegの版を調べる()
        );
        let 動画 = self
            .指定
            .動画の出どころ
            .ファイルを用意する(&self.ffmpegの置き場所, self.指定.読む長さ)?;
        self.動画を測る(&動画)
    }

    fn 動画を測る(&self, 動画: &測る動画のファイル) -> Result<(), String> {
        let 形 = 動画の形::調べる(&self.ffmpegの置き場所, 動画)?;
        let 論理cpuの数 = std::thread::available_parallelism()
            .map_or_else(|_| "不明".to_owned(), |数| 数.to_string());
        println!(
            "測る動画: {動画}({}、{}コマ/秒、先頭から{}) → {} の RGBA で読み出す。論理 CPU の数: {論理cpuの数}",
            形.表示される寸法(),
            形.コマの速さ().一秒あたりのコマ数の表記(),
            self.指定.読む長さ,
            形.出力の寸法(),
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
                "  {ffmpegの数}本: {}(1本あたり{})",
                測定.読み切る時間(),
                測定.一本あたりのコマ数()
            );
            測定の並び.push(測定);
        }
        println!();
        print!(
            "{}",
            負荷の表::組み立てる(&測定の並び, 形.コマの速さ()).markdownの表として書く()
        );
        Ok(())
    }
}
