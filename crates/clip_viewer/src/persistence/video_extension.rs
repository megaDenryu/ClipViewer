//! 動画として開けるファイルの拡張子の一覧。「動画を開く」のファイルダイアログが動画として出す種類を決める。
//! 注意: インストーラーが関連付ける拡張子(installer/file_association.iss の VideoExtensions)と同じ一覧に保つ。
//! 2つが食い違わないことは video_extension_tests.rs が .iss を読んで確かめる。

/// 動画として開ける拡張子とは、ファイルダイアログが動画として出す拡張子(点を付けない小文字)のことである。
/// 並びはインストーラーの一覧と同じにする。
pub(crate) const 動画として開ける拡張子: [&str; 12] = [
    "mp4", "m4v", "mkv", "webm", "mov", "avi", "wmv", "flv", "mpg", "mpeg", "m2ts", "mts",
];
