//! 受け口のスレッド。接続を1つずつ受け、1行の本文を読み、合言葉が合えば頼みを画面のスレッドへ渡して「受け取った」を返す。
//! 1つの接続の読み書きには期限を置き、応答しない相手にスレッドを止められないようにする。

use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;
use std::thread::JoinHandle;
use std::time::Duration;

use eframe::egui;

use super::guide::合言葉;
use super::request::起動の頼み;
use super::wire::{受け取った返事, 受け渡す本文, 本文の長さの上限};

/// 1つの接続の読み書きの期限。
const 接続の読み書きの期限: Duration = Duration::from_secs(2);

/// 受け口のスレッドとは、受け口の裏のスレッドが所有する、聞き手・合言葉・画面のスレッドへの送り口・止める印・描き直しを頼む egui の本体の組のことである。
pub(super) struct 受け口のスレッド {
    pub(super) 聞き手: TcpListener,
    pub(super) 合言葉: 合言葉,
    pub(super) 送り口: Sender<起動の頼み>,
    pub(super) 止めるか: Arc<AtomicBool>,
    pub(super) eguiの本体: egui::Context,
}

impl 受け口のスレッド {
    /// 裏のスレッドを起こし、止める印が立つまで接続を受け続ける。
    pub(super) fn 起こす(self) -> io::Result<JoinHandle<()>> {
        std::thread::Builder::new()
            .name("起動の受け口".to_string())
            .spawn(move || self.受け続ける())
    }

    fn 受け続ける(self) {
        for 接続 in self.聞き手.incoming() {
            if self.止めるか.load(Ordering::SeqCst) {
                return;
            }
            // 注意: 1つの接続の失敗(期限切れ・形の不正・合言葉の違い)で受け口を止めない。その接続を捨てて次を待つ。
            if let Ok(接続) = 接続 {
                let _ = self.一つの接続を受ける(接続);
            }
        }
    }

    fn 一つの接続を受ける(&self, 接続: TcpStream) -> io::Result<()> {
        接続.set_read_timeout(Some(接続の読み書きの期限))?;
        接続.set_write_timeout(Some(接続の読み書きの期限))?;
        let mut 行 = String::new();
        BufReader::new((&接続).take(本文の長さの上限)).read_line(&mut 行)?;
        let 本文: 受け渡す本文 = serde_json::from_str(&行)
            .map_err(|原因| io::Error::new(io::ErrorKind::InvalidData, 原因))?;
        let Some(頼み) = 本文.合言葉を確かめて取り出す(&self.合言葉) else {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "合言葉が違う",
            ));
        };
        self.送り口.send(頼み).map_err(io::Error::other)?;
        self.eguiの本体.request_repaint();
        writeln!(&接続, "{受け取った返事}")
    }
}
