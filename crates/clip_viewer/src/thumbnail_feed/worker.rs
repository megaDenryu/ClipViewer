//! 仕事を順に行う裏のスレッド。1本の裏のスレッドに仕事を頼んだ順に行わせ、結果を画面のスレッドが待たずに受け取る。
//! サムネイルを撮る(FFmpeg を起動する)ことと、キャッシュを読んで画素へ戻すことを、画面のスレッドを止めずに行うために使う。

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::JoinHandle;

/// 裏のスレッドで仕事を行う処理とは、仕事を行うのに要る依存(キャッシュ等)を保持し、裏のスレッドで仕事を1つずつ行って結果を返すもののことである。
pub(super) trait 裏のスレッドで仕事を行う処理: Send + 'static {
    /// 頼む仕事。
    type 仕事: Send + 'static;
    /// 行った結果。
    type 結果: Send + 'static;

    /// 仕事を行う。裏のスレッドで呼ばれる。
    fn 行う(&mut self, 仕事: Self::仕事) -> Self::結果;
}

/// 裏のスレッドが止まっているとは、裏のスレッドが異常終了していて仕事を受け取れないことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("サムネイルの裏のスレッドが止まっている")]
pub(crate) struct 裏のスレッドが止まっている;

/// 動いているスレッドとは、裏のスレッドと、そのスレッドへ仕事を送る口の組のことである。同時に生まれて同時に消える。
struct 動いているスレッド<仕事> {
    送り口: Sender<仕事>,
    スレッド: JoinHandle<()>,
}

/// 仕事を順に行う裏のスレッドとは、処理を持って仕事を頼んだ順に行う1本のスレッドの取っ手と、結果の受け口と、
/// 返事を待っている仕事の数の組のことである。落とすと、まだ始めていない仕事を捨て、行っている途中の仕事を終えるまで待ってから閉じる。
/// 途中の仕事を長く待たせないのは処理の役目である(撮る処理は取り消しの合図で ffmpeg を止める)。
pub(super) struct 仕事を順に行う裏のスレッド
<行う処理: 裏のスレッドで仕事を行う処理> {
    スレッド: Option<動いているスレッド<行う処理::仕事>>,
    結果の受け口: Receiver<行う処理::結果>,
    止める印: Arc<AtomicBool>,
    返事を待っている数: usize,
}

impl<行う処理: 裏のスレッドで仕事を行う処理> 仕事を順に行う裏のスレッド<行う処理> {
    /// 処理を渡して裏のスレッドを起こす。
    pub(super) fn 起こす(処理: 行う処理) -> Self {
        let (送り口, 仕事の受け口) = mpsc::channel();
        let (結果の送り口, 結果の受け口) = mpsc::channel();
        let 止める印 = Arc::new(AtomicBool::new(false));
        let スレッドの止める印 = Arc::clone(&止める印);
        let スレッド = std::thread::spawn(move || {
            仕事を続ける(処理, &仕事の受け口, &結果の送り口, &スレッドの止める印)
        });
        Self {
            スレッド: Some(動いているスレッド {
                送り口, スレッド
            }),
            結果の受け口,
            止める印,
            返事を待っている数: 0,
        }
    }

    /// 仕事を頼む。待たない。
    pub(super) fn 頼む(
        &mut self,
        仕事: 行う処理::仕事,
    ) -> Result<(), 裏のスレッドが止まっている> {
        self.スレッド
            .as_ref()
            .ok_or(裏のスレッドが止まっている)?
            .送り口
            .send(仕事)
            .map_err(|_| 裏のスレッドが止まっている)?;
        self.返事を待っている数 += 1;
        Ok(())
    }

    /// 届いている結果をすべて受け取る。待たない。スレッドが止まっていれば、もう届かない返事を待つのをやめる。
    pub(super) fn 届いた結果を受け取る(&mut self) -> Vec<行う処理::結果> {
        let 結果: Vec<_> = self.結果の受け口.try_iter().collect();
        self.返事を待っている数 = self.返事を待っている数.saturating_sub(結果.len());
        if self.止まったか() {
            self.返事を待っている数 = 0;
        }
        結果
    }

    /// 頼んだ仕事のうち、結果をまだ受け取っていない数。
    pub(super) fn 返事を待っている数(&self) -> usize {
        self.返事を待っている数
    }

    /// 裏のスレッドが異常終了したか。送り口を閉じるのは落とすときだけなので、それまでに終わったなら異常終了である。
    pub(super) fn 止まったか(&self) -> bool {
        self.スレッド
            .as_ref()
            .is_none_or(|動いている| 動いている.スレッド.is_finished())
    }
}

impl<行う処理: 裏のスレッドで仕事を行う処理> Drop
    for 仕事を順に行う裏のスレッド<行う処理>
{
    fn drop(&mut self) {
        self.止める印.store(true, Ordering::Relaxed);
        if let Some(動いている) = self.スレッド.take() {
            drop(動いている.送り口);
            let _ = 動いている.スレッド.join();
        }
    }
}

/// 裏のスレッドの本体。受けた順に仕事を行い、結果を返す。止める印が立ったら残りの仕事を行わずに終える。
fn 仕事を続ける<行う処理: 裏のスレッドで仕事を行う処理>(
    mut 処理: 行う処理,
    受け口: &Receiver<行う処理::仕事>,
    結果の送り口: &Sender<行う処理::結果>,
    止める印: &AtomicBool,
) {
    for 届いた in 受け口 {
        if 止める印.load(Ordering::Relaxed) || 結果の送り口.send(処理.行う(届いた)).is_err()
        {
            return;
        }
    }
}
