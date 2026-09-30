//! 起動の引数と、以前の版からの受け渡しを受け取る層。現在の版は各起動で自分のウインドウを作り、動画をほかのアプリへ渡さない。
//! 起動の引数の読み方、錠を持つアプリが待つ受け口(この計算機の中だけの TCP)、試験で以前の版を再現する送り手、
//! 受け口のポートと合言葉を書いた案内のファイルを持つ。状態も応答も知らず、受け取った頼みを状態へ当てるのは配線(`app/`)である。
//! 参照: _doc/設計/画面.md 判断13

mod guide;
mod receiver;
mod receiver_thread;
mod request;
#[cfg(test)]
mod sender;
mod wire;

#[cfg(test)]
mod deadline_tests;
#[cfg(test)]
mod request_tests;
#[cfg(test)]
mod test_support;
#[cfg(test)]
mod transport_tests;

pub(crate) use guide::受け口の案内ファイル;
pub(crate) use receiver::{受け取っている受け口, 起動の受け口};
pub(crate) use request::{起動の頼み, 開かなかった動画の数};
#[cfg(test)]
pub(crate) use sender::起動の頼みの送り手;
