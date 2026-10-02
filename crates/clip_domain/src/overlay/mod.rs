//! 重ね合わせ。同じ時刻に複数のクリップを並べて同時に再生するときの、再生するものの全部を表す型と、その不変条件を守る操作。
//! 同時再生とは、1つのタイムラインの上に並べた複数のクリップを、同じ時刻に、それぞれの場所と大きさで映し、
//! それぞれの音量で鳴らすことである。重ね合わせはクリップスタックとは別の型であり、クリップスタックを変えない。
//! 参照: _doc/設計/同時再生.md

mod aggregate;
mod aggregate_edit;
mod aggregate_place;
mod aspect;
mod basis;
mod error;
mod inherited;
mod placed;
mod placed_id;
mod rect;
mod row;
mod video_table;
mod volume;

pub use aggregate::{同時に重ねられる行の数, 重ね合わせ};
pub use aspect::{重ねる画面の縦横比, 重ねる画面の縦横比エラー};
pub use basis::{重ねる画面に対する, 重ね合わせ上, 重ね合わせ上の秒};
pub use error::重ね合わせの操作エラー;
pub use inherited::クリップから受け継いだ値;
pub use placed::置いたクリップ;
pub use placed_id::{
    置いたクリップの識別子, 置いたクリップの識別子の発行元
};
pub use rect::{映す矩形, 映す矩形エラー};
pub use row::{タイムラインの行, 行の番号};
pub use video_table::{使う動画の表, 動画の番号};
pub use volume::{置いたクリップの音量, 音の大きさ};
