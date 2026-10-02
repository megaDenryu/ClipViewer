//! 見る側の設定の値。ウインドウの大きさと最大化・音量・速度・左右の向き・アスペクト比・表示サイズ・キーの割り当て(`keys/`)のように、スタックに属さず見る人の好みで決まり、
//! 次に起動したときへ持ち越す値の型を持つ。状態(`state`)と settings.json(`persistence`)の両方が使うため、どちらにも属さない層に置く。
//! 参照: _doc/設計/画面.md 判断17、_doc/設計/ライブラリ.md「settings.json の形式」

mod keys;
mod output_look;
mod speed_range;
mod window_size;

#[cfg(test)]
mod speed_range_tests;

pub(crate) use keys::{
    キーで行う操作, キーの割り当て, キーを変えられない理由, 操作のキー
};
pub(crate) use output_look::{左右の向き, 表示サイズ};
pub(crate) use speed_range::{
    つまみの範囲へ収めた速度, 速度のつまみの刻み, 速度のつまみの範囲
};
pub(crate) use window_size::{
    ウインドウの大きさ, ウインドウの記憶, 最大化の様子, 画面の大きさ
};

use audio_pcm::音量;
use clip_domain::{アスペクト比設定, 再生速度};

/// 見る側の設定とは、次に起動したときへ持ち越す、見る人の好みの値の組のことである。
/// 全体ループと再生モードと画面の構えと消音は含めない(理由は 画面.md 判断17)。どの組み合わせも成立するため、不変条件は無い。
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct 見る側の設定 {
    pub(crate) ウインドウ: ウインドウの記憶,
    pub(crate) 音量: 音量,
    pub(crate) 速度: 再生速度,
    pub(crate) 左右: 左右の向き,
    pub(crate) アスペクト比: アスペクト比設定,
    pub(crate) 表示サイズ: 表示サイズ,
    pub(crate) キー: キーの割り当て,
}

impl 見る側の設定 {
    /// 何も覚えていないときの設定。アプリを初めて起動したときと同じ値である。
    pub(crate) const 既定: Self = Self {
        ウインドウ: ウインドウの記憶::最初,
        音量: 音量::起動したときの音量,
        速度: 再生速度::等倍,
        左右: 左右の向き::そのまま,
        アスペクト比: アスペクト比設定::クロップ連動,
        表示サイズ: 表示サイズ::大きめ,
        キー: キーの割り当て::既定(),
    };
}
