//! ffmpeg の標準出力から RGBA の生データを1コマずつ読む。

use std::io::{self, Read};
use std::process::ChildStdout;

use crate::frame::コマ;
use crate::frame_size::コマの寸法;

/// コマの読み手とは、ffmpeg の標準出力を、1コマ分(幅×高さ×4バイト)ずつ区切って読む仕組みのことである。
pub(crate) struct コマの読み手 {
    出力: ChildStdout,
    寸法: コマの寸法,
    一コマの長さ: usize,
}

impl コマの読み手 {
    /// 標準出力と、出力されるコマの寸法から作る。1コマのバイト数がこの計算機で扱えなければ失敗する。
    pub(crate) fn 作成する(出力: ChildStdout, 寸法: コマの寸法) -> io::Result<Self> {
        let 一コマの長さ = usize::try_from(寸法.一コマのバイト数().値())
            .map_err(|_| io::Error::other("1コマのバイト数がこの計算機で扱える大きさを超える"))?;
        Ok(Self {
            出力,
            寸法,
            一コマの長さ,
        })
    }

    /// 次の1コマを読む。出力がコマの区切りで閉じたなら `None` を返し、コマの途中で閉じたなら失敗を返す。
    pub(crate) fn 次のコマを読む(&mut self) -> io::Result<Option<コマ>> {
        let mut 画素 = vec![0_u8; self.一コマの長さ];
        let mut 読んだ長さ = 0;
        while 読んだ長さ < 画素.len() {
            match self.出力.read(&mut 画素[読んだ長さ..]) {
                Ok(0) => break,
                Ok(長さ) => 読んだ長さ += 長さ,
                Err(原因) if 原因.kind() == io::ErrorKind::Interrupted => continue,
                Err(原因) => return Err(原因),
            }
        }
        if 読んだ長さ == 0 {
            return Ok(None);
        }
        if 読んだ長さ < 画素.len() {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                format!(
                    "コマの途中で出力が閉じた({読んだ長さ}/{}バイト)",
                    画素.len()
                ),
            ));
        }
        Ok(Some(コマ::読み取った画素から作る(
            self.寸法,
            画素,
        )))
    }
}
