//! 起動の頼み。起動の引数(エクスプローラーが渡す動画のパス)を読み、1つ目のアプリへ渡す頼みにする。

use std::ffi::OsString;

use clip_domain::入力された動画パス;

/// 開かなかった動画の数とは、起動の引数に複数の動画が渡されたときに、最初の1つのほかに開かずに捨てた動画の数のことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct 開かなかった動画の数(usize);

impl 開かなかった動画の数 {
    pub(crate) fn 作成する(数: usize) -> Self {
        Self(数)
    }

    /// 1つもないか。
    pub(crate) fn 無いか(self) -> bool {
        self.0 == 0
    }

    /// 数そのもの。文面と受け渡しの本文へ書く境界で使う。
    pub(crate) fn 数(self) -> usize {
        self.0
    }
}

/// 起動の頼みとは、起動したアプリが自分へ、または2つ目のアプリが1つ目のアプリへ頼むことの区別のことである。
/// 動画を開く頼みは、起動の引数の最初の動画のパスと、開かなかった残りの動画の数を持つ。
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum 起動の頼み {
    /// 開く動画が無い。1つ目のアプリへは、ウインドウを前に出すことだけを頼む。
    前に出る,
    動画を開く {
        動画: 入力された動画パス,
        開かなかった数: 開かなかった動画の数,
    },
}

impl 起動の頼み {
    /// 起動の引数(実行ファイルの名前を除いたもの)から読む。複数あれば最初の1つだけを開き、残りの数を覚える。
    /// 引数はすべて動画のパスとして扱う。UTF-8 でない文字は置き換え文字にする(ウインドウへ落とした動画と同じ扱い)。
    pub(crate) fn 引数から読む(引数: impl IntoIterator<Item = OsString>) -> Self {
        let mut 残り = 引数.into_iter();
        match 残り.next() {
            None => Self::前に出る,
            Some(最初) => Self::動画を開く {
                動画: 入力された動画パス::作成する(
                    最初.to_string_lossy().into_owned(),
                ),
                開かなかった数: 開かなかった動画の数::作成する(残り.count()),
            },
        }
    }
}
