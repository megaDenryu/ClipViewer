//! 配色の定数。値は移植元の style.css.ts と View/ヘッダー/style.css.ts の色をそのまま写す。
//! 移植元が半透明で重ねていた色は、下地の #020617 と混ぜた結果の不透明な色にしてある。

use sengen_egui::色;

pub(super) const 琥珀: 色 = 色::from_rgb(245, 158, 11); // #f59e0b
pub(super) const 濃い琥珀: 色 = 色::from_rgb(217, 119, 6); // #d97706
pub(super) const 最も濃い地: 色 = 色::from_rgb(2, 6, 23); // #020617。出力の欄と入力欄の地
pub(super) const 札の地: 色 = 色::from_rgb(15, 23, 42); // #0f172a。ヘッダー・再生コントロール・カードの地
pub(super) const 左ペインの地: 色 = 色::from_rgb(11, 14, 20); // #0b0e14
pub(super) const 出力の帯の地: 色 = 色::from_rgb(10, 16, 34); // rgba(15,23,42,0.6) を #020617 に重ねた色
pub(super) const 線と部品の面の色: 色 = 色::from_rgb(30, 41, 59); // #1e293b
pub(super) const 文字の色: 色 = 色::from_rgb(226, 232, 240); // #e2e8f0
pub(super) const 白に近い文字の色: 色 = 色::from_rgb(248, 250, 252); // #f8fafc
pub(super) const 淡い文字の色: 色 = 色::from_rgb(148, 163, 184); // #94a3b8
pub(super) const 淡い赤: 色 = 色::from_rgb(253, 164, 175); // #fda4af
pub(super) const 空色: 色 = 色::from_rgb(56, 189, 248); // #38bdf8。区間の帯の位置の印
pub(super) const 黒: 色 = 色::BLACK;
pub(super) const 半透明の黒: 色 = 色::from_black_alpha(190); // シアターの操作の欄の地。移植元の黒の濃淡(下端0.88〜上端0)の中ほどの濃さ
pub(super) const トリガー待ちの地: 色 = 色::from_rgba_premultiplied(2, 6, 23, 220);
