//! 音量の型。

/// 音量とは、0以上1以下の音の大きさのことである。0で無音、1で元の大きさである。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct 音量(f64);

impl 音量 {
    /// 起動したときの音量(移植元と同じ0.5)。
    pub const 既定: 音量 = 音量(0.5);

    /// 無音。
    pub const 無音: 音量 = 音量(0.0);

    /// 0未満は0に、1を超える値は1に、非数は0に収めて作る。つまみの値を受け取る境界で使う。
    pub fn 範囲へ収めて作る(値: f64) -> Self {
        if 値.is_nan() {
            return Self(0.0);
        }
        Self(値.clamp(0.0, 1.0))
    }

    /// 0以上1以下の値。つまみへ渡す境界と、標本に掛ける倍率に使う。
    pub fn 値(self) -> f64 {
        self.0
    }
}
