//! ffprobe の JSON 出力の streams の1つの要素の形と、そこから映像の値を取り出す規則。

use serde::Deserialize;

use crate::frame_rate::コマの速さ;
use crate::frame_size::コマの寸法;

/// 平均のコマの速さを採る上限(コマ/秒)。可変フレームレートの動画では r_frame_rate が時刻の刻みの細かさを表し、
/// 90000/1 のような表示に使えない値になるため、平均(avg_frame_rate)を優先し、これを超える平均は壊れた値とみなす。
const 平均を採る速さの上限: f64 = 240.0;

/// 流れとは、ffprobe の出力の streams の1つの要素(映像・音・字幕などの1本の流れ)の形のことである。
#[derive(Deserialize)]
pub(crate) struct 流れ {
    #[serde(rename = "index")]
    pub(super) 番号: Option<u32>,
    #[serde(rename = "codec_type")]
    種類: Option<String>,
    #[serde(rename = "width")]
    幅: Option<u32>,
    #[serde(rename = "height")]
    高さ: Option<u32>,
    #[serde(rename = "avg_frame_rate")]
    平均の速さ: Option<String>,
    #[serde(rename = "r_frame_rate")]
    基本の速さ: Option<String>,
    #[serde(rename = "duration")]
    pub(super) 長さ: Option<String>,
    #[serde(rename = "tags", default)]
    付帯情報: 流れの付帯情報,
    #[serde(rename = "side_data_list", default)]
    副データの一覧: Vec<副データ>,
    #[serde(rename = "disposition", default)]
    扱い: 流れの扱い,
}

#[derive(Deserialize, Default)]
struct 流れの付帯情報 {
    #[serde(rename = "rotate")]
    回転: Option<String>,
}

#[derive(Deserialize)]
struct 副データ {
    #[serde(rename = "rotation")]
    回転: Option<f64>,
}

#[derive(Deserialize, Default)]
struct 流れの扱い {
    #[serde(rename = "attached_pic", default)]
    添付の画像: u8,
}

impl 流れ {
    /// 再生する映像の流れか。添付の画像(mp4 のカバー画像等)は映像の種類でも再生する映像ではない。
    /// ffmpeg の流れの指定 `V`(添付の画像を除く映像)と同じ選び方である。
    pub(crate) fn 再生する映像か(&self) -> bool {
        self.種類.as_deref() == Some("video") && self.扱い.添付の画像 == 0
    }

    /// 音の流れか。
    pub(crate) fn 音か(&self) -> bool {
        self.種類.as_deref() == Some("audio")
    }

    /// 表示される向きの寸法。ffmpeg は回転の情報に従って自動で回して出力するため、90度・270度の回転なら幅と高さが入れ替わる。
    pub(crate) fn 表示される寸法(&self) -> Option<コマの寸法> {
        let 寸法 = コマの寸法::作成する(self.幅?, self.高さ?)?;
        let 付帯の回転 = self
            .付帯情報
            .回転
            .as_deref()
            .and_then(|表記| 表記.trim().parse::<f64>().ok());
        let 回転 = self
            .副データの一覧
            .iter()
            .find_map(|副データ| 副データ.回転)
            .or(付帯の回転);
        let 縦向きに回すか = 回転.is_some_and(|角度| ((角度 % 180.0).abs() - 90.0).abs() < 1.0);
        Some(if 縦向きに回すか {
            寸法.縦横を入れ替える()
        } else {
            寸法
        })
    }

    /// コマの速さ。平均が読めて上限以下なら平均を、そうでなければ r_frame_rate を使う。
    pub(crate) fn コマの速さを選ぶ(&self) -> Option<コマの速さ> {
        let 読む = |表記: &Option<String>| 表記.as_deref().and_then(コマの速さ::分数の表記から読む);
        let 平均 = 読む(&self.平均の速さ)
            .filter(|速さ| 速さ.一秒あたりのコマ数() <= 平均を採る速さの上限);
        平均.or_else(|| 読む(&self.基本の速さ))
    }
}
