//! 重ね合わせ。同じ時刻に複数のクリップを並べて同時に再生するときの、再生するものの全部を表す型と、その不変条件を守る操作。
//! 同時再生とは、1つのタイムラインの上に並べた複数のクリップを、同じ時刻に、それぞれの場所と大きさで映し、
//! それぞれの音量で鳴らすことである。重ね合わせはクリップスタックとは別の型であり、クリップスタックを変えない。
//! 参照: _doc/設計/同時再生.md

mod aggregate;
mod arrange;
mod aspect;
mod basis;
mod error;
mod inherited;
mod placed_id;
mod rect;
mod rows;
mod video_table;
mod volume;

pub use aggregate::重ね合わせ;
pub use arrange::{スタックから並べた結果, スタックから並べるエラー};
pub use aspect::{重ねる画面の縦横比, 重ねる画面の縦横比エラー};
pub use basis::{重ねる画面に対する, 重ね合わせ上, 重ね合わせ上の秒};
pub use error::重ね合わせの操作エラー;
pub use inherited::クリップから受け継いだ値;
pub use placed_id::{
    空の置いたクリップの識別子エラー, 置いたクリップの識別子, 置いたクリップの識別子の発行元,
};
pub use rect::{映す矩形, 映す矩形エラー};
pub use rows::{
    タイムラインの行, タイムラインの行の並び, 同時に重ねられる行の数, 置いたクリップ, 行の番号,
};
pub use video_table::{使う動画の表, 動画の番号};
pub use volume::置いたクリップの音の設定;
