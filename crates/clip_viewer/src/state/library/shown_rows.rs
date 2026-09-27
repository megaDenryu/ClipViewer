//! 並べた一覧から行の並びを作る。見えている行の範囲と見せる行の順から、読めたスタックの識別子を並べ、サムネイルの仕事の順にする。

use clip_domain::スタックの識別子;
use clip_library::一覧の項目;

use super::list_view::見せる行;
use super::shown_list::{並べた一覧, 最後に読んだ一覧};
use super::visible_rows::見えている行の範囲;
use crate::thumbnail_feed::行の並び;

impl 並べた一覧 {
    /// 見えている行の範囲(無ければ見えている行は無い)と見せる行の順から、行の並びを作る。読めないファイルの行は含めない。
    pub(crate) fn 行の並び(
        &self, 見えている: Option<見えている行の範囲>
    ) -> 行の並び<'_> {
        let 見せる順 = self.見せる行の順を借りる();
        let 見えている行 = 見えている
            .and_then(|範囲| 見せる順.get(範囲.番号()))
            .unwrap_or_default();
        行の並び {
            見えている: 見えている行
                .iter()
                .filter_map(|行| self.読めた識別子(行))
                .collect(),
            見せる順: 見せる順
                .iter()
                .filter_map(|行| self.読めた識別子(行))
                .collect(),
        }
    }

    fn 読めた識別子(&self, 行: &見せる行) -> Option<&スタックの識別子> {
        let (見せる行::読めた(位置), 最後に読んだ一覧::読めた(一覧)) = (行, self.一覧())
        else {
            return None;
        };
        match 一覧.項目.get(*位置)? {
            一覧の項目::読めた(読めた) => Some(読めた.スタック.識別子()),
            一覧の項目::読めない(_) => None,
        }
    }
}
