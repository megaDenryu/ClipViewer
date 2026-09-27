//! キャッシュの時計。使った時刻を書くことと、古い書きかけのファイルを見分けることに使う今の時刻を読む口を1つに閉じる。

use std::time::SystemTime;

/// キャッシュの時計とは、サムネイルのキャッシュが今の時刻を読む関数のことである。
/// 配線が実時間の時計を渡し、試験は進み方を決めた時計を渡す(使った時刻の古い順に消すことを確かめるため)。
#[derive(Debug, Clone, Copy)]
pub struct キャッシュの時計(fn() -> SystemTime);

impl キャッシュの時計 {
    /// 計算機の実時間を読む時計。
    pub fn 実時間() -> Self {
        Self(SystemTime::now)
    }

    /// 今の時刻を返す関数から作る。
    pub fn 関数から作る(今を読む: fn() -> SystemTime) -> Self {
        Self(今を読む)
    }

    /// 今の時刻。
    pub(crate) fn 今(self) -> SystemTime {
        (self.0)()
    }
}
