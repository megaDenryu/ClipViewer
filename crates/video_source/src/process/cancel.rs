//! 取り消しの合図。裏のスレッドで待っている子プロセスを、別のスレッドから止めさせるために使う。

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

/// 取り消しの合図とは、子プロセスの終わりを待っている側へ、待つのをやめて子プロセスを止めるよう伝える印のことである。
/// 複製した値はすべて同じ印を指す。一度取り消したら戻らない。
#[derive(Debug, Clone, Default)]
pub struct 取り消しの合図(Arc<AtomicBool>);

impl 取り消しの合図 {
    /// 取り消す。この合図を見て待っている子プロセスは、次の見回りで止められる。
    pub fn 取り消す(&self) {
        self.0.store(true, Ordering::Relaxed);
    }

    /// 取り消されたか。
    pub fn 取り消されたか(&self) -> bool {
        self.0.load(Ordering::Relaxed)
    }
}
