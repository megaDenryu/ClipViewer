//! キーの一覧の窓。F1 で開き、もう一度 F1 か Escape で閉じる。参照: _doc/設計/画面.md 判断14「キーの割り当て」

/// キーの一覧の窓とは、キーの割り当ての一覧を出す窓を開いているかの区別のことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum キーの一覧の窓 {
    #[default]
    閉じている,
    開いている,
}

impl キーの一覧の窓 {
    /// 閉じていれば開き、開いていれば閉じる。
    pub(crate) fn 切り替える(&mut self) {
        *self = match self {
            Self::閉じている => Self::開いている,
            Self::開いている => Self::閉じている,
        };
    }
}
