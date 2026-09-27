//! 区間の帯の語彙。カードの区間の帯のどれを掴んだか、全体の帯か拡大の帯か、帯に置いた区間と位置、放したときに決めるものを表す。
//! 参照: _doc/設計/画面.md 判断11

use clip_domain::{動画上の区間, 動画上の秒};

/// 区間の帯の種類とは、カードの2本の区間の帯のどちらかの区別のことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum 区間の帯の種類 {
    全体の帯,
    拡大の帯,
}

/// 区間の帯で掴んだものとは、カードの区間の帯でドラッグを始めた位置が何に当たったかの区別のことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum 区間の帯で掴んだもの {
    開始のつまみ,
    終了のつまみ,
    区間の内側,
    位置の印,
}

/// 区間の帯のドラッグの結末とは、帯を放したときに状態へ決めるものの区別のことである。
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum 区間の帯のドラッグの結末 {
    区間を決める(動画上の区間),
    位置を決める(動画上の秒),
    何も決めない,
}

/// 区間の帯に置いた値とは、区間の帯に描く区間と位置の印の位置の組のことである。
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct 区間の帯に置いた値 {
    pub(crate) 区間: 動画上の区間,
    pub(crate) 位置: 動画上の秒,
}
