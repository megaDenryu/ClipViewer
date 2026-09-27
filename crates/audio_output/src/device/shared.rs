//! 画面のスレッドと音声のスレッドが共有するもの。再生の指示と音の再生器と置き場を1つの錠に入れ、装置の不具合の知らせを別に持つ。

use std::sync::{Mutex, MutexGuard, PoisonError};
use std::time::Instant;

use audio_pcm::{サンプリング周波数, ステレオの標本};

use super::error::音声出力のエラー;
use crate::instruction::再生の指示;
use crate::player::音の再生器;
use crate::retired::退いた出どころの置き場;

/// 再生の共有の中身とは、画面のスレッドが置き換える再生の指示と、音声のスレッドが進める音の再生器と、退いた出どころの置き場の組のことである。
pub(crate) struct 共有の中身 {
    指示: 再生の指示,
    再生器: 音の再生器,
    置き場: 退いた出どころの置き場,
}

/// 再生の共有とは、共有の中身を入れた錠のことである。
/// 注意: 画面のスレッドは中身を入れ替えるだけで錠を放し、古い指示と置き場の中身は錠の外で捨てる。
/// 音声のスレッドは錠を持ったまま標本を作るが、メモリの確保と解放はしない。
pub(crate) struct 再生の共有(Mutex<共有の中身>);

impl 再生の共有 {
    pub(crate) fn 作成する(周波数: サンプリング周波数, 今: Instant) -> Self {
        Self(Mutex::new(共有の中身 {
            指示: 再生の指示::黙る(今),
            再生器: 音の再生器::作成する(周波数),
            置き場: 退いた出どころの置き場::default(),
        }))
    }

    /// 指示を置き換え、前の指示と、置き場が預かっていた出どころを返す。呼び出し側が錠の外で捨てる。
    pub(crate) fn 指示を置き換える(
        &self,
        指示: 再生の指示,
    ) -> (再生の指示, 退いた出どころの置き場) {
        let mut 中身 = self.錠を取る();
        let 前の指示 = std::mem::replace(&mut 中身.指示, 指示);
        (前の指示, 中身.置き場.引き渡す())
    }

    /// 音の再生器で出力の並びを埋める。音声のスレッドから呼ぶ。
    pub(crate) fn 埋める(&self, 今: Instant, 出力: &mut [ステレオの標本]) {
        let mut 中身 = self.錠を取る();
        let 共有の中身 {
            指示,
            再生器,
            置き場,
        } = &mut *中身;
        再生器.埋める(指示, 今, 出力, 置き場);
    }

    /// これまでに埋めた標本の数。
    pub(crate) fn 埋めた標本数(&self) -> u64 {
        self.錠を取る().再生器.埋めた標本数()
    }

    /// 錠を取る。錠を持ったスレッドがパニックしても、指示の入れ替えは1回の代入なので、中身をそのまま使う。
    fn 錠を取る(&self) -> MutexGuard<'_, 共有の中身> {
        self.0.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

/// 装置の知らせとは、出力の途中で装置が使えなくなったときに cpal が知らせた理由のことである。
#[derive(Default)]
pub(crate) struct 装置の知らせ(Mutex<Option<音声出力のエラー>>);

impl 装置の知らせ {
    /// 使えなくなった理由を覚える。最初の理由だけを残す。
    pub(crate) fn 使えなくなったと覚える(&self, 理由: 音声出力のエラー) {
        let mut 中身 = self.0.lock().unwrap_or_else(PoisonError::into_inner);
        if 中身.is_none() {
            *中身 = Some(理由);
        }
    }

    /// 使えなくなった理由。使えていれば無い。
    pub(crate) fn 使えなくなった理由(&self) -> Option<音声出力のエラー> {
        self.0
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }
}
