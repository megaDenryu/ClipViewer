//! 流し読み。溜めない区間と元動画の再生のために、開始の時刻から動画の終わりまでを順にデコードして受け渡す。
//! 参照: _doc/設計/アーキテクチャ.md 判断1-3

mod ending;
mod frame_stream;
mod messages;
mod reader_thread;

pub use ending::流し読みの状態;
pub use frame_stream::流し読み;
pub use messages::流し読みのコマ;
