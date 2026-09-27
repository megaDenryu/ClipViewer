//! スタックの名前。

use std::fmt;

/// スタックの名前とは、利用者がライブラリのスタックに付けた表示用の名前であり、前後の空白を除いて空でない文字列のことである。
/// 名前はライブラリの中で重複してよい。スタックを指すのは識別子である。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct スタックの名前(String);

/// 空のスタックの名前エラーとは、前後の空白を除くと名前が空になることである。
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("スタックの名前が空である")]
pub struct 空のスタックの名前エラー;

/// 複製したスタックの名前に足す言葉。
const 複製の印: &str = "のコピー";

impl スタックの名前 {
    /// 入力された文字列から、前後の空白を除いて作成する。空なら拒む。
    pub fn 作成する(名前: String) -> Result<Self, 空のスタックの名前エラー> {
        let 除いた = 名前.trim();
        if 除いた.is_empty() {
            return Err(空のスタックの名前エラー);
        }
        Ok(Self(除いた.to_string()))
    }

    /// 複製したスタックに付ける、名前の後ろに「のコピー」を足した名前。
    pub fn 複製の名前(&self) -> Self {
        Self(format!("{}{複製の印}", self.0))
    }

    /// 名前の文字列を返す。表示やファイルの本文へ渡す境界で使う。
    pub fn 文字列(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for スタックの名前 {
    fn fmt(&self, 書き込み先: &mut fmt::Formatter<'_>) -> fmt::Result {
        書き込み先.write_str(&self.0)
    }
}
