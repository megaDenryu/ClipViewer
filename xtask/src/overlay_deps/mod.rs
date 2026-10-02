//! `check-overlay-deps` コマンド: 重ね合わせの作業場の層(`crates/clip_viewer/src/overlay/` の下)が、スタックの作業場の
//! `crate::state`・`crate::command`・`crate::view` を使っていないことを検査する。1つのクレートの中のモジュールの向きは Cargo では強制できないためである。
//! `use` の木と式や型に書いたパスを、`crate`・`super`・`self` から crate ルートのパスへ直して調べる。コメントと文字列の中は調べない。
//! 調べないもの: マクロが組み立てるパス(`$crate` を除く)、`#[path]` の属性と `include!` で読むファイル。
//! 読めないファイル・UTF-8 でないファイル・字句やパスを解析できないファイルは、黙って外さずに見つけたこととして報告する。
//! 参照: _doc/設計/同時再生.md 3-2「依存の向きの検査」

mod findings;
mod module_path;
mod paths;
mod reader;
mod source_root;
mod tokens;
mod use_tree;

#[cfg(test)]
mod folder_tests;
#[cfg(test)]
mod path_tests;

use std::path::Path;

use source_root::ソースルート;

/// 検査を実行して結果を表示する。見つけたことが1つでもあれば失敗を返す。
pub fn 依存の向きを検査する(リポジトリルート: &Path) -> Result<(), String> {
    println!("> check-overlay-deps");
    let 結果 = ソースルート::リポジトリから決める(リポジトリルート).重ね合わせの層を検査する()?;
    for 見つけた in &結果.見つけたことの並び {
        println!("{見つけた}");
    }
    println!(
        "重ね合わせの作業場の層の {} 個のファイルを調べ、{} 件を見つけた",
        結果.調べたファイルの数,
        結果.見つけたことの並び.len()
    );
    if 結果.調べたファイルの数 == 0 {
        Err("重ね合わせの作業場の層に調べるファイルが無い(置き場所が変わったなら検査の置き場所を直す)".to_string())
    } else if 結果.見つけたことの並び.is_empty() {
        Ok(())
    } else {
        Err("重ね合わせの作業場の層が、スタックの作業場のモジュールを使っているか、調べられないファイルがある".to_string())
    }
}
