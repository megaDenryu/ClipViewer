//! 設定ファイルを保存するときに、保存のダイアログへ最初に入れておくファイル名。

use clip_domain::動画ファイル名;

/// 設定ファイルの既定の名前とは、「<動画のファイル名の最初の点より前>_clip_stack_config.json」の形のファイル名のことである。
/// 動画が分からなければ「stack_clip_stack_config.json」である(移植元と同じ)。
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct 設定ファイルの既定の名前(String);

impl 設定ファイルの既定の名前 {
    pub(crate) fn 動画から作る(動画: Option<&動画ファイル名>) -> Self {
        let 拡張子を除いた名前 = 動画
            .and_then(|名前| 名前.文字列().split('.').next())
            .filter(|拡張子を除いた名前| !拡張子を除いた名前.is_empty())
            .unwrap_or("stack");
        Self(format!("{拡張子を除いた名前}_clip_stack_config.json"))
    }

    /// ダイアログへ渡す文字列。
    pub(crate) fn 文字列(&self) -> &str {
        &self.0
    }
}
