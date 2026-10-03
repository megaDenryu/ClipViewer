//! 配線の層の試験。起動の手順・閉じる前の確かめ・起動の頼み・見る側の設定の保存・作業場を移すことの試験と、その試験の道具を集める。

mod close_tests;
mod launch_plan_tests;
pub(in crate::app) mod launch_requests_test_support;
mod launch_requests_tests;
mod launch_window_tests;
mod settings_watch_tests;
mod viewer_settings_save_tests;
mod workspace_drop_tests;
mod workspace_front_tests;
mod workspace_keys_tests;
mod workspace_keys_with_ffmpeg_tests;
mod workspace_overlay_notice_tests;
mod workspace_overlay_sound_tests;
mod workspace_overlay_tests;
mod workspace_overlay_with_ffmpeg_tests;
mod workspace_redraw_tests;
pub(in crate::app) mod workspace_test_frames;
pub(in crate::app) mod workspace_test_keys;
pub(in crate::app) mod workspace_test_snapshot;
pub(in crate::app) mod workspace_test_support;
mod workspace_tests;
