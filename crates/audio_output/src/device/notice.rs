//! 装置の知らせ。cpal が流れの不具合を知らせる関数から書き、画面のスレッドが読む。
//! 標本を作る経路には関わらず、知らされたことを覚えるだけである。

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, PoisonError};

use cpal::ErrorKind;

use super::error::音声出力のエラー;

/// 装置の知らせとは、出力の途中で装置が使えなくなったときに cpal が知らせた理由と、cpal が音の途切れ(`ErrorKind::Xrun`)を
/// 知らせた回数の組のことである。
/// 注意: cpal 0.18 の WASAPI(Windows)は出力の流れで途切れを知らせない(入力の流れでだけ知らせる)。そのため Windows では
/// 途切れを知らされた回数は常に0であり、途切れが無かったことの証明にはならない。
#[derive(Default)]
pub(crate) struct 装置の知らせ {
    使えなくなった理由: Mutex<Option<音声出力のエラー>>,
    途切れを知らされた回数: AtomicU64,
}

impl 装置の知らせ {
    /// cpal が知らせた不具合を覚える。音の途切れは数え、経路の切り替えと優先度の不許可は流れが続くため何もしない。
    /// それ以外は使えなくなったとみなし、最初の理由だけを残す。
    pub(crate) fn 不具合を覚える(&self, 不具合: &cpal::Error) {
        match 不具合.kind() {
            ErrorKind::Xrun => {
                self.途切れを知らされた回数.fetch_add(1, Ordering::Relaxed);
            }
            ErrorKind::DeviceChanged | ErrorKind::RealtimeDenied => {}
            _ => {
                let mut 中身 = self
                    .使えなくなった理由
                    .lock()
                    .unwrap_or_else(PoisonError::into_inner);
                if 中身.is_none() {
                    *中身 = Some(音声出力のエラー::使えなくなった(
                        不具合.clone(),
                    ));
                }
            }
        }
    }

    /// 使えなくなった理由。使えていれば無い。
    pub(crate) fn 使えなくなった理由(&self) -> Option<音声出力のエラー> {
        self.使えなくなった理由
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }

    /// cpal が音の途切れを知らせた回数。
    pub(crate) fn 途切れを知らされた回数(&self) -> u64 {
        self.途切れを知らされた回数.load(Ordering::Relaxed)
    }
}
