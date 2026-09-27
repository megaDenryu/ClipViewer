//! 保持の優先順と、そこから決まる区間の順位。

use std::cmp::Ordering;

use clip_domain::動画上の区間;

/// 保持の優先順とは、コマの倉庫に残したい区間の並びのことである。先頭ほど残したい。
/// 画面は、スタックや再生中のクリップが変わるたびに渡し直す。倉庫はクリップや再生の規則を知らず、この並びだけで捨てる順を決める。
#[derive(Debug, Clone, PartialEq, Default)]
pub struct 保持の優先順(Vec<動画上の区間>);

impl 保持の優先順 {
    /// 残したい順の区間の並びから作成する。
    pub fn 作成する(区間の並び: Vec<動画上の区間>) -> Self {
        Self(区間の並び)
    }

    /// 区間の順位を求める。並びに同じ区間が複数あれば、最も前の位置を使う。
    pub(crate) fn 順位(&self, 区間: &動画上の区間) -> 保持の順位 {
        self.0
            .iter()
            .position(|並びの区間| 並びの区間 == 区間)
            .map_or(保持の順位::優先順に無い, 保持の順位::並びの位置)
    }
}

impl 保持の優先順 {
    /// 区間の並びのうち、最も上位の区間の位置を求める。同じ順位なら前にあるものを選ぶ。並びが空なら `None` である。
    pub(crate) fn 最も優先する位置<'区間>(
        &self,
        区間の並び: impl Iterator<Item = &'区間 動画上の区間>,
    ) -> Option<usize> {
        let mut 最上位: Option<(usize, 保持の順位)> = None;
        for (位置, 区間) in 区間の並び.enumerate() {
            let 順位 = self.順位(区間);
            if 最上位.is_none_or(|(_, 最上位の順位)| 順位 > 最上位の順位) {
                最上位 = Some((位置, 順位));
            }
        }
        最上位.map(|(位置, _)| 位置)
    }
}

/// 保持の順位とは、保持の優先順の中での区間の位置のことである。大きいほど残したい。
/// 並びに無い区間は、並びのどの区間よりも小さい。並びの中では、前にあるほど大きい。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum 保持の順位 {
    優先順に無い,
    並びの位置(usize),
}

impl Ord for 保持の順位 {
    fn cmp(&self, 他方: &Self) -> Ordering {
        match (self, 他方) {
            (Self::優先順に無い, Self::優先順に無い) => Ordering::Equal,
            (Self::優先順に無い, Self::並びの位置(_)) => Ordering::Less,
            (Self::並びの位置(_), Self::優先順に無い) => Ordering::Greater,
            (Self::並びの位置(自分), Self::並びの位置(相手)) => 相手.cmp(自分),
        }
    }
}

impl PartialOrd for 保持の順位 {
    fn partial_cmp(&self, 他方: &Self) -> Option<Ordering> {
        Some(self.cmp(他方))
    }
}

#[cfg(test)]
mod tests {
    use clip_domain::時刻;

    use super::*;

    fn 区間(開始: f64) -> 動画上の区間 {
        let (Ok(開始), Ok(終了)) = (時刻::作成する(開始), 時刻::作成する(開始 + 1.0))
        else {
            panic!("時刻を作れない")
        };
        let Ok(区間) = 動画上の区間::作成する(開始, 終了) else {
            panic!("区間を作れない")
        };
        区間
    }

    #[test]
    fn 優先順の前にある区間ほど順位が高く_並びに無い区間が最も低い() {
        let 優先順 = 保持の優先順::作成する(vec![区間(0.0), 区間(5.0)]);
        assert!(優先順.順位(&区間(0.0)) > 優先順.順位(&区間(5.0)));
        assert!(優先順.順位(&区間(5.0)) > 優先順.順位(&区間(9.0)));
        assert_eq!(優先順.順位(&区間(9.0)), 保持の順位::優先順に無い);
    }

    #[test]
    fn 処理する依頼は優先順で最上位を選び_優先順に無いものは受付順で最後に回す() {
        let 優先順 = 保持の優先順::作成する(vec![区間(3.0), 区間(1.0)]);
        let 待ち = [区間(9.0), 区間(1.0), 区間(8.0), 区間(3.0)];
        assert_eq!(優先順.最も優先する位置(待ち.iter()), Some(3));
        assert_eq!(優先順.最も優先する位置(待ち[..3].iter()), Some(1));
        assert_eq!(
            優先順.最も優先する位置([区間(9.0), 区間(8.0)].iter()),
            Some(0)
        );
        assert_eq!(優先順.最も優先する位置([].iter()), None);
    }
}
