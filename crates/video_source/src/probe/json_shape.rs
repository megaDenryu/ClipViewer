//! ffprobe の JSON 出力の全体の形と、そこから動画の長さと流れを選ぶ規則。

use clip_domain::時間の長さ;
use serde::Deserialize;

use super::stream_shape::流れ;

/// 調査の出力とは、ffprobe の `-show_streams -show_format -of json` の出力の形のことである。
/// 流れの一覧には、映像・音・字幕などのすべての流れがファイルの中の順に並ぶ。
#[derive(Deserialize)]
pub(crate) struct 調査の出力 {
    #[serde(rename = "streams", default)]
    流れの一覧: Vec<流れ>,
    #[serde(rename = "format")]
    入れ物: Option<入れ物>,
}

#[derive(Deserialize)]
struct 入れ物 {
    #[serde(rename = "duration")]
    長さ: Option<String>,
}

impl 調査の出力 {
    /// 再生する最初の映像の流れ。添付の画像を飛ばす。
    pub(crate) fn 最初の映像(&self) -> Option<&流れ> {
        self.流れの一覧.iter().find(|流れ| 流れ.再生する映像か())
    }

    /// 音の流れが1つ以上あるか。音は最初の音の流れを読む(ffmpeg の流れの指定 `a:0`)。
    pub(crate) fn 音があるか(&self) -> bool {
        self.流れの一覧.iter().any(流れ::音か)
    }

    /// 動画の長さ。映像の流れの長さが無ければ入れ物の長さを使う。
    pub(crate) fn 長さ(&self) -> Option<時間の長さ> {
        let 流れの長さ = self.最初の映像().and_then(|流れ| 流れ.長さ.as_deref());
        let 入れ物の長さ = self
            .入れ物
            .as_ref()
            .and_then(|入れ物| 入れ物.長さ.as_deref());
        let 秒 = 流れの長さ.or(入れ物の長さ)?.trim().parse::<f64>().ok()?;
        時間の長さ::作成する(秒).ok()
    }
}
