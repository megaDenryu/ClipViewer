//! サムネイルの供給。ライブラリの一覧に出すスタックの顔と、重ね合わせの一覧に出す重ね合わせの顔を、キャッシュから読むか、無ければ裏のスレッドで撮って(FFmpeg)キャッシュへ置き、
//! 見えている行だけ画素へ戻してテクスチャにし、一覧の行へ渡す。何をいつ撮るか・読むかの判断は `一覧のサムネイル` に閉じる。
//! スタックの作業場と重ね合わせの作業場が、識別子の型だけを替えて別々に持つ(2つの作業場の画面が共有する置き場)。
//! 参照: _doc/設計/ライブラリ.md 判断10・判断11

mod bulk;
mod item;
mod list_rows;
mod list_sync;
mod means;
mod pixels;
mod pixels_arrival;
mod reader;
mod results;
mod rows;
mod schedule;
mod shelf;
mod shoot_order;
mod shooter;
mod show;
mod status;
mod textures;
mod worker;

#[cfg(test)]
mod list_rows_tests;
#[cfg(test)]
mod pixels_tests;
#[cfg(test)]
mod shoot_order_tests;
#[cfg(test)]
mod without_cache;
#[cfg(test)]
mod worker_tests;

#[cfg(test)]
pub(crate) use list_sync::一覧のサムネイルの大きさ;
pub(crate) use means::撮る手立て;
pub(crate) use rows::行の並び;
pub(crate) use shelf::{サムネイルの項目の表, 一覧のサムネイル};
pub(crate) use show::{サムネイルの見せ方, 途中の段階};
#[cfg(test)]
pub(crate) use status::サムネイルを撮らない理由;
