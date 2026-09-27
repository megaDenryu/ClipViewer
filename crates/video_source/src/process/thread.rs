//! 落としたときに終わりを待つスレッドの型。

use std::io;
use std::thread::{Builder, JoinHandle};

/// 終わりを待つスレッドとは、落とされたときにスレッドの終わりを待ってから消える、裏のスレッドの取っ手のことである。
/// スレッドを置き去りにしないために使う。取っ手を `Option` で持つのは、待つ操作が取っ手を消費するためであり、
/// `None` になるのは待ち終えた後だけである。
pub(crate) struct 終わりを待つスレッド<結果: Send + 'static> {
    取っ手: Option<JoinHandle<結果>>,
}

impl<結果: Send + 'static> 終わりを待つスレッド<結果> {
    /// 名前を付けてスレッドを起動する。
    pub(crate) fn 起動する<処理: FnOnce() -> 結果 + Send + 'static>(
        名前: &str,
        本体: 処理,
    ) -> io::Result<Self> {
        let 取っ手 = Builder::new().name(名前.to_string()).spawn(本体)?;
        Ok(Self {
            取っ手: Some(取っ手),
        })
    }

    /// スレッドが終わっているか。待ち終えた後も終わっているとみなす。
    pub(crate) fn 終わったか(&self) -> bool {
        self.取っ手.as_ref().is_none_or(JoinHandle::is_finished)
    }

    /// スレッドが終わるまで待ち、結果を受け取る。スレッドがパニックしたか、既に待ち終えているなら結果は無い。
    pub(crate) fn 終わるまで待つ(&mut self) -> Option<結果> {
        self.取っ手.take().and_then(|取っ手| 取っ手.join().ok())
    }
}

impl<結果: Send + 'static> Drop for 終わりを待つスレッド<結果> {
    fn drop(&mut self) {
        if let Some(取っ手) = self.取っ手.take() {
            // スレッドのパニックは既に失敗として扱われた後なので、ここでは終わったことだけを待つ。
            let _ = 取っ手.join();
        }
    }
}
