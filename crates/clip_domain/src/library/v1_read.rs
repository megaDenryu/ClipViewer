//! 第1版のライブラリのファイルから最新のライブラリのスタックへの変換。

use super::date::ライブラリの日時;
use super::error::ライブラリのファイルの読み込みエラー;
use super::item::ライブラリのスタック;
use super::name::スタックの名前;
use super::stack_id::スタックの識別子;
use super::v1::第1版のファイル;
use crate::stack::クリップスタック;
use crate::video_path::入力された動画パス;

impl 第1版のファイル {
    /// 最新のライブラリのスタックへ変換する。動画のパスは読み込みの時点で正規化する。
    /// 形式の名前と版は読み込みの前に確かめ終えているため、ここでは使わない。
    pub(crate) fn 最新へ変換する(
        self,
    ) -> Result<ライブラリのスタック, ライブラリのファイルの読み込みエラー> {
        let Self {
            format: _,
            version: _,
            identifier,
            name,
            video_path,
            created_at_ms,
            updated_at_ms,
            clips,
        } = self;
        let mut 一覧 = Vec::with_capacity(clips.len());
        for (番号, 第1版) in clips.into_iter().enumerate() {
            let クリップ = 第1版.最新へ変換する().map_err(|理由| {
                ライブラリのファイルの読み込みエラー::クリップが不正 {
                    番号,
                    理由,
                }
            })?;
            一覧.push(クリップ);
        }
        Ok(
            ライブラリのスタック::各項目から組み立てる(
                スタックの識別子::文字列から作成する(identifier)?,
                スタックの名前::作成する(name)?,
                入力された動画パス::作成する(video_path).正規化する(),
                クリップスタック::一覧から作成する(一覧)?,
                ライブラリの日時::紀元からのミリ秒で作成する(created_at_ms)?,
                ライブラリの日時::紀元からのミリ秒で作成する(updated_at_ms)?,
            ),
        )
    }
}
