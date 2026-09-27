//! ffprobe で動画の情報を調べる。

mod error;
mod json;
mod json_shape;
mod stream_index;
mod stream_shape;

pub(crate) use error::調査の失敗;
pub use error::{動画の情報の取得エラー, 読めない項目};
pub(crate) use json::調査の出力を読む;
pub(crate) use stream_index::流れの番号;
pub use stream_index::音の有無;

#[cfg(test)]
mod json_tests;
#[cfg(test)]
mod stream_tests;
