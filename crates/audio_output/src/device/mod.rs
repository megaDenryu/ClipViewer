//! 音声出力装置(cpal)の境界。既定の出力装置を既定の設定で開き、音声のスレッドで音を埋める係を呼ばせる。
//! 参照: _doc/設計/アーキテクチャ.md 判断8-4・8-7

mod callback;
mod error;
mod shared;

pub use error::音声出力のエラー;

use std::sync::Arc;
use std::time::Instant;

use audio_pcm::サンプリング周波数;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{ErrorKind, FromSample, SampleFormat, SizedSample};

use crate::instruction::再生の指示;
use callback::音を埋める係;
use shared::{再生の共有, 装置の知らせ};

/// 音声出力とは、開いた出力装置の流れと、画面のスレッドと音声のスレッドが共有する再生の共有と、装置の知らせの組のことである。
/// 落とすと流れが止まり、音声のスレッドはもう呼ばれない。
pub struct 音声出力 {
    周波数: サンプリング周波数,
    共有: Arc<再生の共有>,
    知らせ: Arc<装置の知らせ>,
    _流れ: cpal::Stream,
}

impl 音声出力 {
    /// 既定の出力装置を既定の設定で開き、黙る指示で鳴らし始める。
    pub fn 既定の装置を開く() -> Result<Self, 音声出力のエラー> {
        let 装置 = cpal::default_host()
            .default_output_device()
            .ok_or(音声出力のエラー::装置が無い)?;
        let 設定 = 装置
            .default_output_config()
            .map_err(音声出力のエラー::設定を取れない)?;
        let 周波数 = サンプリング周波数::作成する(設定.sample_rate())
            .ok_or(音声出力のエラー::周波数が無い)?;
        let 共有 = Arc::new(再生の共有::作成する(周波数, Instant::now()));
        let 知らせ = Arc::new(装置の知らせ::default());
        let 係 = 音を埋める係::作成する(
            Arc::clone(&共有),
            usize::from(設定.channels()),
            周波数,
        );
        let 流れ = match 設定.sample_format() {
            SampleFormat::F32 => 流れを作る::<f32>(&装置, 設定.config(), 係, &知らせ),
            SampleFormat::F64 => 流れを作る::<f64>(&装置, 設定.config(), 係, &知らせ),
            SampleFormat::I16 => 流れを作る::<i16>(&装置, 設定.config(), 係, &知らせ),
            SampleFormat::I32 => 流れを作る::<i32>(&装置, 設定.config(), 係, &知らせ),
            SampleFormat::U16 => 流れを作る::<u16>(&装置, 設定.config(), 係, &知らせ),
            SampleFormat::U8 => 流れを作る::<u8>(&装置, 設定.config(), 係, &知らせ),
            形式 => return Err(音声出力のエラー::対応しない形式(形式)),
        }
        .map_err(音声出力のエラー::流れを作れない)?;
        流れ.play().map_err(音声出力のエラー::始められない)?;
        Ok(Self {
            周波数,
            共有,
            知らせ,
            _流れ: 流れ,
        })
    }

    /// 出力装置のサンプリング周波数。FFmpeg にこの周波数で音を出させる。
    pub fn 周波数(&self) -> サンプリング周波数 {
        self.周波数
    }

    /// 再生の指示を置き換える。前の指示と、音声のスレッドが手放した出どころは、錠の外でここで捨てる。
    pub fn 指示を渡す(&self, 指示: 再生の指示) {
        let 捨てるもの = self.共有.指示を置き換える(指示);
        drop(捨てるもの);
    }

    /// 出力の途中で装置が使えなくなった理由。使えていれば無い。
    pub fn 使えなくなった理由(&self) -> Option<音声出力のエラー> {
        self.知らせ.使えなくなった理由()
    }

    /// 音声のスレッドがこれまでに埋めた標本の数。装置が実際に音を求めているかを確かめるために使う。
    pub fn 埋めた標本数(&self) -> u64 {
        self.共有.埋めた標本数()
    }
}

fn 流れを作る<標本の形式: SizedSample + FromSample<f64>>(
    装置: &cpal::Device,
    設定: cpal::StreamConfig,
    mut 係: 音を埋める係,
    知らせ: &Arc<装置の知らせ>,
) -> Result<cpal::Stream, cpal::Error> {
    let 知らせ = Arc::clone(知らせ);
    装置.build_output_stream(
        設定,
        move |並び: &mut [標本の形式], _| 係.装置の並びを埋める(並び),
        move |不具合| 装置の不具合を覚える(&知らせ, &不具合),
        None,
    )
}

/// 装置の不具合を覚える。経路の切り替え・音の途切れ・優先度の不許可は流れが続くため、使えなくなったとはみなさない。
fn 装置の不具合を覚える(知らせ: &装置の知らせ, 不具合: &cpal::Error) {
    match 不具合.kind() {
        ErrorKind::DeviceChanged | ErrorKind::Xrun | ErrorKind::RealtimeDenied => {}
        _ => 知らせ.使えなくなったと覚える(音声出力のエラー::使えなくなった(
            不具合.clone(),
        )),
    }
}
