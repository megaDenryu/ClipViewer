//! ffprobe の出力を読む計算の試験。

use super::json::調査の出力を読む;
use super::{動画の情報の取得エラー, 読めない項目};

fn 出力(流れ: &str, 入れ物: &str) -> String {
    format!(r#"{{"streams":[{流れ}],"format":{入れ物}}}"#)
}

#[test]
fn 平均のコマの速さと流れの長さを読む() {
    let 本文 = 出力(
        r#"{"index":0,"codec_type":"video","width":1920,"height":1080,"avg_frame_rate":"30000/1001","r_frame_rate":"60/1","duration":"12.5"}"#,
        r#"{"duration":"13.0"}"#,
    );
    let Ok(値) = 調査の出力を読む(&本文) else {
        panic!("読めない")
    };
    assert_eq!((値.元の寸法.幅(), 値.元の寸法.高さ()), (1920, 1080));
    assert!((値.コマの速さ.一秒あたりのコマ数() - 29.97).abs() < 0.01);
    assert_eq!(値.長さ.秒数(), 12.5);
}

#[test]
fn 平均が壊れていれば基本の速さを使い_流れに長さが無ければ入れ物の長さを使う() {
    let 本文 = 出力(
        r#"{"index":0,"codec_type":"video","width":640,"height":360,"avg_frame_rate":"0/0","r_frame_rate":"25/1"}"#,
        r#"{"duration":"3.0"}"#,
    );
    let Ok(値) = 調査の出力を読む(&本文) else {
        panic!("読めない")
    };
    assert_eq!(値.コマの速さ.一秒あたりのコマ数(), 25.0);
    assert_eq!(値.長さ.秒数(), 3.0);
}

#[test]
fn 縦向きに回す動画は幅と高さを入れ替える() {
    let 副データ = 出力(
        r#"{"index":0,"codec_type":"video","width":1920,"height":1080,"avg_frame_rate":"30/1","duration":"1","side_data_list":[{"rotation":-90}]}"#,
        "{}",
    );
    let 付帯情報 = 出力(
        r#"{"index":0,"codec_type":"video","width":1920,"height":1080,"avg_frame_rate":"30/1","duration":"1","tags":{"rotate":"270"}}"#,
        "{}",
    );
    for 本文 in [副データ, 付帯情報] {
        let Ok(値) = 調査の出力を読む(&本文) else {
            panic!("読めない")
        };
        assert_eq!((値.元の寸法.幅(), 値.元の寸法.高さ()), (1080, 1920));
    }
}

#[test]
fn 映像が無ければ映像が無いエラーを返す() {
    let 結果 = 調査の出力を読む(r#"{"streams":[],"format":{}}"#);
    assert!(matches!(結果, Err(動画の情報の取得エラー::映像が無い)));
}

#[test]
fn 寸法が無ければ寸法を読めないエラーを返す() {
    let 本文 = 出力(
        r#"{"index":0,"codec_type":"video","avg_frame_rate":"30/1","duration":"1"}"#,
        "{}",
    );
    let 結果 = 調査の出力を読む(&本文);
    assert!(matches!(
        結果,
        Err(動画の情報の取得エラー::項目を読めない(
            読めない項目::寸法
        ))
    ));
}

#[test]
fn 本文が解釈できない形なら解釈できないエラーを返す() {
    assert!(matches!(
        調査の出力を読む("not json"),
        Err(動画の情報の取得エラー::出力を解釈できない { .. })
    ));
}
