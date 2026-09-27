//! スタックのライブラリの置き場所の決め方。アプリの設定の保管場所の下の `library` フォルダにする。
//! 参照: _doc/設計/ライブラリ.md 判断2

use clip_library::ライブラリのフォルダ;

use super::app_settings::アプリの設定の保管場所;

/// スタックのライブラリのフォルダ名。settings.json と揃えて ASCII にする。
const ライブラリのフォルダ名: &str = "library";

impl アプリの設定の保管場所 {
    /// スタックのライブラリのフォルダ(`%APPDATA%\ClipViewer\library`)。保管場所が無ければ無い。
    pub(crate) fn ライブラリのフォルダ(&self) -> Option<ライブラリのフォルダ> {
        match self {
            Self::フォルダ(フォルダ) => Some(ライブラリのフォルダ::作成する(
                フォルダ.join(ライブラリのフォルダ名),
            )),
            Self::無い => None,
        }
    }
}
