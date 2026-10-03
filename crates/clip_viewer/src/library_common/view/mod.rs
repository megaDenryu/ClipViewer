//! ライブラリの一覧とダイアログの画面の部品。サムネイルの枠・一覧の1行の形・仮想縦スクロールの行の並び・絞り込みと並べ替え・
//! 名前と確かめるダイアログの中身・保存の段階の文字・日時の表示を、スタックの作業場の画面(`view/library/`)と重ね合わせの作業場の画面(`overlay/view/library/`)が、
//! 応答の型だけを替えて共に使う。2つの作業場の画面はお互いを知らないため、どちらにも属さない置き場にする。参照: _doc/設計/ライブラリ.md 判断8・判断10・判断11

mod dialog;
mod row_frame;
mod rows;
mod stage_text;
mod thumbnail;
mod toolbar;

pub(crate) use dialog::{名前のダイアログ, 確かめるダイアログ};
pub(crate) use row_frame::{行の印, 読めた行, 読めない行};
pub(crate) use rows::{一覧の行の並び, 一覧の行の材料};
pub(crate) use stage_text::{保存の段階の文字, 日時の表示};
pub(crate) use toolbar::絞り込みと並べ替えの材料;
