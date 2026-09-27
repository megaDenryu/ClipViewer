//! settings.json 第0版の、ファイル上の形そのままの型と、最新への変換。
//! 第0版とは、版を持たなかった最初の形(`{"ffmpegFolder": "..."}`)のことであり、format と version の両方が無いことで見分ける。
//! 第0版はもう書かない。読んで最新へ変換し、次に保存するときに第1版で書く。第1版が自分の項目として書く名前の項目は、第0版には意味が無いため読まずに捨てる。

use std::path::PathBuf;

use serde::Deserialize;
use serde_json::{Map, Value};

use super::settings::{アプリの設定, 知らない項目, 覚えた見る側の設定};
use super::v1::{ffmpegの置き場所へ変換する, 第1版が書く項目の名前};

#[derive(Deserialize)]
pub(super) struct 第0版の設定 {
    #[serde(rename = "ffmpegFolder", default)]
    ffmpeg_folder: Option<PathBuf>,
    #[serde(flatten)]
    その他: Map<String, Value>,
}

impl 第0版の設定 {
    pub(super) fn 最新へ変換する(self) -> アプリの設定 {
        let Self {
            ffmpeg_folder,
            mut その他,
        } = self;
        // 第1版が自分の項目として書く名前を、知らない項目から除く。残すと、第1版で書くときに同じ名前の項目が2回出る。
        for 名前 in 第1版が書く項目の名前 {
            その他.remove(名前);
        }
        アプリの設定 {
            ffmpegの置き場所: ffmpegの置き場所へ変換する(ffmpeg_folder),
            見る側: 覚えた見る側の設定::覚えていない,
            知らない項目: 知らない項目::作成する(その他),
        }
    }
}
