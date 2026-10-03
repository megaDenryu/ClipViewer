//! ライブラリへの保存の追跡。ライブラリの接続(裏で動くライブラリと頼みと知らせ)と、開いている登録済みの保存物の保存の様子・変更の見張り・頼み直しを、
//! スタックの作業場(`state/library/`)と重ね合わせの作業場(`overlay/state/library/`)が、保存物とライブラリの型だけを替えて共に使う。
//! 2つの作業場の状態はお互いを知らないため、どちらにも属さない置き場にする(`output_measure/` と同じ置き方)。
//! 参照: _doc/設計/ライブラリ.md 判断5・判断11

mod after_register;
mod change_watch;
mod connection;
mod content;
mod failure;
mod library_kind;
mod name_input;
mod purpose;
mod registered;
mod retry;
mod status;
mod usability;

#[cfg(test)]
mod change_watch_tests;

pub(crate) use after_register::登録の後にすること;
pub(crate) use change_watch::見た結果;
pub(crate) use connection::保存物のライブラリの接続;
pub(crate) use failure::ライブラリを使えない理由;
pub(crate) use name_input::入力中の名前;
pub(crate) use purpose::保存の目的の区別;
pub(crate) use registered::登録済みの保存物;
pub(crate) use status::保存の段階;
pub(crate) use usability::保存物のライブラリの使える様子;
