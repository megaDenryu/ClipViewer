//! 行の並びと見えた回。何を先に撮るか・どのテクスチャを残すかを決める材料として、状態が毎フレーム渡す値。

use std::fmt;
use std::hash::Hash;

/// 一覧の行の識別子とは、一覧のサムネイルが一覧の行を指すのに使う識別子(スタックの識別子か重ね合わせの識別子)が満たす性質のことである。
/// 項目の表の鍵にし(比べて写せる)、テクスチャの名前に書き(文字にできる)、裏のスレッドへ仕事と一緒に渡す(スレッドをまたげる)。
pub(crate) trait 一覧の行の識別子:
    Clone + Ord + Hash + fmt::Display + Send + 'static
{
}

impl<識別子: Clone + Ord + Hash + fmt::Display + Send + 'static> 一覧の行の識別子 for 識別子 {}

/// 行の並びとは、一覧で今見えている行の識別子と、見せる順に並べた全部の行の識別子の組のことである。
/// 見えている行を先に撮り、残りは見せる順に撮るために使う。状態が並べた一覧から作り、識別子は借りる。
pub(crate) struct 行の並び<'一覧, 識別子> {
    pub(crate) 見えている: Vec<&'一覧 識別子>,
    pub(crate) 見せる順: Vec<&'一覧 識別子>,
}

impl<識別子> 行の並び<'_, 識別子> {
    /// 見えている行を先に、続けて見せる順の全部の行を並べる。見えている行は2回出るが、2回目には仕事が済んでいるため害は無い。
    pub(super) fn 見えている行を先にした全部の行(
        &self,
    ) -> impl Iterator<Item = &識別子> {
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
