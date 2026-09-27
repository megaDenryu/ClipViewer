//! 依頼の台帳。先読みの並びと自分の行を比べて、捨てる区間・依頼する区間・渡し直す保持の優先順を決める。
//! 倉庫への呼び出しは持たない純粋な判断であり、呼び出しは `読み込んだ動画` が行う。参照: _doc/設計/画面.md 判断4

use std::time::{Duration, Instant};

use clip_domain::動画上の区間;

use super::ledger_state::行の状態;

/// 区間の集まりが変わってから依頼を出すまで待つ時間。区間の数値を続けて変えている間に依頼を出し続けないためである。
/// 区間の帯のドラッグの間はクリップスタックを変えないため、区間の集まりも変わらない。
pub(crate) const 落ち着くまでの時間: Duration = Duration::from_millis(400);

/// 台帳の行とは、1つの区間と、その区間の依頼の状態の組のことである。
#[derive(Debug, Clone)]
pub(crate) struct 台帳の行<札> {
    pub(crate) 区間: 動画上の区間,
    pub(crate) 状態: 行の状態<札>,
}

/// 台帳の手順とは、台帳が決めた、倉庫へ行う呼び出しの一覧のことである。この順(優先順 → 捨てる → 依頼する)に行う。
#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct 台帳の手順 {
    pub(crate) 渡す優先順: Option<Vec<動画上の区間>>,
    pub(crate) 捨てる区間: Vec<動画上の区間>,
    pub(crate) 依頼する区間: Vec<動画上の区間>,
}

/// 依頼の台帳とは、倉庫へ出した依頼の区間ごとの状態と、最後に倉庫へ渡した並びと、区間の集まりが最後に変わった時刻と、
/// 落ち着くのを待って依頼を出していない区間があるかの組のことである。
#[derive(Debug, Clone)]
pub(crate) struct 依頼の台帳<札> {
    pub(crate) 行: Vec<台帳の行<札>>,
    渡した並び: Vec<動画上の区間>,
    集まりが変わった時刻: Option<Instant>,
    pub(super) 依頼を待っている: bool,
}

impl<札> 依頼の台帳<札> {
    pub(crate) fn 空() -> Self {
        Self {
            行: Vec::new(),
            渡した並び: Vec::new(),
            集まりが変わった時刻: None,
            依頼を待っている: false,
        }
    }

    /// 並びと比べて手順を決め、行を手順に合わせて更新する(捨てる区間の行を除き、依頼し直す行を除く)。
    pub(crate) fn 手順を決める(
        &mut self,
        並び: &[動画上の区間],
        今: Instant,
    ) -> 台帳の手順 {
        let 捨てる区間 = self.並びに無い依頼中の行を外す(並び);
        if !同じ集まりか(並び, &self.渡した並び) {
            self.集まりが変わった時刻 = Some(今);
        }
        let mut 渡す優先順 = None;
        if 並び != self.渡した並び.as_slice() {
            self.行.retain(|行| !行.状態.順が変わったら依頼し直すか());
            self.渡した並び = 並び.to_vec();
            渡す優先順 = Some(並び.to_vec());
        }
        let 落ち着いたか = self
            .集まりが変わった時刻
            .is_none_or(|時刻| 今.saturating_duration_since(時刻) >= 落ち着くまでの時間);
        let 行の無い区間: Vec<動画上の区間> = 並び
            .iter()
            .filter(|区間| self.行の位置(区間).is_none())
            .copied()
            .collect();
        self.依頼を待っている = !落ち着いたか && !行の無い区間.is_empty();
        let 依頼する区間 = if 落ち着いたか {
            行の無い区間
        } else {
            Vec::new()
        };
        台帳の手順 {
            渡す優先順,
            捨てる区間,
            依頼する区間,
        }
    }

    fn 並びに無い依頼中の行を外す(
        &mut self,
        並び: &[動画上の区間],
    ) -> Vec<動画上の区間> {
        let (残す, 外す): (Vec<_>, Vec<_>) = std::mem::take(&mut self.行)
            .into_iter()
            .partition(|行| 並び.contains(&行.区間));
        self.行 = 残す;
        外す
            .into_iter()
            .filter(|行| matches!(行.状態, 行の状態::依頼中 { .. }))
            .map(|行| 行.区間)
            .collect()
    }
}

/// 2つの並びが、順を無視して同じ区間の集まりか。並びは同じ区間を1つにまとめてあるため、長さと包含で比べる。
fn 同じ集まりか(左: &[動画上の区間], 右: &[動画上の区間]) -> bool {
    左.len() == 右.len() && 左.iter().all(|区間| 右.contains(区間))
}
