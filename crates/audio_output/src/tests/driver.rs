//! 出力装置の代わりに音の再生器を呼ぶ鳴らし手。時刻を埋めた標本の分だけ進め、左の値を集める。

use std::time::{Duration, Instant};

use audio_pcm::{ステレオの標本, 小数の標本位置, 音量};
use clip_domain::再生速度;

use super::{周波数, 毎秒の標本数};
use crate::{
    位置を飛ばした回数, 再生の指示, 再生の様子, 終わりで続く音, 退いた出どころの置き場, 音の再生器,
    音の区間,
};

/// 出力装置の代わりに、時刻を標本の分だけ進めながら再生器に埋めさせるもの。
pub(crate) struct 鳴らし手 {
    pub(crate) 再生器: 音の再生器,
    pub(crate) 置き場: 退いた出どころの置き場,
    pub(crate) 基準: Instant,
    埋めた数: u32,
}

impl 鳴らし手 {
    pub(crate) fn 作成する() -> Self {
        Self {
            再生器: 音の再生器::作成する(周波数()),
            置き場: 退いた出どころの置き場::default(),
            基準: Instant::now(),
            埋めた数: 0,
        }
    }

    /// 最初の標本を作る時刻に指示した、再生している指示。
    pub(crate) fn 指示(
        &self,
        位置: u32,
        区間: Option<音の区間>,
        行き先: 終わりで続く音,
    ) -> 再生の指示 {
        再生の指示 {
            様子: 再生の様子::再生している,
            位置: 小数の標本位置::収めて作る(f64::from(位置)),
            指示した時刻: self.今(),
            速度: 再生速度::等倍,
            音量: 音量::範囲へ収めて作る(1.0),
            区間,
            行き先,
            飛ばした回数: 位置を飛ばした回数::default(),
        }
    }

    pub(crate) fn 今(&self) -> Instant {
        self.基準 + Duration::from_secs_f64(f64::from(self.埋めた数) / f64::from(毎秒の標本数))
    }

    /// 512標本ずつ、合わせて数だけ埋めさせ、左の値を並べて返す。
    pub(crate) fn 鳴らす(&mut self, 指示: &再生の指示, 数: u32) -> Vec<f64> {
        let mut 左の並び = Vec::new();
        let mut 一回分 = [ステレオの標本::無音; 512];
        let mut 残り = 数;
        while 残り > 0 {
            let 今回 = 残り.min(512);
            let 出力 = &mut 一回分[..usize::try_from(今回).expect("長さ")];
            self.再生器.埋める(指示, self.今(), 出力, &mut self.置き場);
            左の並び.extend(出力.iter().map(|標本| 標本.左));
            self.埋めた数 += 今回;
            残り -= 今回;
        }
        左の並び
    }
}
