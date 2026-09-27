//! 第4版の triggerEvent の表記と、最新のトリガーの対応。読み込みと書き出しの両方向を1箇所に置き、対応の食い違いを防ぐ。

use super::v4::第4版のトリガー;
use crate::trigger::トリガー;

impl From<第4版のトリガー> for トリガー {
    fn from(第4版: 第4版のトリガー) -> Self {
        match 第4版 {
            第4版のトリガー::自動進行 => Self::自動進行,
            第4版のトリガー::Enterキー待ち => Self::Enterキー待ち,
            第4版のトリガー::Spaceキー待ち => Self::Spaceキー待ち,
            第4版のトリガー::クリック待ち => Self::クリック待ち,
        }
    }
}

impl From<トリガー> for 第4版のトリガー {
    fn from(トリガー: トリガー) -> Self {
        match トリガー {
            トリガー::自動進行 => Self::自動進行,
            トリガー::Enterキー待ち => Self::Enterキー待ち,
            トリガー::Spaceキー待ち => Self::Spaceキー待ち,
            トリガー::クリック待ち => Self::クリック待ち,
        }
    }
}
