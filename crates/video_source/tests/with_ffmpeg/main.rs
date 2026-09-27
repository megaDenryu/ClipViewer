//! FFmpeg を実際に起動する結合試験。FFmpeg の無い環境で黙って成功させないため、すべて `#[ignore]` にしてある。
//! `cargo xtask verify` が FFmpeg を見つけたときに、環境変数 CLIPVIEWER_FFMPEG_DIR にその場所を入れて `--ignored` で流す。
//! 手で流すとき(実行場所はリポジトリのルート):
//! `CLIPVIEWER_FFMPEG_DIR=C:\ffmpeg\bin cargo test -p video_source --test with_ffmpeg -- --ignored`

#![allow(clippy::expect_used)]

mod common;
mod drawn_number;
mod gap_video;
mod jpeg_check;
mod late_video;
mod numbered_videos;
mod numbering_gap;
mod numbering_late;
mod numbering_store;
mod numbering_stream;
mod numbering_thumbnail;
mod probe;
mod sound_cleanup;
mod sound_store;
mod sound_stream;
mod sound_support;
mod stalling_server;
mod store_cleanup;
mod store_contents;
mod store_limits;
mod store_order;
mod store_support;
mod stream;
mod test_videos;
mod thumbnail;
mod thumbnail_memo;
mod two_streams;
