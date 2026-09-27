//! スタックのライブラリの項目と、その保存の形式。ファイルを知らず、ファイルの本文(文字列)との変換だけを持つ。
//! 版ごとの型(v1.rs)を置き、読み込みは版を判別して最新のライブラリのスタックへ変換し、書き出しは最新の版で行う。
//! 参照: _doc/設計/ライブラリ.md

mod date;
mod error;
mod file_text;
mod format;
mod import;
mod item;
mod item_change;
mod name;
mod stack_id;
mod thumbnail;
mod thumbnail_size;
mod v1;
mod v1_clip;
mod v1_read;
mod v1_values;
mod v1_write;

pub use date::{ライブラリの日時, ライブラリの日時エラー};
pub use error::{
    ライブラリのクリップの不備, ライブラリのファイルの書き出しエラー,
    ライブラリのファイルの読み込みエラー, ライブラリへ取り込めない理由,
};
pub use file_text::ライブラリのファイルの本文;
pub use item::ライブラリのスタック;
pub(crate) use name::複製の印;
pub use name::{スタックの名前, 空のスタックの名前エラー};
pub use stack_id::{スタックの識別子, スタックの識別子の不備};
pub use thumbnail::{サムネイルの撮り方, サムネイルの画像};
pub use thumbnail_size::サムネイルの大きさ;
