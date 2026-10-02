//! キーの割り当ての値。キーで行う操作の閉じた列挙と、操作ごとの既定のキーと、利用者の変更を重ねた今の割り当てを持つ。
//! 見る側の設定の1項目として settings.json に保存し、状態(`state`)と画面(`view/keys`)が今の割り当てからキー操作と設定のダイアログを作る。
//! キーの組は SengenEgui の `キーの組` で持ち、文字列にするのは settings.json の境界(persistence)だけである。参照: _doc/設計/画面.md 判断18

mod assignment;
mod default_keys;
mod operation;
mod operation_keys;
mod operation_text;
mod overlap;

#[cfg(test)]
mod assignment_tests;

pub(crate) use assignment::キーの割り当て;
pub(crate) use operation::キーで行う操作;
pub(crate) use operation_keys::操作のキー;
pub(crate) use overlap::キーを変えられない理由;
