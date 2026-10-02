//! 起動の受け口。1つ目のアプリが、この計算機の中だけの TCP のポートで2つ目のアプリからの頼みを待つ。
//! ウインドウを作る前に待ち始めて案内を書き(`開く`)、egui の本体ができてから裏のスレッドで受け取り始める(`受け取り始める`)。
//! その間に来た接続は OS の待ち行列に残るため、ウインドウを作っている間に起動した2つ目のアプリも頼みを渡せる。

use std::io;
use std::net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver};
use std::thread::JoinHandle;
use std::time::Duration;

use eframe::egui;

use super::guide::{受け口の案内, 受け口の案内ファイル, 合言葉};
use super::receiver_thread::受け口のスレッド;
use super::request::起動の頼み;

/// 止めるときに受け口のスレッドを起こす接続の期限。
const 起こす接続の期限: Duration = Duration::from_secs(1);

/// 起動の受け口とは、待ち始めて案内を書いたが、まだ受け取り始めていない受け口のことである。
#[derive(Debug)]
pub(crate) struct 起動の受け口 {
    聞き手: TcpListener,
    合言葉: 合言葉,
}

impl 起動の受け口 {
    /// この計算機の中だけのポート(OS が空いているものを選ぶ)で待ち始め、ポートと新しい合言葉を案内のファイルへ書く。
    pub(crate) fn 開く(案内ファイル: &受け口の案内ファイル) -> io::Result<Self> {
        let 聞き手 = TcpListener::bind(SocketAddr::from((Ipv4Addr::LOCALHOST, 0)))?;
        let 合言葉 = 合言葉::作る();
        案内ファイル.書く(&受け口の案内 {
            ポート: 聞き手.local_addr()?.port(),
            合言葉: 合言葉.clone(),
        })?;
        Ok(Self {
            聞き手, 合言葉
        })
    }

    /// 裏のスレッドで受け取り始める。頼みを受け取るたびに egui へ描き直しを頼み、画面のスレッドの次のフレームで取り出させる。
    pub(crate) fn 受け取り始める(
        self,
        画面描画の共有状態: egui::Context,
    ) -> io::Result<受け取っている受け口> {
        let 番地 = self.聞き手.local_addr()?;
        let 止めるか = Arc::new(AtomicBool::new(false));
        let (送り口, 受け取った頼み) = mpsc::channel();
        let スレッド = 受け口のスレッド {
            聞き手: self.聞き手,
            合言葉: self.合言葉,
            送り口,
            止めるか: Arc::clone(&止めるか),
            画面描画の共有状態,
        }
        .起こす()?;
        Ok(受け取っている受け口 {
            受け取った頼み,
            止めるか,
            番地,
            スレッド: Some(スレッド),
        })
    }
}

/// 受け取っている受け口とは、裏のスレッドが受け取った起動の頼みを、画面のスレッドが取り出す口のことである。
/// 落とすと裏のスレッドを止めて終わるのを待つ。
pub(crate) struct 受け取っている受け口 {
    受け取った頼み: Receiver<起動の頼み>,
    止めるか: Arc<AtomicBool>,
    番地: SocketAddr,
    スレッド: Option<JoinHandle<()>>,
}

impl 受け取っている受け口 {
    /// 届いている頼みをすべて取り出す。待たない。
    pub(crate) fn 届いた頼みを受け取る(&self) -> Vec<起動の頼み> {
        self.受け取った頼み.try_iter().collect()
    }
}

impl Drop for 受け取っている受け口 {
    /// 止める印を立て、接続を待って止まっているスレッドを自分への接続で起こしてから、終わるのを待つ。
    fn drop(&mut self) {
        self.止めるか.store(true, Ordering::SeqCst);
        let _ = TcpStream::connect_timeout(&self.番地, 起こす接続の期限);
        if let Some(スレッド) = self.スレッド.take() {
            let _ = スレッド.join();
        }
    }
}
