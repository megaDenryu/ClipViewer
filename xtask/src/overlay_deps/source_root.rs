//! ソースルート。検査する層のフォルダの下の `.rs` のファイルを集め、1つずつ字句に分けて禁じた参照を探す。
//! ファイルを読めない・UTF-8 でない・解析できないときは、そのファイルを外さずに見つけたこととして返す。

use std::path::{Path, PathBuf};

use super::findings::{検査の結果, 見つけたこと};
use super::layer::検査する層;
use super::module_path::モジュールパス;
use super::paths::禁じた参照を探す;
use super::reason::解析できない理由;
use super::tokens::字句に分ける;

/// ソースルートとは、`clip_viewer` のクレートの `src` フォルダのことである。モジュールパスはここから数える。
pub struct ソースルート(PathBuf);

impl ソースルート {
    pub fn リポジトリから決める(リポジトリルート: &Path) -> Self {
        Self(
            リポジトリルート
                .join("crates")
                .join("clip_viewer")
                .join("src"),
        )
    }

    #[cfg(test)]
    pub fn フォルダから作る(フォルダ: PathBuf) -> Self {
        Self(フォルダ)
    }

    /// 検査する層のフォルダの下の、すべての `.rs` のファイルを名前の順に調べる。フォルダを読めなければ失敗を返す。
    pub fn 層を検査する(&self, 層: &検査する層) -> Result<検査の結果, String> {
        let mut ファイルの並び = Vec::new();
        self.rsのファイルを集める(&self.0.join(層.フォルダの名前), &mut ファイルの並び)?;
        ファイルの並び.sort();
        let 見つけたことの並び = ファイルの並び
            .iter()
            .flat_map(|ファイル| self.ファイルを調べる(ファイル, 層.禁じたモジュール))
            .collect();
        Ok(検査の結果 {
            調べたファイルの数: ファイルの並び.len(),
            見つけたことの並び,
        })
    }

    fn rsのファイルを集める(
        &self,
        フォルダ: &Path,
        集まり: &mut Vec<PathBuf>,
    ) -> Result<(), String> {
        let 項目の並び = std::fs::read_dir(フォルダ)
            .map_err(|原因| format!("{} を読めない: {原因}", フォルダ.display()))?;
        for 項目 in 項目の並び {
            let パス = 項目
                .map_err(|原因| format!("{} を読めない: {原因}", フォルダ.display()))?
                .path();
            if パス.is_dir() {
                self.rsのファイルを集める(&パス, 集まり)?;
            } else if パス.extension().is_some_and(|拡張子| 拡張子 == "rs") {
                集まり.push(パス);
            }
        }
        Ok(())
    }

    fn ファイルを調べる(
        &self,
        ファイル: &Path,
        禁じたモジュール: &[&str],
    ) -> Vec<見つけたこと> {
        let 今のモジュールパス = モジュールパス::ファイルの位置から作る(
            &self.ソースルートからの位置(ファイル),
        );
        let 結果 = std::fs::read(ファイル)
            .map_err(|原因| 解析できない理由::作成する(0, &format!("読めない: {原因}")))
            .and_then(|中身| {
                String::from_utf8(中身)
                    .map_err(|_| 解析できない理由::作成する(0, "UTF-8 でない"))
            })
            .and_then(|本文| {
                禁じた参照を探す(
                    &字句に分ける(&本文)?,
                    &今のモジュールパス,
                    禁じたモジュール,
                )
            });
        match 結果 {
            Ok(参照の並び) => 参照の並び
                .into_iter()
                .map(|参照| 見つけたこと::禁じた参照(ファイル.to_path_buf(), 参照))
                .collect(),
            Err(理由) => vec![見つけたこと::解析できない(
                ファイル.to_path_buf(),
                理由,
            )],
        }
    }

    fn ソースルートからの位置(&self, ファイル: &Path) -> Vec<String> {
        ファイル
            .strip_prefix(&self.0)
            .unwrap_or(ファイル)
            .components()
            .map(|部分| 部分.as_os_str().to_string_lossy().into_owned())
            .collect()
    }
}
