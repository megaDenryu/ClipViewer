//! ライブラリの一覧に出すサムネイルのキャッシュ。撮り方から求めた要約値をファイル名にして JPEG を置き、読み、捨てる。
//!
//! キャッシュは消えても撮り直せる派生のデータであり、ライブラリ(スタックのデータ)とは別の置き場所に置く。
//! 顔が変われば撮り方が変わって別のファイル名になるため、古いサムネイルを見せることが構造的に起きない。
//! 容量の上限を超えたら、使った時刻の古いものから消す。参照: _doc/設計/ライブラリ.md 判断10

#![warn(missing_docs)]

mod cache;
mod capacity;
mod clock;
mod error;
mod folder;
mod format_version;
mod place;
mod prune;
mod summary;
#[cfg(test)]
mod summary_tests;

pub use cache::サムネイルのキャッシュ;
pub use capacity::キャッシュの容量;
pub use clock::キャッシュの時計;
pub use error::キャッシュの読み書きエラー;
pub use folder::キャッシュのフォルダ;
pub use place::キャッシュの置き場所;
pub use summary::撮り方の要約値;
