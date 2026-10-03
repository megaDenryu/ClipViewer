//! 文字と部品の装飾。見出し・補足・ボタン・クリップカード・トリガー待ちの札・区間の帯に付ける名前付きのスタイルである。

use sengen_egui::{スタイル, 余白画素, 画素, 角丸画素};

use super::palette;

pub(crate) const 見出し: スタイル = スタイル {
    太字: Some(true),
    文字色: Some(palette::淡い文字の色),
    ..スタイル::無指定
};

pub(crate) const 補足: スタイル = スタイル {
    弱い: Some(true),
    ..スタイル::無指定
};

pub(crate) const 不備の文: スタイル = スタイル {
    文字色: Some(palette::淡い赤),
    ..スタイル::無指定
};

pub(crate) const 動画の名前: スタイル = スタイル {
    文字色: Some(palette::琥珀),
    太字: Some(true),
    ..スタイル::無指定
};

pub(crate) const 強調ボタン: スタイル = スタイル {
    背景色: Some(palette::濃い琥珀),
    文字色: Some(palette::最も濃い地),
    ..スタイル::無指定
};

pub(crate) const 危険ボタン: スタイル = スタイル {
    文字色: Some(palette::淡い赤),
    ..スタイル::無指定
};

pub(crate) const 札: スタイル = スタイル {
    背景色: Some(palette::札の地),
    内余白: Some(余白画素(8)),
    角丸: Some(角丸画素(6)),
    枠線色: Some(palette::線と部品の面の色),
    ..スタイル::無指定
};

pub(crate) const 選ばれた札: スタイル = スタイル {
    枠線色: Some(palette::琥珀),
    枠線太さ: Some(画素(2.0)),
    ..札
};

pub(crate) const 警告の札: スタイル = スタイル {
    枠線色: Some(palette::琥珀),
    ..札
};

pub(crate) const トリガー待ちの札: スタイル = スタイル {
    背景色: Some(palette::トリガー待ちの地),
    内余白: Some(余白画素(10)),
    角丸: Some(角丸画素(6)),
    枠線色: Some(palette::琥珀),
    ..スタイル::無指定
};

/// 区間の帯。地は最も濃い地、つまみは白に近い色、位置の印は空色にする。区間の塗りはテーマの強調色(琥珀色)であり、
/// その上でつまみと位置の印を見分けられる色を選ぶ。
pub(crate) const 区間の帯: スタイル = スタイル {
    背景色: Some(palette::最も濃い地),
    枠線色: Some(palette::白に近い文字の色),
    文字色: Some(palette::空色),
    ..スタイル::無指定
};
