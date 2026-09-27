//! スタックのライブラリの操作。登録・自動保存・一覧から開く・一覧の項目を変える操作・ブラウザ版の設定の取り込みを状態と裏のスレッドへ当てる。
//! 参照: _doc/設計/ライブラリ.md

mod discard;
mod edit;
mod import;
mod open;
mod ops;
mod register;
mod save_now;

pub(crate) use ops::ライブラリの操作;
