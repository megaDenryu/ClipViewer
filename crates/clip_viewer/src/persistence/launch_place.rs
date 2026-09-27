//! 起動の受け口の案内の置き場所の決め方。ライブラリの錠と同じアプリの設定の保管場所(`%APPDATA%\ClipViewer`)に置く。
//! 錠を取れるかで1つ目のアプリを決めるため、案内の置き場所を錠と同じ範囲にそろえる。参照: _doc/設計/画面.md 判断13

use super::app_settings::アプリの設定の保管場所;
use crate::launch::受け口の案内ファイル;

/// 起動の受け口の案内のファイル名。settings.json と揃えて ASCII にする。
const 案内のファイル名: &str = "launch.json";

impl アプリの設定の保管場所 {
    /// 起動の受け口の案内ファイル(`%APPDATA%\ClipViewer\launch.json`)。保管場所が無ければ無い。
    pub(crate) fn 起動の受け口の案内ファイル(
        &self,
    ) -> Option<受け口の案内ファイル> {
        match self {
            Self::フォルダ(フォルダ) => Some(受け口の案内ファイル::作成する(
                フォルダ.join(案内のファイル名),
            )),
            Self::無い => None,
        }
    }
}
