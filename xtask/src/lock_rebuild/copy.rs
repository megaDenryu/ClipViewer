//! 作業の複製。リポジトリの作業ツリーを、差し替えの設定が効かない一時フォルダ(%TEMP%)へ写す。
//! ビルドの出力(`target`)と Git の管理情報(`.git`)は写さない。cargo が依存を解くのに要るのはマニフェストとソースだけであるため。

use std::path::{Path, PathBuf};

/// 写さないフォルダの名。
const 写さないフォルダ: [&str; 2] = ["target", ".git"];

/// 作業の複製とは、一時フォルダへ写したリポジトリの作業ツリーのことである。落とすと一時フォルダを消す。
pub struct 作業の複製(PathBuf);

impl 作業の複製 {
    /// 一時フォルダの下の `ClipViewer-lock-without-patch` へ写す。前回の複製が残っていれば消してから写す。
    pub fn 一時フォルダへ作る(リポジトリルート: &Path) -> Result<Self, String> {
        let 複製先 = std::env::temp_dir().join("ClipViewer-lock-without-patch");
        if 複製先.exists() {
            std::fs::remove_dir_all(&複製先).map_err(|原因| 失敗の文(&複製先, 原因))?;
        }
        let 複製 = Self(複製先);
        複製.フォルダを写す(リポジトリルート, &複製.0)?;
        Ok(複製)
    }

    pub fn フォルダ(&self) -> &Path {
        &self.0
    }

    fn フォルダを写す(&self, 元: &Path, 先: &Path) -> Result<(), String> {
        std::fs::create_dir_all(先).map_err(|原因| 失敗の文(先, 原因))?;
        for 項目 in std::fs::read_dir(元).map_err(|原因| 失敗の文(元, 原因))? {
            let 項目 = 項目.map_err(|原因| 失敗の文(元, 原因))?;
            let 元の項目 = 項目.path();
            let 先の項目 = 先.join(項目.file_name());
            let 種類 = 項目.file_type().map_err(|原因| 失敗の文(&元の項目, 原因))?;
            if 種類.is_dir() {
                if !写さないフォルダ.iter().any(|名| 項目.file_name() == *名) {
                    self.フォルダを写す(&元の項目, &先の項目)?;
                }
            } else if 種類.is_file() {
                std::fs::copy(&元の項目, &先の項目)
                    .map_err(|原因| 失敗の文(&元の項目, 原因))?;
            }
        }
        Ok(())
    }
}

impl Drop for 作業の複製 {
    fn drop(&mut self) {
        let _消した = std::fs::remove_dir_all(&self.0);
    }
}

fn 失敗の文(パス: &Path, 原因: std::io::Error) -> String {
    format!("{}: {原因}", パス.display())
}
