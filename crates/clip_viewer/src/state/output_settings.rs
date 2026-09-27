//! 出力の設定。出力のアスペクト比・表示サイズ・左右の向き・画面の構え(編集かシアターか)を持つ。
//! 表示サイズと左右の向きの型は、settings.json も使うため `viewer_settings` に置く。

use clip_domain::アスペクト比設定;

use crate::viewer_settings::{左右の向き, 表示サイズ};

/// 画面の構えとは、編集の部品を出すか、隠して出力を広げるか(シアター)、スタックのライブラリの一覧を出すかの区別のことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum 画面の構え {
    #[default]
    編集,
    シアター,
    ライブラリ,
}

/// 出力の設定とは、出力の縦横比の決め方と表示サイズと左右の向きと画面の構えの組のことである。
/// どの組み合わせも成立するため、不変条件は無い。
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct 出力の設定 {
    pub(crate) アスペクト比: アスペクト比設定,
    pub(crate) 表示サイズ: 表示サイズ,
    pub(crate) 左右: 左右の向き,
    pub(crate) 構え: 画面の構え,
}
