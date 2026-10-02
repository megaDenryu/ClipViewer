//! 重ね合わせの再生の規則。重ね合わせ上の時刻に行ごとに映すものを求める処理と、重ね合わせの時計の規則。
//! 画面と供給は、時計で進めた時刻を映すものを求める処理へ渡し、行ごとの結果から映像と音の指示を組み立てる。

mod at_time;
mod at_time_result;
mod clock;
mod row_lookup;
mod span_end;

pub use at_time_result::{
    置いたクリップの再生の中身, 行が映すもの, 行の映し方
};
pub use clock::{重ね合わせの再生位置, 重ね合わせの時計を進めた結果};
pub use span_end::置いたクリップの区間の終わりの行き先;
