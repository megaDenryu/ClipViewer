//! 窓へ落とされたファイル。拡張子で、動画として開くか、設定ファイル(JSON)として扱うかを見分ける。参照: _doc/設計/画面.md 判断14

use std::path::PathBuf;

use clip_domain::入力された動画パス;

use crate::persistence::設定ファイルのパス;

/// 落としたファイルとは、利用者が窓へ落とした OS のファイルのパスのことである。
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct 落としたファイル(PathBuf);

/// 落としたファイルの見分け方とは、落としたファイルを設定ファイルとして扱うか、動画として開くかの区別のことである。
pub(crate) enum 落としたファイルの見分け方 {
    設定ファイル(設定ファイルのパス),
    動画(入力された動画パス),
}

impl 落としたファイル {
    pub(crate) fn 作成する(パス: PathBuf) -> Self {
        Self(パス)
    }

    /// 拡張子が json(大文字と小文字を区別しない)なら設定ファイル、それ以外は動画として見分ける。
    pub(crate) fn 見分ける(self) -> 落としたファイルの見分け方 {
        let 設定ファイルか = self
            .0
            .extension()
            .is_some_and(|拡張子| 拡張子.eq_ignore_ascii_case("json"));
        if 設定ファイルか {
            return 落としたファイルの見分け方::設定ファイル(
                設定ファイルのパス::落とされたファイルから作る(self.0),
            );
        }
        落としたファイルの見分け方::動画(入力された動画パス::作成する(
            self.0.to_string_lossy().into_owned(),
        ))
    }
}
