//! 装置の流れを開く手順。既定の出力装置を既定の設定で選び、時刻を渡して埋めさせる共有を音を埋める係に持たせて流れを始める。
//! 1本目の流れ(スタックの作業場の音)と2本目の流れ(行を混ぜる音)は、同じ手順で同じ既定の装置に開く。

use std::sync::Arc;

use audio_pcm::サンプリング周波数;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{ErrorKind, FromSample, SampleFormat, SizedSample};

use super::callback::音を埋める係;
use super::error::音声出力のエラー;
use super::filling::時刻を渡して埋めさせる共有;
use super::shared::装置の知らせ;

/// 選んだ装置とは、既定の出力装置と、その既定の設定と、設定のサンプリング周波数の組のことである。
/// 流れを始める前に周波数を読めるよう、選ぶことと始めることを分ける(共有は周波数を知って作るため)。
pub(crate) struct 選んだ装置 {
    装置: cpal::Device,
    設定: cpal::SupportedStreamConfig,
    周波数: サンプリング周波数,
}

/// 開いた流れとは、鳴らし始めた cpal の流れと、その流れの装置の知らせの組のことである。落とすと流れが止まる。
pub(crate) struct 開いた流れ {
    知らせ: Arc<装置の知らせ>,
    _流れ: cpal::Stream,
}

impl 選んだ装置 {
    /// 既定の出力装置と、その既定の設定を選ぶ。
    pub(crate) fn 既定の装置を選ぶ() -> Result<Self, 音声出力のエラー> {
        let 装置 = cpal::default_host()
            .default_output_device()
            .ok_or(音声出力のエラー::装置が無い)?;
        let 設定 = 装置
            .default_output_config()
            .map_err(音声出力のエラー::設定を取れない)?;
        let 周波数 = サンプリング周波数::作成する(設定.sample_rate())
            .ok_or(音声出力のエラー::周波数が無い)?;
        Ok(Self {
            装置, 設定, 周波数
        })
    }

    /// 選んだ設定のサンプリング周波数。
    pub(crate) fn 周波数(&self) -> サンプリング周波数 {
        self.周波数
    }

    /// 共有を持つ音を埋める係で流れを作り、鳴らし始める。
    pub(crate) fn 流れを始める<共有: 時刻を渡して埋めさせる共有>(
        self,
        共有: Arc<共有>,
    ) -> Result<開いた流れ, 音声出力のエラー> {
        let 知らせ = Arc::new(装置の知らせ::default());
        let 係 =
            音を埋める係::作成する(共有, usize::from(self.設定.channels()), self.周波数);
        let 装置 = &self.装置;
        let 設定 = self.設定.config();
        let 流れ = match self.設定.sample_format() {
            SampleFormat::F32 => 流れを作る::<f32, _>(装置, 設定, 係, &知らせ),
            SampleFormat::F64 => 流れを作る::<f64, _>(装置, 設定, 係, &知らせ),
            SampleFormat::I16 => 流れを作る::<i16, _>(装置, 設定, 係, &知らせ),
            SampleFormat::I32 => 流れを作る::<i32, _>(装置, 設定, 係, &知らせ),
            SampleFormat::U16 => 流れを作る::<u16, _>(装置, 設定, 係, &知らせ),
            SampleFormat::U8 => 流れを作る::<u8, _>(装置, 設定, 係, &知らせ),
            形式 => return Err(音声出力のエラー::対応しない形式(形式)),
        }
        .map_err(音声出力のエラー::流れを作れない)?;
        流れ.play().map_err(音声出力のエラー::始められない)?;
        Ok(開いた流れ {
            知らせ,
            _流れ: 流れ,
        })
    }
}

impl 開いた流れ {
    /// 出力の途中で装置が使えなくなった理由。使えていれば無い。
    pub(crate) fn 使えなくなった理由(&self) -> Option<音声出力のエラー> {
        self.知らせ.使えなくなった理由()
    }
}

fn 流れを作る<標本の形式: SizedSample + FromSample<f64>, 共有: 時刻を渡して埋めさせる共有>(
    装置: &cpal::Device,
    設定: cpal::StreamConfig,
    mut 係: 音を埋める係<共有>,
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
