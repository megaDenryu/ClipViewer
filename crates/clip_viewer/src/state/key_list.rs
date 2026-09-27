//! キーの一覧のダイアログ。F1 で開き、もう一度 F1 か Escape で閉じる。参照: _doc/設計/画面.md 判断14「キーの割り当て」

/// キーの一覧のダイアログとは、キーの割り当ての一覧を出すダイアログを開いているかの区別のことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum キーの一覧のダイアログ {
    #[default]
    閉じている,
    開いている,
}

impl キーの一覧のダイアログ {
    /// 閉じていれば開き、開いていれば閉じる。
    pub(crate) fn 切り替える(&mut self) {
        *self = match self {
            Self::閉じている => Self::開いている,
            Self::開いている => Self::閉じている,
        };
    }
}
