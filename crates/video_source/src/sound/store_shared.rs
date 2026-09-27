//! 音の倉庫を画面のスレッドと裏のスレッドで共有する状態。行の並びと、止めるよう求めたかと、読んでいる ffmpeg を1つの錠で守る。

use std::sync::{Condvar, Mutex, MutexGuard, PoisonError};

use clip_domain::動画上の区間;

use super::plan::音の溜め方;
use super::rows::音の行の並び;
use crate::process::起動した子プロセス;

/// 倉庫の状態とは、音の行の並びと、裏のスレッドに止まるよう求めたかと、裏のスレッドが読んでいる ffmpeg の組のことである。
/// ffmpeg を共有の側に置くのは、倉庫を落とす側が裏のスレッドを待たずに止められるようにするためである。
pub(super) struct 倉庫の状態 {
    pub(super) 行: 音の行の並び,
    止めるか: bool,
    読んでいる子プロセス: Option<起動した子プロセス>,
}

/// 倉庫の共有とは、画面のスレッドと裏のスレッドが共有する倉庫の状態と、状態が変わったことを知らせる合図の組のことである。
pub(super) struct 倉庫の共有 {
    状態: Mutex<倉庫の状態>,
    合図: Condvar,
}

impl 倉庫の共有 {
    /// 溜め方を持つ空の行の並びで作る。
    pub(super) fn 空(溜め方: 音の溜め方) -> Self {
        Self {
            状態: Mutex::new(倉庫の状態 {
                行: 音の行の並び::空(溜め方),
                止めるか: false,
                読んでいる子プロセス: None,
            }),
            合図: Condvar::new(),
        }
    }

    /// 溜める区間の並びを置き換え、次に読む区間を待っている裏のスレッドを起こす。
    pub(super) fn 並びを置き換える(&self, 並び: &[動画上の区間]) {
        self.錠を取る().行.並びを置き換える(並び);
        self.合図.notify_all();
    }

    /// 裏のスレッドに止まるよう求め、読んでいる ffmpeg を止める。裏のスレッドは読みかけの出力が閉じてすぐに気づく。
    pub(super) fn 止めるよう求める(&self) {
        let mut 状態 = self.錠を取る();
        状態.止めるか = true;
        if let Some(子プロセス) = 状態.読んでいる子プロセス.as_mut() {
            子プロセス.止める();
        }
        drop(状態);
        self.合図.notify_all();
    }

    /// 錠を取る。錠を持ったスレッドがパニックしても、行の並びは1回の置き換えの途中でしか崩れないため、中身をそのまま使う。
    pub(super) fn 錠を取る(&self) -> MutexGuard<'_, 倉庫の状態> {
        self.状態.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// 次に読む区間が出るまで待ち、読んでいるにして返す。止めるよう求められたら `None` を返す。裏のスレッドから呼ぶ。
    pub(super) fn 次に読む区間を待つ(&self) -> Option<動画上の区間> {
        let mut 状態 = self.錠を取る();
        loop {
            if 状態.止めるか {
                return None;
            }
            if let Some(区間) = 状態.行.次に読む区間() {
                return Some(区間);
            }
            状態 = self.合図.wait(状態).unwrap_or_else(PoisonError::into_inner);
        }
    }

    /// 読み始めた ffmpeg を預ける。倉庫が既に落とされていれば預からずに止めて、偽を返す。
    pub(super) fn 子プロセスを預ける(
        &self, 子プロセス: 起動した子プロセス
    ) -> bool {
        let mut 状態 = self.錠を取る();
        if 状態.止めるか {
            return false;
        }
        状態.読んでいる子プロセス = Some(子プロセス);
        true
    }

    /// 預けた ffmpeg を引き取る。終わりを待つのは錠の外で行う。
    pub(super) fn 子プロセスを引き取る(&self) -> Option<起動した子プロセス> {
        self.錠を取る().読んでいる子プロセス.take()
    }

    /// 読んでいる区間がまだ要るか。止めるよう求められたか、並びから外されたら要らない。
    pub(super) fn まだ要るか(&self, 区間: &動画上の区間) -> bool {
        let 状態 = self.錠を取る();
        !状態.止めるか && 状態.行.まだ読むか(区間)
    }
}
