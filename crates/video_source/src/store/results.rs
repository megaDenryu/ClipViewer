//! 倉庫の項目から外れた依頼の結末と、その記録。

use std::collections::VecDeque;

use crate::process::デコードの失敗;

use super::outcome::依頼の状況;
use super::ticket::受付の札;

/// 覚えておく結末の数。これより古い札を問い合わせると、知らない札として答える。
/// 画面が札を問い合わせ続けても記録が際限なく増えないように上限を置く。
const 覚えておく結末の数: usize = 1024;

/// 終わった依頼の結末とは、倉庫の項目から外れた依頼が、なぜ外れたかの区別のことである。
#[derive(Debug)]
pub(crate) enum 終わった依頼の結末 {
    失敗(デコードの失敗),
    取り消した,
    捨てた,
}

/// 結末の記録とは、倉庫の項目から外れた依頼の札と結末を、新しいものから `覚えておく結末の数` まで覚えたもののことである。
#[derive(Debug, Default)]
pub(crate) struct 結末の記録(VecDeque<記録した結末>);

#[derive(Debug)]
struct 記録した結末 {
    札: 受付の札,
    結末: 終わった依頼の結末,
}

impl 結末の記録 {
    pub(crate) fn 記録する(&mut self, 札: 受付の札, 結末: 終わった依頼の結末) {
        self.0.push_back(記録した結末 { 札, 結末 });
        if self.0.len() > 覚えておく結末の数 {
            self.0.pop_front();
        }
    }

    pub(crate) fn 状況(&self, 札: 受付の札) -> 依頼の状況<'_> {
        let 見つけた = self.0.iter().rev().find(|記録| 記録.札 == 札);
        見つけた.map_or(依頼の状況::知らない札, |記録| 記録.結末.状況())
    }
}

impl 終わった依頼の結末 {
    fn 状況(&self) -> 依頼の状況<'_> {
        match self {
            Self::失敗(失敗) => 依頼の状況::失敗(失敗),
            Self::取り消した => 依頼の状況::取り消した,
            Self::捨てた => 依頼の状況::捨てた,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::ticket::札の発行元;

    #[test]
    fn 結末の記録は上限を超えると古いものから忘れる() {
        let mut 発行元 = 札の発行元::default();
        let mut 記録 = 結末の記録::default();
        let 最初の札 = 発行元.発行する();
        記録.記録する(最初の札, 終わった依頼の結末::捨てた);
        for _ in 0..1024 {
            記録.記録する(発行元.発行する(), 終わった依頼の結末::取り消した);
        }
        assert!(matches!(記録.状況(最初の札), 依頼の状況::知らない札));
    }
}
