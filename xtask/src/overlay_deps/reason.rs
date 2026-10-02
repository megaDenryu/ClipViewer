//! 解析できない理由。字句分け・パスの読み・パスの直しのどこで失敗しても、同じ型で行と説明を返す。

/// 解析できない理由とは、ソースを字句に分けられなかった、またはパスを読めなかった・直せなかった行と説明の組のことである。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct 解析できない理由 {
    pub 行: usize,
    pub 説明: String,
}

impl 解析できない理由 {
    pub fn 作成する(行: usize, 説明: &str) -> Self {
        Self {
            行,
            説明: 説明.to_string(),
        }
    }
}
