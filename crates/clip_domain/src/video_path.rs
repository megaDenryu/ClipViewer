//! 動画のパスとファイル名の型。利用者が入力したパスを正規化し、Windowsの絶対パスとして検証する。
//! 移植元 `Service/動画パス検証.ts` の移植である。

/// 入力された動画パスとは、利用者が入力欄へ書いた、正規化する前の動画のパスの文字列のことである。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct 入力された動画パス(String);

/// 正規化した動画パスとは、前後の空白と、前後で対になった引用符を取り除いた動画のパスのことである。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct 正規化した動画パス(String);

/// 動画ファイル名とは、動画のパスの最後の要素である、空でないファイル名のことである。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct 動画ファイル名(String);

impl 入力された動画パス {
    /// 入力欄の文字列から作成する。
    pub fn 作成する(入力: String) -> Self {
        Self(入力)
    }

    /// 入力欄の文字列そのものを返す。入力欄へ書き戻す境界で使う。
    pub fn 文字列(&self) -> &str {
        &self.0
    }

    /// 前後の空白を取り除き、前後が同じ種類の引用符で囲まれていればそれも取り除く。
    pub fn 正規化する(&self) -> 正規化した動画パス {
        let 前後空白除去 = self.0.trim();
        let 引用符で囲まれているか = |引用符: char| {
            前後空白除去.chars().count() >= 2
                && 前後空白除去.starts_with(引用符)
                && 前後空白除去.ends_with(引用符)
        };
        if 引用符で囲まれているか('"') || 引用符で囲まれているか('\'') {
            let 引用符の中 = &前後空白除去[1..前後空白除去.len() - 1];
            return 正規化した動画パス(引用符の中.trim().to_string());
        }
        正規化した動画パス(前後空白除去.to_string())
    }
}

impl 正規化した動画パス {
    /// パスの文字列を返す。ファイルを開く境界や設定ファイルへ渡す境界で使う。
    pub fn 文字列(&self) -> &str {
        &self.0
    }

    /// パスが空か。
    pub fn 空か(&self) -> bool {
        self.0.is_empty()
    }

    /// パスの最後の要素をファイル名として取り出す。区切りは `\` と `/` の両方である。
    pub fn ファイル名(&self) -> Option<動画ファイル名> {
        let 最後の要素 = self.0.rsplit(['\\', '/']).next().unwrap_or("");
        動画ファイル名::作成する(最後の要素.to_string())
    }

    /// 前後の引用符が釣り合っていないか。先頭と末尾の一方だけに引用符がある状態を指す。
    pub(crate) fn 前後の引用符が釣り合っていないか(&self) -> bool {
        let 片側だけか = |引用符: char| self.0.starts_with(引用符) != self.0.ends_with(引用符);
        片側だけか('"') || 片側だけか('\'')
    }

    /// ウィンドウズの絶対パスに見えるか。「C:\」「C:/」の形か、「\\サーバー\共有」の形を指す。
    pub(crate) fn ウィンドウズの絶対パスに見えるか(&self) -> bool {
        let 文字: Vec<char> = self.0.chars().collect();
        let 区切りか = |c: &char| *c == '\\' || *c == '/';
        let ドライブ文字の形か = 文字.len() >= 3
            && 文字[0].is_ascii_alphabetic()
            && 文字[1] == ':'
            && 区切りか(&文字[2]);
        ドライブ文字の形か || 共有フォルダの形か(&self.0)
    }
}

/// 「\\サーバー\共有」の形か。サーバー名と共有名はそれぞれ区切りを含まない1文字以上である。
fn 共有フォルダの形か(パス: &str) -> bool {
    let Some(残り) = パス.strip_prefix("\\\\") else {
        return false;
    };
    let mut 要素 = 残り.splitn(2, ['\\', '/']);
    let サーバー名 = 要素.next().unwrap_or("");
    let 共有名以降 = 要素.next().unwrap_or("");
    let 共有名 = 共有名以降.split(['\\', '/']).next().unwrap_or("");
    !サーバー名.is_empty() && !共有名.is_empty()
}

impl 動画ファイル名 {
    /// 文字列から作成する。空文字列なら作らない。
    pub fn 作成する(名前: String) -> Option<Self> {
        (!名前.is_empty()).then_some(Self(名前))
    }

    /// ファイル名の文字列を返す。表示や設定ファイルへ渡す境界で使う。
    pub fn 文字列(&self) -> &str {
        &self.0
    }

    /// 大文字と小文字を区別せずに同じ名前か。
    pub fn 大文字小文字を区別せず等しいか(&self, 他方: &Self) -> bool {
        self.0.to_lowercase() == 他方.0.to_lowercase()
    }

    /// 最後の点より前の名前(拡張子を除いた名前)。点が無ければファイル名そのもの。登録する名前の初期値に使う。
    pub fn 拡張子を除いた名前(&self) -> &str {
        self.0.rsplit_once('.').map_or(&self.0, |(幹, _)| 幹)
    }
}
