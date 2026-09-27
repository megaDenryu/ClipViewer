//! 行の並びと見えた回。何を先に撮るか・どのテクスチャを残すかを決める材料として、状態が毎フレーム渡す値。

use clip_domain::スタックの識別子;

/// 行の並びとは、一覧で今見えている行のスタックの識別子と、見せる順に並べた全部の行のスタックの識別子の組のことである。
/// 見えている行を先に撮り、残りは見せる順に撮るために使う。状態が並べた一覧から作り、識別子は借りる。
pub(crate) struct 行の並び<'一覧> {
    pub(crate) 見えている: Vec<&'一覧 スタックの識別子>,
    pub(crate) 見せる順: Vec<&'一覧 スタックの識別子>,
}

impl 行の並び<'_> {
    /// 見えている行を先に、続けて見せる順の全部の行を並べる。見えている行は2回出るが、2回目には仕事が済んでいるため害は無い。
    pub(super) fn 見えている行から順に(
        &self,
    ) -> impl Iterator<Item = &スタックの識別子> {
        self.見えている.iter().chain(self.見せる順.iter()).copied()
    }
}

/// 見えた回とは、一覧のサムネイルが仕事を進めた回を数えた番号のことである。テクスチャが最後に見えた回を比べ、
/// 見えなくなってから長いものを先に捨てる。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub(crate) struct 見えた回(u64);

impl 見えた回 {
    /// 次の回。
    pub(super) fn 次(self) -> Self {
        Self(self.0.saturating_add(1))
    }
}
