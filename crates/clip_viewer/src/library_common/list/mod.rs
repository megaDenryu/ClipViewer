//! ライブラリの一覧の見せ方。最後に読んだ一覧と、名前での絞り込みと並べ替えと、そこから作った見せる行の順と、見えている行の範囲を、
//! スタックの作業場(ライブラリの構えの一覧)と重ね合わせの作業場(重ね合わせの一覧)が、一覧の型だけを替えて共に使う。
//! 2つの作業場の状態はお互いを知らないため、どちらにも属さない置き場にする。参照: _doc/設計/ライブラリ.md 判断8・判断11

mod listing;
mod shown;
mod shown_rows;
mod view;
mod visible_rows;

#[cfg(test)]
mod view_tests;

pub(crate) use listing::{一覧の項目の参照, 並べられる一覧};
pub(crate) use shown::{並べた一覧, 最後に読んだ一覧};
pub(crate) use view::{並べ替え方, 絞り込みの語, 見せる行};
pub(crate) use visible_rows::見えている行の範囲;
