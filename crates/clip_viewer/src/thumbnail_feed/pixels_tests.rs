//! 戻した画素の試験。JPEG を寸法どおりの赤・緑・青・不透明度の並びへ戻し、JPEG でないバイト列は理由付きで失敗する。
#![allow(clippy::expect_used)]

use clip_domain::サムネイルの画像;

use super::pixels::戻した画素;

/// 幅8・高さ4の赤一色の JPEG を image クレートで作る。
fn 赤い画像() -> サムネイルの画像 {
    let 画素 = image::RgbImage::from_pixel(8, 4, image::Rgb([255, 0, 0]));
    let mut バイト列 = std::io::Cursor::new(Vec::new());
    image::DynamicImage::ImageRgb8(画素)
        .write_to(&mut バイト列, image::ImageFormat::Jpeg)
        .expect("JPEG に書ける");
    サムネイルの画像::作成する(バイト列.into_inner())
}

#[test]
fn 画像を寸法どおりの四バイトの画素へ戻す() {
    let 戻した = 戻した画素::サムネイルの画像から戻す(&赤い画像()).expect("戻せる");
    assert_eq!((戻した.寸法().幅(), 戻した.寸法().高さ()), (8, 4));
    let 最初の画素 = &戻した.並び()[..4];
    assert!(
        最初の画素[0] > 200 && 最初の画素[1] < 60 && 最初の画素[3] == 255,
        "{最初の画素:?}"
    );
    assert_eq!(戻した.並び().len(), 8 * 4 * 4);
}

#[test]
fn 画像でないバイト列は理由付きで失敗する() {
    let 壊れた = サムネイルの画像::作成する(b"not a jpeg".to_vec());
    let 理由 = 戻した画素::サムネイルの画像から戻す(&壊れた).expect_err("戻せない");
    assert!(
        理由.to_string().contains("サムネイルの画像を読めない"),
        "{理由}"
    );
}
