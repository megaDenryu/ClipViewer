//! ウインドウの題名。起動の部分(main.rs)がウインドウを作るときに eframe へ渡す。ウインドウの大きさの型は、settings.json も使うため `viewer_settings` に置く。

/// ウインドウの題名。利用者が版を確かめられるように、アプリの名に版を添える。
pub(crate) const ウインドウの題名: &str = concat!("ClipViewer ", env!("CARGO_PKG_VERSION"));
