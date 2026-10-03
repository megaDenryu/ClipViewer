//! 重ね合わせの名前。

use std::fmt;

/// 重ね合わせの名前とは、利用者がライブラリの重ね合わせに付けた表示用の名前であり、前後の空白を除いて空でない文字列のことである。
/// 名前はライブラリの中で重複してよい。重ね合わせを指すのは識別子である。スタックの名前とは別の型である。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct 重ね合わせの名前(String);

/// 空の重ね合わせの名前エラーとは、前後の空白を除くと名前が空になることである。
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("重ね合わせの名前が空である")]
pub struct 空の重ね合わせの名前エラー;

impl 重ね合わせの名前 {
    /// 入力された文字列から、前後の空白を除いて作成する。空なら拒む。
    pub fn 作成する(名前: String) -> Result<Self, 空の重ね合わせの名前エラー> {
        let 除いた = 名前.trim();
        if 除いた.is_empty() {
            return Err(空の重ね合わせの名前エラー);
        }
        Ok(Self(除いた.to_string()))
    }

    /// 名前の文字列を返す。表示やファイルの本文へ渡す境界で使う。
    pub fn 文字列(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for 重ね合わせの名前 {
    fn fmt(&self, 書き込み先: &mut fmt::Formatter<'_>) -> fmt::Result {
        書き込み先.write_str(&self.0)
    }
}
