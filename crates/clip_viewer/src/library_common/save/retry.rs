//! 保存の頼み直し。保存に失敗した後に保存を頼み直すかと、頼み直す時刻を持つ。
//! 参照: _doc/設計/ライブラリ.md 判断5・判断11

use std::time::{Duration, Instant};

/// 保存に失敗してから頼み直すまでの間隔。書けない状態(ディスクが一杯・ほかのプロセスが開いている)の間に書き込みを繰り返し過ぎないためである。
const 頼み直すまでの間隔: Duration = Duration::from_secs(3);

/// 頼み直しとは、保存に失敗した後に保存を頼み直すかと、頼み直す時刻の区別のことである。
/// 知らせを受けたときには今の時刻を知らないため、時刻は次にフレームの時刻で見たときに決める。
#[derive(Debug, Clone, Copy, Default)]
pub(super) enum 頼み直し {
    #[default]
    無し,
    時刻を決める前,
    時刻(Instant),
}

impl 頼み直し {
    /// 今、頼み直すときか。失敗を受けて最初に見たときに、頼み直す時刻を決める。
    pub(super) fn 頼み直すときか(&mut self, 今: Instant) -> bool {
        match *self {
            Self::無し => false,
            Self::時刻を決める前 => {
                *self = Self::時刻(今 + 頼み直すまでの間隔);
                false
            }
            Self::時刻(時刻) => 今 >= 時刻,
        }
    }

    /// 頼み直すまでの残りの時間。頼み直さないなら無い。時刻を決める前なら、次に見たときに決めるため0である。
    pub(super) fn 頼み直すまでの時間(&self, 今: Instant) -> Option<Duration> {
        match *self {
            Self::無し => None,
            Self::時刻を決める前 => Some(Duration::ZERO),
            Self::時刻(時刻) => Some(時刻.saturating_duration_since(今)),
        }
    }
}
