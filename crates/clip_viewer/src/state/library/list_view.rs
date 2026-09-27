//! 一覧の見せ方。名前での絞り込みと並べ替えを持ち、ライブラリの一覧から見せる順の並びを作る。読めないファイルは最後にまとめる。

use std::cmp::Ordering;

use clip_library::{ライブラリの一覧, 一覧の項目, 読めたスタック};

/// 並べ替え方とは、ライブラリの一覧の読めたスタックを並べる順の区別のことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum 並べ替え方 {
    #[default]
    更新日時の新しい順,
    更新日時の古い順,
    名前順,
    作った日時の新しい順,
}

impl 並べ替え方 {
    pub(crate) const 一覧: [Self; 4] = [
        Self::更新日時の新しい順,
        Self::更新日時の古い順,
        Self::名前順,
        Self::作った日時の新しい順,
    ];

    fn 比べる(self, 甲: &読めたスタック, 乙: &読めたスタック) -> Ordering {
        let (甲, 乙) = (&甲.スタック, &乙.スタック);
        match self {
            Self::更新日時の新しい順 => 乙.更新した日時().cmp(&甲.更新した日時()),
            Self::更新日時の古い順 => 甲.更新した日時().cmp(&乙.更新した日時()),
            Self::名前順 => 甲
                .名前()
                .文字列()
                .to_lowercase()
                .cmp(&乙.名前().文字列().to_lowercase()),
            Self::作った日時の新しい順 => 乙.作った日時().cmp(&甲.作った日時()),
        }
    }
}

/// 絞り込みの語とは、利用者が一覧の絞り込みの欄に書いた、名前に含まれていてほしい文字列のことである。空なら絞り込まない。
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct 絞り込みの語(String);

impl 絞り込みの語 {
    pub(crate) fn 作成する(文字列: String) -> Self {
        Self(文字列)
    }

    pub(crate) fn 文字列(&self) -> &str {
        &self.0
    }

    /// 大文字と小文字を区別せず、名前がこの語を含むか。語が空白だけなら、どの名前も合う。
    fn 合うか(&self, 名前: &str) -> bool {
        let 語 = self.0.trim().to_lowercase();
        語.is_empty() || 名前.to_lowercase().contains(&語)
    }
}

/// 一覧の見せ方とは、一覧の絞り込みの語と並べ替え方の組のことである。
#[derive(Debug, Clone, Default)]
pub(crate) struct 一覧の見せ方 {
    pub(crate) 絞り込み: 絞り込みの語,
    pub(crate) 並べ替え: 並べ替え方,
}

/// 見せる行とは、ライブラリの一覧の項目のうち見せるものを、一覧の中の位置で指したもののことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum 見せる行 {
    読めた(usize),
    読めない(usize),
}

impl 一覧の見せ方 {
    /// 一覧から見せる行の順を作る。読めないファイルは絞り込まず、フォルダを読んだ順のまま最後に出す。
    pub(crate) fn 見せる順に並べる(
        &self, 一覧: &ライブラリの一覧
    ) -> Vec<見せる行> {
        let mut 読めた: Vec<(usize, &読めたスタック)> = Vec::new();
        let mut 読めない = Vec::new();
        for (位置, 項目) in 一覧.項目.iter().enumerate() {
            match 項目 {
                一覧の項目::読めた(スタック)
                    if self.絞り込み.合うか(スタック.スタック.名前().文字列()) =>
                {
                    読めた.push((位置, スタック))
                }
                一覧の項目::読めた(_) => {}
                一覧の項目::読めない(_) => 読めない.push(見せる行::読めない(位置)),
            }
        }
        読めた.sort_by(|(_, 甲), (_, 乙)| self.並べ替え.比べる(甲, 乙));
        読めた
            .into_iter()
            .map(|(位置, _)| 見せる行::読めた(位置))
            .chain(読めない)
            .collect()
    }
}
