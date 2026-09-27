//! 起動の頼みの送り手。2つ目のアプリが、案内のファイルから1つ目のアプリの受け口を知り、頼みを渡して返事を待つ。
//! 期限までは渡し直す(1つ目が起動の途中でまだ案内を書いていないことがあるため)。期限を過ぎたら諦め、呼び出し側は普通に起動する。

use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::time::{Duration, Instant};

use super::guide::受け口の案内ファイル;
use super::request::起動の頼み;
use super::wire::{受け取った返事, 受け渡す本文};

/// 渡し直すまでの間隔。
const 渡し直す間隔: Duration = Duration::from_millis(100);

/// 渡せなかった理由とは、期限までに1つ目のアプリへ頼みを渡せなかった理由の区別のことである(最後に試したときの理由)。
#[derive(Debug, thiserror::Error)]
pub(crate) enum 渡せなかった理由 {
    #[error("受け口の案内を読めない: {0}")]
    案内を読めない(io::Error),
    #[error("受け口につなげない: {0}")]
    つなげない(io::Error),
    #[error("受け口とやり取りできない(応答しない): {0}")]
    やり取りできない(io::Error),
    #[error("受け口の返事が違う")]
    返事が違う,
}

/// 起動の頼みの送り手とは、受け口の案内ファイルと、頼みを渡し終えるまで待つ期限の組のことである。
#[derive(Debug, Clone)]
pub(crate) struct 起動の頼みの送り手 {
    案内ファイル: 受け口の案内ファイル,
    期限: Duration,
}

impl 起動の頼みの送り手 {
    pub(crate) fn 作成する(
        案内ファイル: 受け口の案内ファイル, 期限: Duration
    ) -> Self {
        Self {
            案内ファイル, 期限
        }
    }

    /// 頼みを渡し、受け口が「受け取った」と返すまで待つ。期限までは渡し直し、過ぎたら最後の理由を返す。時計を読む境界はここである。
    pub(crate) fn 送る(&self, 頼み: &起動の頼み) -> Result<(), 渡せなかった理由> {
        let 期限の時刻 = Instant::now() + self.期限;
        loop {
            let 残り = 期限の時刻.saturating_duration_since(Instant::now());
            let 理由 = match self.一度渡す(頼み, 残り) {
                Ok(()) => return Ok(()),
                Err(理由) => 理由,
            };
            if Instant::now() + 渡し直す間隔 >= 期限の時刻 {
                return Err(理由);
            }
            std::thread::sleep(渡し直す間隔);
        }
    }

    fn 一度渡す(
        &self,
        頼み: &起動の頼み,
        残り: Duration,
    ) -> Result<(), 渡せなかった理由> {
        let 案内 = self
            .案内ファイル
            .読む()
            .map_err(渡せなかった理由::案内を読めない)?;
        let 期限 = 残り.max(Duration::from_millis(1));
        let 接続 =
            TcpStream::connect_timeout(&案内.番地(), 期限).map_err(渡せなかった理由::つなげない)?;
        let 本文 =
            serde_json::to_string(&受け渡す本文::作成する(案内.合言葉, 頼み)).map_err(|原因| {
                渡せなかった理由::やり取りできない(io::Error::other(原因))
            })?;
        let mut 返事 = String::new();
        接続
            .set_read_timeout(Some(期限))
            .and_then(|()| 接続.set_write_timeout(Some(期限)))
            .and_then(|()| writeln!(&接続, "{本文}"))
            .and_then(|()| BufReader::new(&接続).read_line(&mut 返事))
            .map_err(渡せなかった理由::やり取りできない)?;
        if 返事.trim_end() == 受け取った返事 {
            Ok(())
        } else {
            Err(渡せなかった理由::返事が違う)
        }
    }
}
