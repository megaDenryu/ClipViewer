//! egui の画面の通知では利用者へ届かない知らせを見せる口。起動の途中の失敗と、落ちたときの知らせ(`crash_record`)が使う。
//! release のビルドはコンソールのウインドウを持たず標準エラーが見えないため、標準エラーへ出すのに加えて OS のメッセージボックスで見せる。
//! egui の画面で見せないのは、日本語フォントを読めなかった失敗を日本語フォントの無い egui では読めず、ウインドウを作れなかった失敗と
//! 落ちたときには egui の画面そのものが無いか動いていないためである。

/// 画面の外で見せる知らせとは、egui の画面の通知では利用者へ届かない失敗の文面のことである。
pub(crate) struct 画面の外で見せる知らせ(String);

impl 画面の外で見せる知らせ {
    pub(crate) fn 作成する(文面: String) -> Self {
        Self(文面)
    }

    /// 標準エラーへ出し、OS のメッセージボックスで見せる。利用者が閉じるまで戻らない。
    pub(crate) fn 利用者へ見せる(&self) {
        eprintln!("{}", self.0);
        rfd::MessageDialog::new()
            .set_level(rfd::MessageLevel::Warning)
            .set_title("ClipViewer")
            .set_description(self.0.as_str())
            .set_buttons(rfd::MessageButtons::Ok)
            .show();
    }
}
