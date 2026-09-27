//! 記録に書く日時。落ちたときの記録と警告の記録が読む時計はここ1箇所であり、書式の使い分けはこの値のメソッドで行う。

use chrono::{DateTime, Local};

/// 記録の日時とは、記録を書いた瞬間の、この計算機の時間帯での日時のことである。
pub(super) struct 記録の日時(DateTime<Local>);

impl 記録の日時 {
    /// 今の日時を時計から読む。
    pub(super) fn 今() -> Self {
        Self(Local::now())
    }

    /// ファイルの名に入れる形(例: `20260927-061342`)。名の並びが日時の順になり、Windows のファイル名に使えない文字を含まない。
    pub(super) fn ファイルの名に入れる形(&self) -> String {
        self.0.format("%Y%m%d-%H%M%S").to_string()
    }

    /// 落ちたときの記録の本文に書く形。時間帯の差まで含める(例: `2026-09-27T06:13:42.014626+09:00`)。
    pub(super) fn 本文に書く形(&self) -> String {
        self.0.to_rfc3339()
    }

    /// 警告の記録の1行の頭に書く形(例: `2026-09-27 06:13:42`)。
    pub(super) fn 行の頭に書く形(&self) -> String {
        self.0.format("%Y-%m-%d %H:%M:%S").to_string()
    }
}
