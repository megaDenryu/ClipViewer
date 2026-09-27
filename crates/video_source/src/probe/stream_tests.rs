//! ffprobe の出力から、読む映像の流れと音の有無を選ぶ規則の試験。

use super::json::調査の出力を読む;
use super::stream_index::{流れの番号, 音の有無};
use super::動画の情報の取得エラー;

fn 出力(流れ: &str, 入れ物: &str) -> String {
    format!(r#"{{"streams":[{流れ}],"format":{入れ物}}}"#)
}

#[test]
fn 添付の画像を飛ばして最初の映像の流れの番号を読み_音の流れがあれば音があるとする() {
    let 本文 = format!(
        r#"{{"streams":[{},{},{}],"format":{{}}}}"#,
        r#"{"index":0,"codec_type":"video","width":600,"height":600,"avg_frame_rate":"0/0","r_frame_rate":"90000/1","disposition":{"attached_pic":1}}"#,
        r#"{"index":1,"codec_type":"audio","duration":"2"}"#,
        r#"{"index":2,"codec_type":"video","width":640,"height":360,"avg_frame_rate":"30/1","duration":"2","disposition":{"attached_pic":0}}"#,
    );
    let Ok(値) = 調査の出力を読む(&本文) else {
        panic!("読めない")
    };
    assert_eq!(値.映像の流れ, 流れの番号::作成する(2));
    assert_eq!((値.元の寸法.幅(), 値.元の寸法.高さ()), (640, 360));
    assert_eq!(値.音の有無, 音の有無::ある);
}

#[test]
fn 音の流れが無ければ音が無いとする() {
    let 本文 = 出力(
        r#"{"index":0,"codec_type":"video","width":640,"height":360,"avg_frame_rate":"30/1","duration":"1"}"#,
        "{}",
    );
    let Ok(値) = 調査の出力を読む(&本文) else {
        panic!("読めない")
    };
    assert_eq!(値.音の有無, 音の有無::無い);
}

#[test]
fn 添付の画像しか無ければ映像が無いエラーを返す() {
    let 本文 = 出力(
        r#"{"index":0,"codec_type":"video","width":600,"height":600,"disposition":{"attached_pic":1}}"#,
        "{}",
    );
    assert!(matches!(
        調査の出力を読む(&本文),
        Err(動画の情報の取得エラー::映像が無い)
    ));
}
