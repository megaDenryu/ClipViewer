//! コマの倉庫。区間のコマを裏のスレッドでデコードして RAM に溜め、保持の優先順に従って捨てる。
//! 参照: _doc/設計/アーキテクチャ.md 判断1

mod decoder_thread;
mod entry;
mod frame_store;
mod intake;
mod messages;
mod outcome;
mod priority;
mod queue;
mod queue_state;
mod record;
mod results;
mod room;
mod stored;
mod ticket;
mod worker;

pub use frame_store::コマの倉庫;
pub use outcome::{
    依頼の状況, 倉庫を開くエラー, 受け付けない理由, 溜める依頼の結果, 溜め終えた内訳,
};
pub use priority::保持の優先順;
pub use ticket::受付の札;

#[cfg(test)]
mod intake_tests;
#[cfg(test)]
mod queue_state_tests;
#[cfg(test)]
mod record_tests;
#[cfg(test)]
mod room_tests;
#[cfg(test)]
mod test_support;
