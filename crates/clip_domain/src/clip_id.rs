//! クリップの識別子と、その発行の仕組み。

use std::fmt;

/// クリップ識別子とは、クリップスタックの中でクリップを一意に指す、空でない文字列のことである。
/// 新しく発行するときの表記は移植元と同じ「mod-<ミリ秒>-<乱数>」である。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct クリップ識別子(String);

/// 空の識別子エラーとは、識別子の文字列が空であることである。
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("クリップ識別子が空である")]
pub struct 空の識別子エラー;

impl クリップ識別子 {
    /// 設定ファイル等にある既存の識別子の文字列から作成する。空文字列を拒む。
    pub fn 文字列から作成する(
        文字列: String
    ) -> Result<Self, 空の識別子エラー> {
        if 文字列.is_empty() {
            Err(空の識別子エラー)
        } else {
            Ok(Self(文字列))
        }
    }

    /// 発行時刻と乱数から、移植元と同じ表記「mod-<ミリ秒>-<乱数>」の識別子を組み立てる。
    pub fn 時刻と乱数から組み立てる(
        時刻: 発行時刻, 乱数: 識別子の乱数
    ) -> Self {
        Self(format!("mod-{}-{}", 時刻.0, 乱数.0))
    }

    /// 識別子の文字列を返す。設定ファイルへ書き出す境界で使う。
    pub fn 文字列(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for クリップ識別子 {
    fn fmt(&self, 書き込み先: &mut fmt::Formatter<'_>) -> fmt::Result {
        書き込み先.write_str(&self.0)
    }
}

/// 発行時刻とは、識別子を発行した時刻を、1970年1月1日(協定世界時)からのミリ秒で表したもののことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct 発行時刻(pub(crate) u64);

impl 発行時刻 {
    /// 1970年1月1日(協定世界時)からのミリ秒から作成する。
    pub fn 紀元からのミリ秒で作成する(ミリ秒: u64) -> Self {
        Self(ミリ秒)
    }
}

/// 識別子の乱数とは、同じミリ秒に発行した識別子を区別するための0以上9999以下の数のことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct 識別子の乱数(pub(crate) u16);

impl 識別子の乱数 {
    /// 任意の乱数を10000で割った余りにして作成する。
    pub fn 乱数を畳んで作成する(乱数: u32) -> Self {
        Self(u16::try_from(乱数 % 10_000).unwrap_or(0))
    }
}

/// クリップ識別子の発行元とは、新しいクリップ識別子を1つずつ発行するもののことである。
/// 時刻と乱数の取得をドメインの外へ出し、試験では決まった識別子を発行できるようにするために置く。
pub trait クリップ識別子の発行元 {
    /// 新しい識別子を1つ発行する。
    fn 新しい識別子を発行する(&mut self) -> クリップ識別子;
}
