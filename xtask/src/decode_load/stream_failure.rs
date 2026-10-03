//! 1本の ffmpeg の読み出しが成り立たなかった理由と、同時に読んだ全部の結果のまとめ方。

use super::byte_count::バイト数;

/// 一本の読み出しの失敗とは、1本の ffmpeg の読み出しが、自分の失敗で終わったか、ほかの1本の失敗を受けて止めたかの区別のことである。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum 一本の読み出しの失敗 {
    /// この ffmpeg の起動・読み取り・終了のどれかが失敗した。持つのは理由である。
    失敗した(String),
    /// ほかの ffmpeg が失敗したため、読み終える前に止めた。
    ほかの失敗を受けて止めた,
}

impl 一本の読み出しの失敗 {
    /// 同時に読んだ全部の結果をまとめる。どれかが失敗していれば、止めたものより自分で失敗したものの理由を返す。
    pub fn 結果の並びをまとめる(
        結果の並び: Vec<Result<バイト数, Self>>,
    ) -> Result<Vec<バイト数>, String> {
        let mut 止めたものがあるか = false;
        let mut バイト数の並び = Vec::with_capacity(結果の並び.len());
        for 結果 in 結果の並び {
            match 結果 {
                Ok(バイト数) => バイト数の並び.push(バイト数),
                Err(Self::失敗した(理由)) => return Err(理由),
                Err(Self::ほかの失敗を受けて止めた) => 止めたものがあるか = true,
            }
        }
        if 止めたものがあるか {
            return Err("ffmpeg を止めたが、止めるきっかけになった失敗が見つからない".to_owned());
        }
        Ok(バイト数の並び)
    }
}
