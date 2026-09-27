//! FFmpeg の置き場所の欄。FFmpeg が見つからないときだけ、理由と手順と、フォルダを入力して保存する欄を出す。

use sengen_egui::{
    ノード, ボタン, 一行テキスト入力, 子, 文字表示, 横並び, 無し, 画素, 縦積み
};

use crate::command::{FFmpegの操作, 応答};
use crate::state::{FFmpegの状況, アプリの状態};
use crate::view::styles;

pub(crate) fn ffmpegの置き場所の欄(状態: &アプリの状態) -> ノード<応答> {
    let FFmpegの状況::見つからない {
        理由,
        入力中のフォルダ,
    } = &状態.ffmpegの状況
    else {
        return 無し();
    };
    縦積み(子![
        文字表示(format!("FFmpeg が見つからないため動画を開けない。{理由}")).装飾(styles::不備の文),
        文字表示(
            "手順: ffmpeg.exe と ffprobe.exe が入ったフォルダ(例: C:\\ffmpeg\\bin)を下に入力し、「保存して探し直す」を押す。\
             環境変数 CLIPVIEWER_FFMPEG_DIR か PATH にそのフォルダを入れて起動し直しても見つかる。"
        )
        .装飾(styles::補足),
        横並び(子![
            一行テキスト入力(入力中のフォルダ.文字列(), |入力| 応答::FFmpeg(FFmpegの操作::フォルダの入力を書き換える(入力)))
                .案内文("ffmpeg.exe と ffprobe.exe のあるフォルダ")
                .幅(画素(420.0)),
            ボタン("保存して探し直す", 応答::FFmpeg(FFmpegの操作::フォルダを保存して探し直す)).装飾(styles::強調ボタン),
        ]),
    ])
    .装飾(styles::警告の札)
    .into()
}
