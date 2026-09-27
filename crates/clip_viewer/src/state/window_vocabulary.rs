//! ウインドウの語彙。起動の部分が毎フレーム知らせるウインドウの様子(全画面の様子・ウインドウの形・画面の大きさ)と、状態がウインドウへ頼む全画面の出入りと大きさの変更の型を持つ。
//! ウインドウの状態(`window.rs`)がこれらを受け取り、配線(`app/close.rs`)が頼みをウインドウへの指示にする。参照: _doc/設計/画面.md 判断16・判断17

use crate::viewer_settings::{ウインドウの大きさ, 画面の大きさ};

/// 全画面の様子とは、OS のウインドウが画面全体を覆う全画面か、そうでないかの区別のことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum 全画面の様子 {
    #[default]
    全画面でない,
    全画面,
}

impl 全画面の様子 {
    pub(super) fn 逆(self) -> Self {
        match self {
            Self::全画面でない => Self::全画面,
            Self::全画面 => Self::全画面でない,
        }
    }
}

/// ウインドウの形とは、全画面でないときのウインドウが、最小化・最大化・普通の大きさ(大きさを egui がまだ知らせていなければ無い)のどれかの区別のことである。
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum ウインドウの形 {
    最小化している,
    最大化している,
    普通(Option<ウインドウの大きさ>),
}

/// ウインドウの様子とは、起動の部分が毎フレーム egui から読んで知らせる、全画面の様子とウインドウの形と画面の大きさ(まだ知らせていなければ無い)の組のことである。
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct ウインドウの様子 {
    pub(crate) 全画面: 全画面の様子,
    pub(crate) 形: ウインドウの形,
    pub(crate) 画面: Option<画面の大きさ>,
}

/// ウインドウへの頼みとは、状態が決めて、まだウインドウへ送っていない全画面の出入りと大きさの変更の組のことである。
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub(crate) struct ウインドウへの頼み {
    pub(crate) 全画面: Option<全画面の様子>,
    pub(crate) 大きさ: Option<ウインドウの大きさ>,
}
