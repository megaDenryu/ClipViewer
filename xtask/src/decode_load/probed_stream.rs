//! ffprobe の `-of flat` の出力から読んだ1本の流れと、そこから映像の値を取り出す規則。
//! 参照: 取り出す規則は crates/video_source/src/probe/stream_shape.rs の `流れ` と同じである(xtask は video_source に依存しないため写している)。

use std::collections::BTreeMap;

use super::frame_rate::コマの速さ;
use super::frame_size::画素の寸法;

/// 平均のコマの速さを採る上限(コマ/秒)。これを超える平均は壊れた値とみなし、r_frame_rate を使う(video_source と同じ値)。
const 平均を採る速さの上限: f64 = 240.0;
/// flat の出力で、流れの項目の行が始まる表記。
const 流れの行の始まり: &str = "streams.stream.";

/// 調べた流れとは、ffprobe の出力の streams の1つの要素(映像・音などの1本の流れ)から、測るのに使う項目を読んだもののことである。
#[derive(Default)]
pub struct 調べた流れ {
    番号: Option<u32>,
    種類: Option<String>,
    幅: Option<u32>,
    高さ: Option<u32>,
    平均の速さ: Option<String>,
    基本の速さ: Option<String>,
    添付の画像か: bool,
    副データの回転: Option<f64>,
    付帯の回転: Option<f64>,
}

impl 調べた流れ {
    /// `streams.stream.<位置>.<項目>=<値>` の行の並びを、流れの並びへ読む。この形でない行があれば、その行を挙げて失敗にする。
    /// 使う項目(index・codec_type・width・height・avg_frame_rate・r_frame_rate・attached_pic・rotate・rotation)のほかは読み飛ばす。
    pub fn flatの出力から並びを読む(出力: &str) -> Result<Vec<Self>, String> {
        let mut 位置ごとの流れ: BTreeMap<u32, Self> = BTreeMap::new();
        for 行 in 出力.lines().map(str::trim).filter(|行| !行.is_empty()) {
            let (位置, 項目, 値) = 行を位置と項目と値に分ける(行)
                .ok_or_else(|| format!("ffprobe の出力の行「{行}」を読めない"))?;
            位置ごとの流れ
                .entry(位置)
                .or_default()
                .項目を受け取る(項目, 値);
        }
        Ok(位置ごとの流れ.into_values().collect())
    }

    fn 項目を受け取る(&mut self, 項目: &str, 値: &str) {
        match 項目 {
            "index" => self.番号 = 値.parse().ok(),
            "codec_type" => self.種類 = Some(値.to_owned()),
            "width" => self.幅 = 値.parse().ok(),
            "height" => self.高さ = 値.parse().ok(),
            "avg_frame_rate" => self.平均の速さ = Some(値.to_owned()),
            "r_frame_rate" => self.基本の速さ = Some(値.to_owned()),
            "disposition.attached_pic" => self.添付の画像か = 値 == "1",
            "tags.rotate" => self.付帯の回転 = 値.trim().parse().ok(),
            名前 if 名前.starts_with("side_data_list.") && 名前.ends_with(".rotation") => {
                self.副データの回転 = self.副データの回転.or_else(|| 値.parse().ok());
            }
            _ => {}
        }
    }

    /// 再生する映像の流れか。添付の画像(mp4 のカバー画像等)は映像の種類でも再生する映像ではない。
    pub fn 再生する映像か(&self) -> bool {
        self.種類.as_deref() == Some("video") && !self.添付の画像か
    }

    /// ファイルの中の流れの通し番号(ffprobe の index)。
    pub fn 番号(&self) -> Option<u32> {
        self.番号
    }

    /// 表示される向きの寸法。ffmpeg は回転の情報に従って自動で回すため、90度・270度の回転なら幅と高さが入れ替わる。
    pub fn 表示される寸法(&self) -> Option<画素の寸法> {
        let 寸法 = 画素の寸法::作成する(self.幅?, self.高さ?)?;
        let 回転 = self.副データの回転.or(self.付帯の回転);
        let 縦向きに回すか = 回転.is_some_and(|角度| ((角度 % 180.0).abs() - 90.0).abs() < 1.0);
        Some(if 縦向きに回すか {
            寸法.縦横を入れ替える()
        } else {
            寸法
        })
    }

    /// コマの速さ。平均が読めて上限以下なら平均を、そうでなければ r_frame_rate を使う。
    pub fn コマの速さを選ぶ(&self) -> Option<コマの速さ> {
        let 読む = |表記: &Option<String>| 表記.as_deref().and_then(コマの速さ::分数の表記から読む);
        let 平均 = 読む(&self.平均の速さ)
            .filter(|速さ| 速さ.一秒あたりのコマ数() <= 平均を採る速さの上限);
        平均.or_else(|| 読む(&self.基本の速さ))
    }
}

fn 行を位置と項目と値に分ける(行: &str) -> Option<(u32, &str, &str)> {
    let 残り = 行.strip_prefix(流れの行の始まり)?;
    let (位置, 残り) = 残り.split_once('.')?;
    let (項目, 値) = 残り.split_once('=')?;
    Some((位置.parse().ok()?, 項目, 値.trim_matches('"')))
}
