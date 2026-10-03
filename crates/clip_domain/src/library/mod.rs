//! スタックのライブラリの項目(ライブラリのスタック)と重ね合わせのライブラリの項目(ライブラリの重ね合わせ)と、その保存の形式。
//! ファイルを知らず、ファイルの本文(文字列)との変換だけを持つ。版ごとの型(スタックは v1.rs、重ね合わせは overlay_v1.rs)を置き、
//! 読み込みは版を判別して最新の型へ変換し、書き出しは最新の版で行う。
//! 参照: _doc/設計/ライブラリ.md

mod date;
mod error;
mod file_text;
mod format;
mod import;
mod item;
mod item_change;
mod name;
mod overlay_error;
mod overlay_format;
mod overlay_id;
mod overlay_item;
mod overlay_name;
mod overlay_thumbnail;
mod overlay_v1;
mod overlay_v1_placed;
mod overlay_v1_read;
mod overlay_v1_values;
mod overlay_v1_write;
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
pub use overlay_error::{
    置いたクリップの値の不備, 重ね合わせのファイルの読み込みエラー
};
pub use overlay_id::{重ね合わせの識別子, 重ね合わせの識別子の不備};
pub use overlay_item::ライブラリの重ね合わせ;
pub use overlay_name::{空の重ね合わせの名前エラー, 重ね合わせの名前};
pub use stack_id::{スタックの識別子, スタックの識別子の不備};
pub use thumbnail::{サムネイルの撮り方, サムネイルの画像};
pub use thumbnail_size::サムネイルの大きさ;
