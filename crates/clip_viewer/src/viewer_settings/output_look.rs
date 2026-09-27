//! 出力の見せ方の値。表示サイズと左右の向きを持つ。出力の設定(`state/output_settings.rs`)と settings.json の両方が使う。

/// 表示サイズとは、出力の欄の中で出力をどの大きさに描くかの区別のことである。移植元の選択肢と同じ4つである。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum 表示サイズ {
    標準,
    #[default]
    大きめ,
    横幅優先,
    余白なし,
}

/// 左右の向きとは、出力の画面(編集の構えの出力とシアター)に映像をそのまま映すか、左右を入れ替えて鏡のように映すかの区別のことである。
/// 見る側の設定であり、クリップの項目ではない。プレビューのクロップ枠は元の動画の向きのまま描く(画面.md 判断15)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum 左右の向き {
    #[default]
    そのまま,
    反転,
}
