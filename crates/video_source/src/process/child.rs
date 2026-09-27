//! 起動した子プロセス。ffmpeg と ffprobe を出力をパイプで受けて起動し、止めて終わりを待つ後始末をこの型の中に閉じる。

use std::io;
use std::process::{Child, ChildStderr, ChildStdout, Command, ExitStatus, Stdio};

/// 起動した子プロセスとは、出力をパイプで受けて起動した外部プロセス(ffmpeg・ffprobe)の取っ手のことである。
/// 落とすと止めて終わりを待つため、読む仕組みを作れずに途中で返る経路でも子プロセスを置き去りにしない。
pub(crate) struct 起動した子プロセス(Child);

/// 出力のパイプとは、起動した子プロセスの標準出力と標準エラー出力の組のことである。
pub(crate) struct 出力のパイプ {
    pub(crate) 標準出力: ChildStdout,
    pub(crate) 標準エラー: ChildStderr,
}

impl 起動した子プロセス {
    /// 命令を起動する。標準入力は閉じ、標準出力と標準エラー出力はパイプで受け取る。
    pub(crate) fn 起動する(命令: &mut Command) -> io::Result<(Self, 出力のパイプ)> {
        let mut 子プロセス = Self(
            命令
                .stdin(Stdio::null())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()?,
        );
        let 標準出力 = 子プロセス.0.stdout.take();
        let 標準エラー = 子プロセス.0.stderr.take();
        match 標準出力.zip(標準エラー) {
            Some((標準出力, 標準エラー)) => Ok((
                子プロセス,
                出力のパイプ {
                    標準出力,
                    標準エラー,
                },
            )),
            None => {
                子プロセス.止めて終わりを待つ();
                Err(io::Error::other("子プロセスの出力のパイプを取れない"))
            }
        }
    }

    /// 止める。終わりは待たない。既に終わっていても失敗にしない。
    pub(crate) fn 止める(&mut self) {
        // 既に終わった子プロセスを止めると失敗が返るが、止まっていることに変わりはないため無視する。
        let _ = self.0.kill();
    }

    /// 止めて、終わりを待つ。既に終わっていても失敗にしない。
    pub(crate) fn 止めて終わりを待つ(&mut self) {
        self.止める();
        // 止めた後の終わりを確かめられなくても、これ以上できることは無いため無視する。
        let _ = self.0.wait();
    }

    /// 終わるまで待ち、終了状態を受け取る。既に終わっていれば、その終了状態をすぐに返す。
    pub(crate) fn 終わりを待つ(&mut self) -> io::Result<ExitStatus> {
        self.0.wait()
    }

    /// 終わっていれば終了状態を返す。待たずに返る。
    pub(crate) fn 終わったかを確かめる(&mut self) -> io::Result<Option<ExitStatus>> {
        self.0.try_wait()
    }
}

impl Drop for 起動した子プロセス {
    fn drop(&mut self) {
        self.止めて終わりを待つ();
    }
}
