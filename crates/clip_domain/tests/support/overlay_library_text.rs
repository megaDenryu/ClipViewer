//! 重ね合わせのファイルの本文の試験の部品。1つの置いたクリップを持つ第1版の本文を作り、指定の箇所を置き換えて壊す。

use clip_domain::*;

pub fn 読む(
    本文: &str,
) -> Result<ライブラリの重ね合わせ, 重ね合わせのファイルの読み込みエラー> {
    ライブラリの重ね合わせ::本文から読み込む(
        &ライブラリのファイルの本文::作成する(本文.to_string()),
    )
}

/// 1つの行に1つの置いたクリップを持つ第1版の本文。置き換える箇所を指定して壊す。
pub fn 本文(置き換え: &[(&str, &str)]) -> String {
    let mut 本文 = concat!(
        r#"{"format":"ClipViewer.overlay","version":1,"identifier":"overlay-1-1","name":"甲","#,
        r#""createdAtMs":1,"updatedAtMs":2,"aspect":"16:9","videos":["C:/v.mp4"],"rows":[{"items":["#,
        r#"{"id":"p1","name":"一","video":0,"start":1.0,"end":2.0,"repeat":2,"#,
        r#""crop":{"x":0.0,"y":0.0,"w":100.0,"h":100.0},"at":0.0,"#,
        r#""rect":{"x":0.0,"y":0.0,"w":100.0,"h":100.0},"volume":1.0,"muted":false}]}]}"#
    )
    .to_string();
    for (前, 後) in 置き換え {
        assert!(本文.contains(前), "置き換える箇所が無い: {前}");
        本文 = 本文.replace(前, 後);
    }
    本文
}
