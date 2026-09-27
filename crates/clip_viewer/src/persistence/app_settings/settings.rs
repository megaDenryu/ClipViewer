//! 最新のアプリの設定。版ごとの型はこれへ変換してから使い、書くときはこれから最新の版の型を作る。

use serde_json::{Map, Value};
use video_source::FFmpegの置き場所の設定;

use crate::viewer_settings::見る側の設定;

/// 知らない項目とは、settings.json の同じ版の中で、このアプリが知らない項目の組のことである。
/// 新しいアプリが同じ版に足した項目であり、読んだまま持ち回り、書くときに戻す(古いアプリが新しい項目を消さないため)。
#[derive(Debug, Clone, Default, PartialEq)]
pub(super) struct 知らない項目(Map<String, Value>);

impl 知らない項目 {
    pub(super) fn 作成する(項目: Map<String, Value>) -> Self {
        Self(項目)
    }

    /// 項目の組を返す。ファイルの形の型へ渡す境界で使う。
    pub(super) fn 組にする(self) -> Map<String, Value> {
        self.0
    }
}

/// 覚えた見る側の設定とは、settings.json が見る側の設定を覚えているか、まだ一度も書いていないかの区別のことである。
/// 覚えていないときは項目を書かない(FFmpeg の置き場所だけを保存したファイルに、見る側の項目を足さないため)。
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) enum 覚えた見る側の設定 {
    覚えていない,
    覚えている(見る側の設定),
}

/// アプリの設定とは、settings.json に書くアプリの設定の、このアプリが知っている最新の形のことである。
/// 項目を足すときは、版ごとの型(`v1.rs`)の変換と一緒に足す。変換は `..` を使わずに分解・組み立てるため、足し忘れはコンパイルエラーになる。
#[derive(Debug, Clone, PartialEq)]
pub(super) struct アプリの設定 {
    pub(super) ffmpegの置き場所: FFmpegの置き場所の設定,
    pub(super) 見る側: 覚えた見る側の設定,
    pub(super) 知らない項目: 知らない項目,
}

impl アプリの設定 {
    /// 何も設定していない設定。settings.json がまだ無いときに使う。
    pub(super) fn 何も無い() -> Self {
        Self {
            ffmpegの置き場所: FFmpegの置き場所の設定::未設定,
            見る側: 覚えた見る側の設定::覚えていない,
            知らない項目: 知らない項目::default(),
        }
    }

    /// 保存した FFmpeg の置き場所。
    pub(super) fn ffmpegの置き場所(&self) -> &FFmpegの置き場所の設定 {
        &self.ffmpegの置き場所
    }

    /// FFmpeg の置き場所だけを変えた設定を返す。ほかの項目は変えない。
    pub(super) fn ffmpegの置き場所を変えた(
        self,
        置き場所: FFmpegの置き場所の設定,
    ) -> Self {
        Self {
            ffmpegの置き場所: 置き場所,
            ..self
        }
    }

    /// 覚えている見る側の設定。覚えていなければ既定の設定である。
    pub(super) fn 見る側の設定(&self) -> 見る側の設定 {
        match self.見る側 {
            覚えた見る側の設定::覚えていない => 見る側の設定::既定,
            覚えた見る側の設定::覚えている(設定) => 設定,
        }
    }

    /// 見る側の設定だけを変えた設定を返す。ほかの項目は変えない。
    pub(super) fn 見る側の設定を変えた(self, 設定: 見る側の設定) -> Self {
        Self {
            見る側: 覚えた見る側の設定::覚えている(設定),
            ..self
        }
    }
}
