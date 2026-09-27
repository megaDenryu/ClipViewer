//! 依存の固定ファイル(Cargo.lock)の読み書きと、作り直しで変わった行の確かめ。

use std::path::{Path, PathBuf};

/// 依存の固定ファイルとは、ワークスペースのルートにある Cargo.lock の本文のことである。
pub struct 依存の固定ファイル(String);

impl 依存の固定ファイル {
    pub fn 読む(フォルダ: &Path) -> Result<Self, String> {
        let パス = ファイルのパス(フォルダ);
        std::fs::read_to_string(&パス)
            .map(Self)
            .map_err(|原因| format!("{}: {原因}", パス.display()))
    }

    pub fn 書く(&self, フォルダ: &Path) -> Result<(), String> {
        let パス = ファイルのパス(フォルダ);
        std::fs::write(&パス, &self.0).map_err(|原因| format!("{}: {原因}", パス.display()))
    }

    pub fn 含むか(&self, 文字列: &str) -> bool {
        self.0.contains(文字列)
    }

    /// 差し替えていた依存の source の行(書き出しが一致する行)を除くと、元と行の並びがまったく同じかを確かめる。
    /// 行の集合でなく並びで比べるため、同じ行が増えた・減った・入れ替わった変更も見逃さない。
    /// 他の依存の版が動いていたら取り込まない。差し替えを外すこと以外の変更を、リリースの前に紛れ込ませないためである。
    pub fn 差し替えの行のほかは同じかを確かめる(
        &self,
        元: &Self,
        差し替えの行の書き出し: &str,
    ) -> Result<(), String> {
        let 差し替えの行を除く = |本文: &str| -> Vec<String> {
            本文
                .lines()
                .filter(|行| !行.starts_with(差し替えの行の書き出し))
                .map(str::to_string)
                .collect()
        };
        let 元の行 = 差し替えの行を除く(&元.0);
        let 今の行 = 差し替えの行を除く(&self.0);
        match 元の行.iter().zip(&今の行).position(|(元, 今)| 元 != 今) {
            None if 元の行.len() == 今の行.len() => Ok(()),
            違う位置 => {
                let 行目 = 違う位置.unwrap_or(元の行.len().min(今の行.len())) + 1;
                Err(format!(
                    "作り直した Cargo.lock で、差し替えていた依存の source の行の他にも変わった行がある(source の行を除いて {行目} 行目から)。取り込まずに止める"
                ))
            }
        }
    }
}

fn ファイルのパス(フォルダ: &Path) -> PathBuf {
    フォルダ.join("Cargo.lock")
}

#[cfg(test)]
mod tests {
    use super::依存の固定ファイル;

    const 印: &str = "source = \"git+https://example/x";

    fn 本文(行一覧: &[&str]) -> 依存の固定ファイル {
        依存の固定ファイル(行一覧.join("\n"))
    }

    #[test]
    fn 差し替えの行だけが増えたなら同じとみなす() {
        let 元 = 本文(&["[[package]]", "name = \"x\""]);
        let 今 = 本文(&[
            "[[package]]",
            "name = \"x\"",
            "source = \"git+https://example/x?rev=1\"",
        ]);
        assert!(今.差し替えの行のほかは同じかを確かめる(&元, 印).is_ok());
    }

    #[test]
    fn 同じ行が増えただけでも違うとみなす() {
        let 元 = 本文(&["[[package]]", "a", "[[package]]"]);
        let 今 = 本文(&["[[package]]", "a", "[[package]]", "a"]);
        assert!(今.差し替えの行のほかは同じかを確かめる(&元, 印).is_err());
    }
}
