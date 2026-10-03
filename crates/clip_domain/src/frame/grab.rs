//! 枠を掴む所の語彙。ドラッグで枠の内側を掴んだか、四隅のつまみのどれかを掴んだかを表す。クロップ枠と映す矩形の枠が共に使う。

/// 四隅のつまみとは、枠の大きさを変えるためにドラッグする角の区別のことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum 四隅のつまみ {
    /// 左上の角。
    左上,
    /// 右上の角。
    右上,
    /// 左下の角。
    左下,
    /// 右下の角。
    右下,
}

impl 四隅のつまみ {
    pub(crate) fn 左側か(self) -> bool {
        matches!(self, Self::左上 | Self::左下)
    }

    pub(crate) fn 上側か(self) -> bool {
        matches!(self, Self::左上 | Self::右上)
    }
}

/// 枠の掴む所とは、ドラッグで枠の内側を掴んだ(枠を動かす)か、四隅のつまみのどれかを掴んだ(大きさを変える)かの区別のことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum 枠の掴む所 {
    /// 枠の内側。
    内側,
    /// 四隅のつまみのどれか。
    隅(四隅のつまみ),
}
