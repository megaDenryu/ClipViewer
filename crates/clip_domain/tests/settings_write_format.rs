//! 書き出した本文が、ブラウザ版の `スタック設定をエクスポートする` と同じ項目名・同じ順・同じ表記を持つことの試験。
#![allow(clippy::expect_used)]

mod support;

use std::time::{Duration, SystemTime};

use clip_domain::*;
use support::試験用のスタック;

fn 書き出した本文() -> String {
    let スタック = 試験用のスタック();
    let パス = match 入力された動画パス::作成する("C:\\v\\a.mp4".to_string()).検証する(None)
    {
        動画パスの検証結果::有効(パス) => パス,
        動画パスの検証結果::不正 { 不備, .. } => panic!("有効なはず: {不備}"),
    };
    let 書き出す = 書き出す設定 {
        スタック: スタック.空でないことを確かめる().expect("空でない"),
        動画のパス: &パス,
    };
    let 時刻 = SystemTime::UNIX_EPOCH + Duration::from_millis(1_725_000_000_000);
    let 日時 = 書き出し日時::時刻から作成する(時刻).expect("1970年より後");
    書き出す
        .本文へ書き出す(&日時)
        .expect("書き出せる")
        .文字列()
        .to_string()
}

/// 本文の中で、項目名がこの順に現れることを確かめる。探し始めの位置を前の項目の後ろへずらしながら探す。
fn この順に現れる(本文: &str, 始まり: usize, 項目名の一覧: &[&str]) -> usize {
    let mut 探し始め = 始まり;
    for 項目名 in 項目名の一覧 {
        let 表記 = format!("\"{項目名}\":");
        let 位置 = 本文[探し始め..]
            .find(&表記)
            .unwrap_or_else(|| panic!("{項目名} が順に現れない"));
        探し始め += 位置 + 表記.len();
    }
    探し始め
}

#[test]
fn 書き出した本文はブラウザ版と同じ項目を同じ順に持つ() {
    let 本文 = 書き出した本文();
    let 見出し = [
        "application",
        "version",
        "exportDate",
        "expectedVideoName",
        "expectedVideoPath",
        "modifiers",
    ];
    let クリップの項目 = [
        "id",
        "name",
        "active",
        "start",
        "end",
        "repeat",
        "crop",
        "x",
        "y",
        "w",
        "h",
        "triggerEvent",
    ];
    let クリップの始まり = この順に現れる(&本文, 0, &見出し);
    この順に現れる(&本文, クリップの始まり, &クリップの項目);
    assert!(
        本文.starts_with("{\n  \"application\": \"ModifierVideoStack\",\n  \"version\": \"4.0\"")
    );
}

#[test]
fn 書き出した本文の値はブラウザ版と同じ表記である() {
    let 値: serde_json::Value = serde_json::from_str(&書き出した本文()).expect("JSONである");
    assert_eq!(値["exportDate"], "2024-08-30T06:40:00.000Z");
    assert_eq!(値["expectedVideoName"], "");
    assert_eq!(値["expectedVideoPath"], "C:\\v\\a.mp4");
    let 一つ目 = &値["modifiers"][0];
    assert_eq!(一つ目["id"], "A");
    assert_eq!(一つ目["name"], "名前");
    assert_eq!(一つ目["active"], true);
    assert_eq!(一つ目["start"], 10.0);
    assert_eq!(一つ目["end"], 12.0);
    assert_eq!(一つ目["repeat"], 3);
    assert_eq!(
        一つ目["crop"],
        serde_json::json!({"x": 0.0, "y": 0.0, "w": 100.0, "h": 100.0})
    );
    assert_eq!(一つ目["triggerEvent"], "Space");
    assert_eq!(値["modifiers"][1]["active"], false);
    assert_eq!(値["modifiers"][3]["repeat"], "infinite");
    assert_eq!(値["modifiers"][3]["triggerEvent"], "Enter");
    assert_eq!(値["modifiers"][4]["triggerEvent"], "Click");
    assert_eq!(値["modifiers"][2]["triggerEvent"], "none");
}
