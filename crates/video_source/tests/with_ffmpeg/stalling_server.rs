//! 止まる配信元。この計算機の中だけの TCP で動画のファイルを配り、2つ目からの接続には何も送らずに黙る。
//! 読んでいる途中で固まった ffmpeg を試験で作るために使う。ffprobe は1つ目の接続で動画の情報を調べられ、
//! その後に倉庫が起動した ffmpeg は2つ目の接続で入力を待ち続け、何も出力しない。

use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver};
use std::time::Duration;

use clip_domain::{入力された動画パス, 正規化した動画パス};

/// 止まる配信元とは、1つ目の接続へ動画のファイルを送って閉じ、2つ目の接続を黙って持ち続ける裏のスレッドと、
/// 2つ目の接続を受けたことの知らせを受け取る口の組のことである。試験の終わりまで接続を持ち続ける。
pub struct 止まる配信元 {
    アドレス: String,
    黙った知らせ: Receiver<()>,
}

impl 止まる配信元 {
    /// 動画のファイルを配る配信元を起動する。
    pub fn 起動する(ファイル: PathBuf) -> Self {
        let 待ち受け = TcpListener::bind("127.0.0.1:0").expect("待ち受けを開ける");
        let アドレス = format!("tcp://{}", 待ち受け.local_addr().expect("アドレス"));
        let (送り口, 黙った知らせ) = mpsc::channel();
        std::thread::spawn(move || {
            let mut 最初 = 待ち受け.accept().expect("1つ目の接続").0;
            // ffprobe は必要な分を読んだら接続を閉じることがあり、送り切れないのは失敗ではないため無視する。
            let _ = 最初.write_all(&std::fs::read(ファイル).expect("動画を読める"));
            drop(最初);
            let 黙る接続: TcpStream = 待ち受け.accept().expect("2つ目の接続").0;
            let _ = 送り口.send(());
            // 接続を閉じると ffmpeg が入力の終わりに気づいてしまうため、試験の間は持ち続ける。
            std::thread::sleep(Duration::from_secs(120));
            drop(黙る接続);
        });
        Self {
            アドレス,
            黙った知らせ,
        }
    }

    /// ffmpeg と ffprobe へ渡す入力のパス(tcp://127.0.0.1:<番号>)。
    pub fn 動画のパス(&self) -> 正規化した動画パス {
        入力された動画パス::作成する(self.アドレス.clone()).正規化する()
    }

    /// 2つ目の接続を受けて黙り始めるまで待つ。
    pub fn 黙るまで待つ(&self) {
        self.黙った知らせ
            .recv_timeout(Duration::from_secs(30))
            .expect("2つ目の接続が来ない");
    }
}
