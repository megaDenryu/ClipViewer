//! 第4版の設定ファイルから最新のスタック設定への変換。
//! 欠けた項目の既定値は移植元 `Service/設定IOサービス.ts` の `JSONからクリップを復元する` と同じである。

use super::error::{クリップの値の不備, 設定ファイルの読み込みエラー};
use super::v4::{
    無限ループの表記, 第4版のクリップ, 第4版のクロップ, 第4版の繰り返し, 第4版の設定ファイル,
};
use super::スタック設定;
use crate::clip::{クリップ, クリップ名};
use crate::clip_id::{クリップ識別子, クリップ識別子の発行元};
use crate::crop::クロップ範囲;
use crate::repeat::{リピート回数, リピート設定};
use crate::stack::クリップスタック;
use crate::time::時刻;
use crate::trigger::トリガー;
use crate::video_path::{入力された動画パス, 動画ファイル名};
use crate::video_span::動画上の区間;

impl 第4版の設定ファイル {
    /// 最新のスタック設定へ変換する。期待する動画のパスは読み込みの時点で正規化する(移植元の読み込み処理と同じ)。
    pub(crate) fn 最新へ変換する(
        self,
        発行元: &mut impl クリップ識別子の発行元,
    ) -> Result<スタック設定, 設定ファイルの読み込みエラー> {
        let mut 一覧 = Vec::with_capacity(self.modifiers.len());
        for (番号, 第4版) in self.modifiers.into_iter().enumerate() {
            let クリップ = 第4版.クリップへ変換する(発行元).map_err(|理由| {
                設定ファイルの読み込みエラー::クリップが不正 { 番号, 理由 }
            })?;
            一覧.push(クリップ);
        }
        let スタック = クリップスタック::一覧から作成する(一覧)
            .map_err(設定ファイルの読み込みエラー::スタックを作れない)?;
        let 期待する動画のパス =
            入力された動画パス::作成する(self.expected_video_path.unwrap_or_default()).正規化する();
        Ok(スタック設定 {
            スタック,
            期待する動画: self.expected_video_name.and_then(動画ファイル名::作成する),
            期待する動画のパス: (!期待する動画のパス.空か()).then_some(期待する動画のパス),
        })
    }
}

impl 第4版のクリップ {
    fn クリップへ変換する(
        self,
        発行元: &mut impl クリップ識別子の発行元,
    ) -> Result<クリップ, クリップの値の不備> {
        let 識別子 = match self.id.map(クリップ識別子::文字列から作成する) {
            Some(Ok(識別子)) => 識別子,
            _ => 発行元.新しい識別子を発行する(),
        };
        let 名前 = self
            .name
            .filter(|名前| !名前.is_empty())
            .unwrap_or_else(|| "Unnamed Clip".to_string());
        let 区間 = 動画上の区間::作成する(
            時刻::作成する(self.start.unwrap_or(0.0))?,
            時刻::作成する(self.end.unwrap_or(5.0))?,
        )?;
        let mut クリップ =
            クリップ::既定値で作成する(識別子, クリップ名::作成する(名前));
        クリップ.有効か = self.active.unwrap_or(true);
        クリップ.区間 = 区間;
        クリップ.リピート = self.repeat.map_or(
            Ok(リピート設定::回数指定(リピート回数::一回)),
            第4版の繰り返し::最新へ変換する,
        )?;
        クリップ.クロップ = self
            .crop
            .map_or(クロップ範囲::全体, 第4版のクロップ::最新へ変換する);
        クリップ.トリガー = self
            .trigger_event
            .map_or(トリガー::自動進行, トリガー::from);
        Ok(クリップ)
    }
}

impl 第4版の繰り返し {
    fn 最新へ変換する(self) -> Result<リピート設定, クリップの値の不備> {
        match self {
            Self::文字列(表記) if 表記 == 無限ループの表記 => {
                Ok(リピート設定::無限ループ)
            }
            Self::文字列(表記) => Err(クリップの値の不備::繰り返し(表記)),
            Self::回数(数値) => {
                let 回数 = 数値.as_f64().unwrap_or(1.0);
                Ok(リピート設定::回数指定(
                    リピート回数::切り捨てて範囲へ収める(回数),
                ))
            }
        }
    }
}

impl 第4版のクロップ {
    fn 最新へ変換する(self) -> クロップ範囲 {
        クロップ範囲::各値を範囲へ収めて作成する(
            self.x.unwrap_or(0.0),
            self.y.unwrap_or(0.0),
            self.w.unwrap_or(100.0),
            self.h.unwrap_or(100.0),
        )
    }
}
