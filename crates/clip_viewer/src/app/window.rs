//! 窓の題名。起動の部分(main.rs)が窓を作るときに eframe へ渡す。窓の大きさの型は、settings.json も使うため `viewer_settings` に置く。

/// 窓の題名。利用者が版を確かめられるように、アプリの名に版を添える。
pub(crate) const 窓の題名: &str = concat!("ClipViewer ", env!("CARGO_PKG_VERSION"));
