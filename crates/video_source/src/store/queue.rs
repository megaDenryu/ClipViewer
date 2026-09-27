//! 依頼の待ち行列。画面のスレッドが依頼を積み・取り消し、裏のスレッドが保持の優先順で次の依頼を取り出す。

use std::io;
use std::process::ExitStatus;
use std::sync::{Arc, Condvar, Mutex, MutexGuard, PoisonError};

use crate::process::起動した子プロセス;

use super::priority::保持の優先順;
use super::queue_state::{処理待ちの依頼, 待ち行列の中身};
use super::ticket::受付の札;

/// 依頼の待ち行列とは、待ち行列の中身を2つのスレッドで共有し、依頼が積まれるのを裏のスレッドが待てるようにしたもののことである。
#[derive(Clone, Default)]
pub(crate) struct 依頼の待ち行列(Arc<(Mutex<待ち行列の中身>, Condvar)>);

impl 依頼の待ち行列 {
    pub(crate) fn 積む(&self, 依頼: 処理待ちの依頼) {
        self.中身を借りる().積む(依頼);
        self.0.1.notify_all();
    }

    pub(crate) fn 優先順を置き換える(&self, 優先順: 保持の優先順) {
        self.中身を借りる().優先順を置き換える(優先順);
    }

    pub(crate) fn 取り消す(&self, 札: 受付の札) {
        self.中身を借りる().取り消す(札);
    }

    pub(crate) fn すべて止める(&self) {
        self.中身を借りる().すべて止める();
        self.0.1.notify_all();
    }

    /// 処理待ちの依頼が積まれるまで待ち、保持の優先順で最も上位の依頼を取り出す。倉庫が落とされたら `None` を返す。
    pub(crate) fn 次の依頼を待つ(&self) -> Option<処理待ちの依頼> {
        let mut 中身 = self.中身を借りる();
        loop {
            if 中身.止めるか() {
                return None;
            }
            if let Some(依頼) = 中身.次の依頼を取り出す() {
                return Some(依頼);
            }
            中身 = self.0.1.wait(中身).unwrap_or_else(PoisonError::into_inner);
        }
    }

    pub(crate) fn 子プロセスを預ける(&self, 子プロセス: 起動した子プロセス) {
        self.中身を借りる().子プロセスを預ける(子プロセス);
    }

    /// 預けた ffmpeg を引き取り、終わるまで待つ。待つ間は錠を取らない。
    pub(crate) fn 引き取って終わりを待つ(&self) -> Option<io::Result<ExitStatus>> {
        let 子プロセス = self.中身を借りる().子プロセスを引き取る();
        子プロセス.map(|mut 子プロセス| 子プロセス.終わりを待つ())
    }

    /// 実行中の依頼を終え、取り消されていたかを返す。
    pub(crate) fn 実行を終えて取り消されていたかを返す(&self) -> bool {
        self.中身を借りる().実行を終えて取り消されていたかを返す()
    }

    /// 中身を借りる。もう一方のスレッドがパニックしていても、中身は壊れた不変条件を持たないため、そのまま使う。
    fn 中身を借りる(&self) -> MutexGuard<'_, 待ち行列の中身> {
        self.0.0.lock().unwrap_or_else(PoisonError::into_inner)
    }
}
