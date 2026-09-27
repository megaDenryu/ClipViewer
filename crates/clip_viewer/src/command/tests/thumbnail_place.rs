//! サムネイルの試験の置き場所。試験のライブラリの一時フォルダに置くサムネイルのキャッシュと、動画のあるスタックと、
//! キャッシュへ置く画像を作る。%LOCALAPPDATA% の本物のフォルダを使わない。
#![allow(clippy::expect_used)]

use std::path::PathBuf;

use clip_domain::{サムネイルの画像, スタックの識別子, トリガー};
use clip_library::スタックのライブラリ;
use thumbnail_cache::{
    キャッシュのフォルダ, キャッシュの容量, キャッシュの時計, キャッシュの置き場所,
    サムネイルのキャッシュ,
};

use super::library_support::試験のライブラリ;
use super::クリップを作る;
use crate::thumbnail_feed::一覧のサムネイルの大きさ;

impl 試験のライブラリ {
    /// 一時フォルダの中のサムネイルのキャッシュのフォルダ。
    pub(super) fn キャッシュのフォルダ(&self) -> キャッシュのフォルダ {
        キャッシュのフォルダ::作成する(self.一時フォルダ.join("thumbnails"))
    }

    /// 一時フォルダに置くサムネイルのキャッシュ。
    pub(super) fn キャッシュ(&self) -> サムネイルのキャッシュ {
        サムネイルのキャッシュ::作成する(
            キャッシュの置き場所::フォルダ(self.キャッシュのフォルダ()),
            キャッシュの容量::既定,
            キャッシュの時計::実時間(),
        )
    }

    /// 一時フォルダの中のファイルのパス。試験の後に一時フォルダごと消える。
    fn 一時のファイルのパス(&self, ファイル名: &str) -> PathBuf {
        self.一時フォルダ.join(ファイル名)
    }
}

/// 動画(中身の空のファイル)のあるスタックをライブラリへ保存し、画像を渡されたらキャッシュへ置く。動画はスタックごとに別のファイルにする。
pub(super) fn 動画のあるスタックを置く(
    試験: &試験のライブラリ,
    識別子: &str,
    キャッシュに置く画像: Option<&[u8]>,
) -> スタックの識別子 {
    // 動画のパスをスタックごとに分け、撮り方(キャッシュのファイル名)をスタックごとに別にする。
    let 動画 = 試験.一時のファイルのパス(&format!("{識別子}.mp4"));
    if let Some(フォルダ) = 動画.parent() {
        std::fs::create_dir_all(フォルダ).expect("一時フォルダを作れる");
    }
    std::fs::write(&動画, b"").expect("動画を置ける");
    let 並び = clip_domain::クリップスタック::一覧から作成する(
        vec![クリップを作る("甲", 0.0, 1.0, トリガー::自動進行)],
    )
    .expect("並び");
    let スタック = clip_domain::ライブラリのスタック::新しく作る(
        スタックの識別子::文字列から作成する(識別子.to_string()).expect("識別子"),
        clip_domain::スタックの名前::作成する("甲".into()).expect("名前"),
        clip_domain::入力された動画パス::作成する(動画.display().to_string()).正規化する(),
        並び,
        clip_domain::ライブラリの日時::紀元からのミリ秒で作成する(1_000).expect("日時"),
    );
    スタックのライブラリ::作成する(試験.フォルダ.clone())
        .保存する(&スタック)
        .expect("保存");
    if let Some(バイト列) = キャッシュに置く画像 {
        let 撮り方 = スタック
            .サムネイルの撮り方(一覧のサムネイルの大きさ)
            .expect("撮り方");
        試験
            .キャッシュ()
            .置く(&撮り方, &サムネイルの画像::作成する(バイト列.to_vec()))
            .expect("置ける");
    }
    スタック.識別子().clone()
}

pub(super) fn 赤い画像() -> Vec<u8> {
    let 画素 = image::RgbImage::from_pixel(16, 9, image::Rgb([255, 0, 0]));
    let mut バイト列 = std::io::Cursor::new(Vec::new());
    image::DynamicImage::ImageRgb8(画素)
        .write_to(&mut バイト列, image::ImageFormat::Jpeg)
        .expect("JPEG に書ける");
    バイト列.into_inner()
}
