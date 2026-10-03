//! `check-overlay-deps` コマンド: 重ね合わせの作業場の層(`crates/clip_viewer/src/overlay/`)が、スタックの作業場の `crate::state`・
//! `crate::command`・`crate::view` を使えないことと、ライブラリの部品の共有の置き場(`crates/clip_viewer/src/library_common/`)が、
//! それに加えて `crate::overlay`・`crate::app` も使えないことと、取り消しの履歴の共有の置き場(`crates/clip_viewer/src/edit_history/`)も同じく使えないことを検査する(検査する層は `layer.rs` が並べる)。調べないもの: マクロが組み立てるパス、`#[path]` と `include!` で読むファイル、
//! 検査する層の外のモジュールが再公開したもの(`crate::他::state` のような経由)。参照: _doc/設計/同時再生.md 3-2「依存の向きの検査」

mod extern_crate;
mod findings;
mod forbidden_reference;
mod layer;
mod module_path;
mod paths;
mod read_use_tree;
mod reader;
mod reason;
mod skipping;
mod source_root;
mod token;
mod tokens;
mod use_tree;
mod written_path;

#[cfg(test)]
mod folder_tests;
#[cfg(test)]
mod path_glob_alias_tests;
#[cfg(test)]
mod path_test_support;
#[cfg(test)]
mod path_tests;

use std::path::Path;

use layer::{検査する層, 検査する層の並び};
use source_root::ソースルート;

/// 検査する層ごとに検査を実行して結果を表示する。どれかの層で見つけたことが1つでもあるか、調べるファイルが無ければ失敗を返す。
pub fn 依存の向きを検査する(リポジトリルート: &Path) -> Result<(), String> {
    println!("> check-overlay-deps");
    let ソースルート = ソースルート::リポジトリから決める(リポジトリルート);
    let 失敗の並び: Vec<String> = 検査する層の並び
        .iter()
        .filter_map(|層| 層を検査して表示する(&ソースルート, 層).err())
        .collect();
    if 失敗の並び.is_empty() {
        Ok(())
    } else {
        Err(失敗の並び.join("。"))
    }
}

/// 1つの層を検査して、見つけたことと調べたファイルの数を表示する。
fn 層を検査して表示する(
    ソースルート: &ソースルート,
    層: &検査する層,
) -> Result<(), String> {
    let 結果 = ソースルート.層を検査する(層)?;
    for 見つけた in &結果.見つけたことの並び {
        println!("{見つけた}");
    }
    println!(
        "{}の {} 個のファイルを調べ、{} 件を見つけた",
        層.呼び名,
        結果.調べたファイルの数,
        結果.見つけたことの並び.len()
    );
    if 結果.調べたファイルの数 == 0 {
        Err(format!(
            "{}に調べるファイルが無い(置き場所が変わったなら検査の置き場所を直す)",
            層.呼び名
        ))
    } else if 結果.見つけたことの並び.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "{}が、使ってはならないモジュールを使っているか、調べられないファイルがある",
            層.呼び名
        ))
    }
}
