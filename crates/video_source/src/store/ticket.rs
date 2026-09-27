//! 区間を溜める依頼の受付の札。

/// 受付の札とは、コマの倉庫が区間を溜める依頼を受け付けたときに返す、依頼を指し示す番号のことである。
/// 同じ倉庫の中で重複しない。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct 受付の札(u64);

/// 札の発行元とは、受付の札を重複しないよう順に発行するもののことである。
#[derive(Debug, Default)]
pub(crate) struct 札の発行元 {
    次の番号: u64,
}

impl 札の発行元 {
    /// 新しい札を発行する。
    pub(crate) fn 発行する(&mut self) -> 受付の札 {
        let 札 = 受付の札(self.次の番号);
        self.次の番号 = self.次の番号.saturating_add(1);
        札
    }
}
