//! 手元の SengenEgui のフォルダと、cargo の `--config` へ渡す差し替えの設定。

use std::ffi::OsString;
use std::path::{Path, PathBuf};

/// 差し替える git 依存の取得元。crates/clip_viewer/Cargo.toml の sengen_egui の `git` と一致させる。
const 差し替える取得元: &str = "https://github.com/megaDenryu/SengenEgui";

/// 手元の SengenEgui のフォルダを既定の場所(リポジトリの1つ上の SengenEgui)以外から選ぶための環境変数の名。
pub const フォルダを指す環境変数: &str = "CLIPVIEWER_SENGEN_EGUI_DIR";

/// 手元のSengenEguiのフォルダとは、push する前の SengenEgui の作業ツリーのうち、Cargo.toml があると確かめた絶対パスのことである。
/// パスは UTF-8 で書け、単引用符を含まないと確かめてある(差し替えの設定を TOML のリテラル文字列で書くため)。
pub struct 手元のSengenEguiのフォルダ(PathBuf);

impl 手元のSengenEguiのフォルダ {
    /// 環境変数の値があればそのフォルダを、無ければリポジトリの1つ上の SengenEgui を選び、使えるかを確かめる。
    pub fn 見つける(
        リポジトリルート: &Path,
        環境変数の値: Option<OsString>,
    ) -> Result<Self, String> {
        let 候補 = match 環境変数の値 {
            Some(値) => PathBuf::from(値),
            None => リポジトリルート
                .parent()
                .map_or_else(|| リポジトリルート.join(".."), Path::to_path_buf)
                .join("SengenEgui"),
        };
        let フォルダ = std::path::absolute(&候補)
            .map_err(|原因| format!("{} を絶対パスにできない: {原因}", 候補.display()))?;
        if !フォルダ.join("Cargo.toml").is_file() {
            return Err(format!(
                "手元の SengenEgui が見つからない({} に Cargo.toml が無い)。SengenEgui をリポジトリの1つ上へ置くか、環境変数 {フォルダを指す環境変数} にフォルダを渡す。push 済みの SengenEgui を使うなら、このコマンドを使わず crates/clip_viewer/Cargo.toml の rev を上げる",
                フォルダ.display()
            ));
        }
        match フォルダ.to_str() {
            Some(文字列) if !文字列.contains('\'') => Ok(Self(フォルダ)),
            _ => Err(format!(
                "手元の SengenEgui のパス {} は、UTF-8 で書けないか単引用符を含むため、cargo の設定に書けない",
                フォルダ.display()
            )),
        }
    }

    pub fn フォルダ(&self) -> &Path {
        &self.0
    }

    /// cargo の `--config` へ渡す1行。git 依存の sengen_egui を、このフォルダの path 依存へ差し替える。
    pub fn 差し替えの設定(&self) -> String {
        format!(
            "patch.\"{差し替える取得元}\".sengen_egui.path='{}'",
            self.0.display()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn フォルダが無ければ理由を示して止まる() {
        let 無いフォルダ = std::env::temp_dir().join("ClipViewer-xtask-無いSengenEgui");
        let 結果 = 手元のSengenEguiのフォルダ::見つける(
            Path::new("."),
            Some(無いフォルダ.into_os_string()),
        );
        assert!(結果.is_err_and(|理由| 理由.contains("Cargo.toml が無い")));
    }

    #[test]
    fn 既定ではリポジトリの一つ上のフォルダを探す() {
        let 結果 = 手元のSengenEguiのフォルダ::見つける(
            Path::new("存在しない親/ClipViewer"),
            None,
        );
        assert!(
            結果.is_err_and(|理由| 理由.contains("存在しない親") && 理由.contains("SengenEgui"))
        );
    }

    #[test]
    fn 差し替えの設定は取得元とフォルダを名指す() {
        let フォルダ = 手元のSengenEguiのフォルダ(PathBuf::from("C:\\devs\\SengenEgui"));
        assert_eq!(
            フォルダ.差し替えの設定(),
            "patch.\"https://github.com/megaDenryu/SengenEgui\".sengen_egui.path='C:\\devs\\SengenEgui'"
        );
    }
}
