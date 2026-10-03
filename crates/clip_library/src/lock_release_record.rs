//! 錠を放した順の記録。試験のときだけ組み込み、錠のファイルを落とした順を、試験を走らせているスレッドごとに記録する。
//! スレッドごとに分けるのは、並んで走るほかの試験の錠と混ざらないためである。

use std::cell::RefCell;
use std::path::{Path, PathBuf};

thread_local! {
    static 放した錠のフォルダ: RefCell<Vec<PathBuf>> = const { RefCell::new(Vec::new()) };
}

/// 錠を放したフォルダを記録する。錠のファイルを落とすときに呼ぶ。
pub(crate) fn 放した錠を記録する(フォルダ: &Path) {
    放した錠のフォルダ.with(|記録| 記録.borrow_mut().push(フォルダ.to_path_buf()));
}

/// これまでの記録を取り出し、記録を空にする。
pub(crate) fn 記録を取り出す() -> Vec<PathBuf> {
    放した錠のフォルダ.with(|記録| std::mem::take(&mut *記録.borrow_mut()))
}
