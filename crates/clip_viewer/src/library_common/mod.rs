//! ライブラリの部品の共有の置き場。スタックの作業場(ライブラリの構えとヘッダーの登録の様子)と重ね合わせの作業場(重ね合わせの一覧とヘッダーの登録の様子)が、
//! 保存物・一覧・応答の型だけを替えて共に使う、ライブラリへの保存の追跡(`save/`)と一覧の見せ方(`list/`)と画面の部品(`view/`)を置く。
//! 2つの作業場はお互いを知らないため、どちらにも属さない置き場にする(`output_measure/` と同じ置き方)。参照: _doc/設計/ライブラリ.md 判断11

mod check_result;
mod close_decision;
pub(crate) mod list;
pub(crate) mod save;
pub(crate) mod view;

pub(crate) use check_result::確かめた結果;
pub(crate) use close_decision::アプリを閉じてよいとの決め;
