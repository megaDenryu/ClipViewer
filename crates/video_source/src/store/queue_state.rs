//! 依頼の待ち行列の中身。処理待ちの依頼と実行中の依頼を持ち、次に処理する依頼を保持の優先順で選ぶ。
//! 同期は持たない。複数のスレッドから使うときは `依頼の待ち行列` が鍵を掛けてから呼ぶ。

use clip_domain::動画上の区間;

use crate::process::{デコードの指定, 起動した子プロセス};

use super::priority::保持の優先順;
use super::ticket::受付の札;

/// 処理待ちの依頼とは、受け付けて、裏のスレッドがまだ始めていない区間のデコードの依頼のことである。
pub(crate) struct 処理待ちの依頼 {
    pub(crate) 札: 受付の札,
    pub(crate) 区間: 動画上の区間,
    pub(crate) 指定: デコードの指定,
}

/// 実行中の依頼とは、裏のスレッドが処理待ちから取り出した依頼の札と、起動した ffmpeg と、取り消されたかの組のことである。
/// 子プロセスが無いのは、取り出してから ffmpeg を預けるまでの間だけである。
struct 実行中の依頼 {
    札: 受付の札,
    子プロセス: Option<起動した子プロセス>,
    取り消されたか: bool,
}

/// 待ち行列の中身とは、受付順の処理待ちの依頼・保持の優先順・実行中の依頼・倉庫が落とされたかの組のことである。
#[derive(Default)]
pub(crate) struct 待ち行列の中身 {
    処理待ち: Vec<処理待ちの依頼>,
    優先順: 保持の優先順,
    実行中: Option<実行中の依頼>,
    止めるか: bool,
}

impl 待ち行列の中身 {
    pub(crate) fn 積む(&mut self, 依頼: 処理待ちの依頼) {
        self.処理待ち.push(依頼);
    }

    pub(crate) fn 優先順を置き換える(&mut self, 優先順: 保持の優先順) {
        self.優先順 = 優先順;
    }

    pub(crate) fn 止めるか(&self) -> bool {
        self.止めるか
    }

    /// 札の依頼を取り消す。処理待ちなら待ち行列から外し、実行中なら ffmpeg を止める。
    pub(crate) fn 取り消す(&mut self, 札: 受付の札) {
        self.処理待ち.retain(|依頼| 依頼.札 != 札);
        if let Some(実行中) = self.実行中.as_mut().filter(|実行中| 実行中.札 == 札) {
            実行中.止める();
        }
    }

    /// 倉庫を落とすときに、処理待ちの依頼をすべて捨て、実行中の ffmpeg を止める。
    pub(crate) fn すべて止める(&mut self) {
        self.止めるか = true;
        self.処理待ち.clear();
        if let Some(実行中) = self.実行中.as_mut() {
            実行中.止める();
        }
    }

    /// 保持の優先順で最も上位の処理待ちの依頼を取り出し、実行中として記録する。優先順に無い依頼は、受付順で最後に回る。
    pub(crate) fn 次の依頼を取り出す(&mut self) -> Option<処理待ちの依頼> {
        let 位置 = self
            .優先順
            .最も優先する位置(self.処理待ち.iter().map(|依頼| &依頼.区間))?;
        let 依頼 = self.処理待ち.remove(位置);
        self.実行中 = Some(実行中の依頼 {
            札: 依頼.札,
            子プロセス: None,
            取り消されたか: false,
        });
        Some(依頼)
    }

    /// 起動した ffmpeg を実行中の依頼へ預ける。預ける前に取り消されていたなら、その場で止める。
    pub(crate) fn 子プロセスを預ける(
        &mut self, mut 子プロセス: 起動した子プロセス
    ) {
        match self.実行中.as_mut() {
            Some(実行中) => {
                実行中.子プロセス = Some(子プロセス);
                if 実行中.取り消されたか {
                    実行中.止める();
                }
            }
            None => 子プロセス.止める(),
        }
    }

    pub(crate) fn 子プロセスを引き取る(&mut self) -> Option<起動した子プロセス> {
        self.実行中
            .as_mut()
            .and_then(|実行中| 実行中.子プロセス.take())
    }

    /// 実行中の依頼を終え、取り消されていたかを返す。
    pub(crate) fn 実行を終えて取り消されていたかを返す(&mut self) -> bool {
        self.実行中
            .take()
            .is_some_and(|実行中| 実行中.取り消されたか)
    }
}

impl 実行中の依頼 {
    fn 止める(&mut self) {
        self.取り消されたか = true;
        if let Some(子プロセス) = self.子プロセス.as_mut() {
            子プロセス.止める();
        }
    }
}
