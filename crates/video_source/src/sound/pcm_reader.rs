//! ffmpeg の標準出力から f32le・2チャンネルの PCM を、左右の値の並びとして読む。

use std::io::{self, Read};

use audio_pcm::{左右の値, 左右の値のバイト数};

/// 1回に読むバイト数。流し読みで音声のスレッドへ届くまでの遅れを小さく保つため、大きくしすぎない。
const 一回に読むバイト数: usize = 16 * 1024;

/// 1標本(左右の f32 の組)のバイト数。
const 一標本のバイト数: usize = 左右の値のバイト数;

/// 1回の読み取りで書き先へ足す標本の数の上限。持ち越した端数のバイトと合わせても1標本しか増えない。
pub(crate) const 一回に足す標本数の上限: usize = 一回に読むバイト数 / 一標本のバイト数 + 1;

/// 読んだ結果とは、出力から標本を読んだか、出力が閉じたかの区別のことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum 読んだ結果 {
    読んだ,
    終わった,
}

/// 左右の値の読み手とは、ffmpeg の出力を読み、1標本の途中で切れたバイトを次の読み取りまで持ち越すもののことである。
pub(crate) struct 左右の値の読み手<入力: Read> {
    入力: 入力,
    溜め: Vec<u8>,
}

impl<入力: Read> 左右の値の読み手<入力> {
    pub(crate) fn 作成する(入力: 入力) -> Self {
        Self {
            入力,
            溜め: Vec::with_capacity(一回に読むバイト数 + 一標本のバイト数),
        }
    }

    /// 出力を1回読み、そろった標本を書き先へ足す。出力が閉じたら終わったと返す。途中で切れた最後の標本は捨てる。
    pub(crate) fn 次の塊を読む(
        &mut self,
        書き先: &mut Vec<左右の値>,
    ) -> io::Result<読んだ結果> {
        let 前の長さ = self.溜め.len();
        self.溜め.resize(前の長さ + 一回に読むバイト数, 0);
        let 読んだ長さ = loop {
            match self.入力.read(&mut self.溜め[前の長さ..]) {
                Ok(長さ) => break 長さ,
                Err(原因) if 原因.kind() == io::ErrorKind::Interrupted => continue,
                Err(原因) => return Err(原因),
            }
        };
        self.溜め.truncate(前の長さ + 読んだ長さ);
        if 読んだ長さ == 0 {
            return Ok(読んだ結果::終わった);
        }
        let そろった長さ = self.溜め.len() - self.溜め.len() % 一標本のバイト数;
        書き先.extend(
            self.溜め[..そろった長さ]
                .as_chunks::<一標本のバイト数>()
                .0
                .iter()
                .map(バイトから左右の値を作る),
        );
        self.溜め.drain(..そろった長さ);
        Ok(読んだ結果::読んだ)
    }
}

fn バイトから左右の値を作る(
    一標本: &[u8; 一標本のバイト数]
) -> 左右の値 {
    let [左0, 左1, 左2, 左3, 右0, 右1, 右2, 右3] = *一標本;
    [
        f32::from_le_bytes([左0, 左1, 左2, 左3]),
        f32::from_le_bytes([右0, 右1, 右2, 右3]),
    ]
}
