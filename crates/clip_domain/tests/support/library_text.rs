//! ライブラリのファイルの本文の試験の部品。1つのクリップを持つ第1版の本文を作り、指定の箇所を置き換えて壊す。

use clip_domain::*;

pub fn 読む(
    本文: &str,
) -> Result<ライブラリのスタック, ライブラリのファイルの読み込みエラー> {
    ライブラリのスタック::本文から読み込む(&ライブラリのファイルの本文::作成する(
        本文.to_string(),
    ))
}

/// 1つのクリップを持つ第1版の本文。置き換える箇所を指定して壊す。
pub fn 本文(置き換え: &[(&str, &str)]) -> String {
    let mut 本文 = concat!(
        r#"{"format":"ClipViewer.library","version":1,"identifier":"stack-1-1","name":"甲","#,
        r#""videoPath":"C:/v.mp4","createdAtMs":1,"updatedAtMs":2,"clips":[{"id":"c1","name":"一","#,
        r#""active":true,"start":1.0,"end":2.0,"repeat":2,"crop":{"x":0.0,"y":0.0,"w":100.0,"h":100.0},"#,
        r#""trigger":"Space"}]}"#
    )
    .to_string();
    for (前, 後) in 置き換え {
        assert!(本文.contains(前), "置き換える箇所が無い: {前}");
        本文 = 本文.replace(前, 後);
    }
    本文
}
