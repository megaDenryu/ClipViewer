//! インストーラーに書く版。Inno Setup の VersionInfoVersion(実行ファイルの版の情報)は数字とピリオドだけを受けるため、
//! 版の文字列(将来の `0.2.0-beta.1` のような先行版を含む)から数字だけの版を別に取り出す。

/// アプリの版とは、ワークスペースの版(ルートの Cargo.toml の [workspace.package] version)の文字列のことである。
pub struct アプリの版(&'static str);

impl アプリの版 {
    /// xtask の版はワークスペースの版を継ぐため、xtask を組んだときの版がアプリの版である。
    pub fn ワークスペースの版() -> Self {
        Self(env!("CARGO_PKG_VERSION"))
    }

    /// 版の文字列そのもの。インストーラーの名と「アプリと機能」に出る版に使う。
    pub fn 文字列(&self) -> &'static str {
        self.0
    }

    /// 先行版の印(`-` 以降)とビルドの印(`+` 以降)を除いた、数字とピリオドだけの版。VersionInfoVersion に渡す。
    /// 例: `0.2.0-beta.1` は `0.2.0` になる。先行版と正式版が同じ数字の版になるが、利用者に見える版は文字列の方である。
    pub fn 数字だけの版(&self) -> Result<&'static str, String> {
        let 数字の部分 = self.0.split(['-', '+']).next().unwrap_or_default();
        let 区切り一覧: Vec<&str> = 数字の部分.split('.').collect();
        let 数字だけか = 区切り一覧
            .iter()
            .all(|区切り| !区切り.is_empty() && 区切り.bytes().all(|文字| 文字.is_ascii_digit()));
        if 数字だけか && (1..=4).contains(&区切り一覧.len()) {
            Ok(数字の部分)
        } else {
            Err(format!(
                "版「{}」から数字だけの版を取り出せない(1〜4個の数字をピリオドで区切った形が要る)",
                self.0
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::アプリの版;

    #[test]
    fn 正式版はそのまま数字だけの版になる() {
        assert_eq!(アプリの版("0.1.0").数字だけの版(), Ok("0.1.0"));
    }

    #[test]
    fn 先行版とビルドの印は数字だけの版から除く() {
        assert_eq!(アプリの版("0.2.0-beta.1").数字だけの版(), Ok("0.2.0"));
        assert_eq!(アプリの版("1.2.3+abc").数字だけの版(), Ok("1.2.3"));
    }

    #[test]
    fn 数字でない区切りや5個以上の区切りは受けない() {
        assert!(アプリの版("1.x.0").数字だけの版().is_err());
        assert!(アプリの版("1.2.3.4.5").数字だけの版().is_err());
        assert!(アプリの版("").数字だけの版().is_err());
    }
}
