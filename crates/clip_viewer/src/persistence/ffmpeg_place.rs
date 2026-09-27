//! 起動したときに使う FFmpeg の置き場所の決め方。参照: _doc/設計/画面.md 判断7

use video_source::{FFmpegの置き場所の設定, FFmpegを置いたフォルダ};

/// FFmpegの置き場所の候補とは、環境変数 CLIPVIEWER_FFMPEG_DIR が指すフォルダと、アプリの設定に保存した置き場所の組のことである。
#[derive(Debug, Clone)]
pub(crate) struct FFmpegの置き場所の候補 {
    pub(crate) 環境変数のフォルダ: Option<FFmpegを置いたフォルダ>,
    pub(crate) 保存した置き場所: FFmpegの置き場所の設定,
}

impl FFmpegの置き場所の候補 {
    /// 使う置き場所。環境変数があればそれを、無ければ保存した置き場所を使う。どちらも PATH より先に探す。
    /// 環境変数を先にするのは、その起動だけ別の FFmpeg を使う指定だからである。
    pub(crate) fn 使う置き場所(self) -> FFmpegの置き場所の設定 {
        self.環境変数のフォルダ
            .map_or(self.保存した置き場所, FFmpegの置き場所の設定::設定済み)
    }
}
