//! サムネイルのキャッシュの形式の版。要約値に混ぜる表記と、ファイル名に入れる印と、印から読み取った版と今の版との比べ方を持つ。

use std::cmp::Ordering;

/// ファイル名の印の頭の文字。印は `v4` のように、この文字と版の番号でできている。
const 印の頭: &str = "v";

/// キャッシュの形式の版とは、同じ撮り方から同じ画像ができる約束の番号のことである。撮る引数(ffmpeg の品質やフィルタ)を変えて
/// 同じ撮り方で別の画像になるときと、要約の求め方を変えたときに上げる。番号は要約値に混ざり、ファイル名にも印として入る。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub(crate) struct キャッシュの形式の版(u32);

/// 版の比べた結果とは、ファイル名から読み取った版が、今の版より古いか、今の版か、今の版より新しいかの区別のことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum 版の比べた結果 {
    古い,
    今の版,
    新しい,
}

impl キャッシュの形式の版 {
    /// このアプリが撮って置くサムネイルの版。
    pub(crate) const 今: Self = Self(4);

    /// 要約値に混ぜる表記。
    pub(crate) fn 要約に混ぜる表記(self) -> String {
        format!("ClipViewer.thumbnail.{}", self.0)
    }

    /// ファイル名の拡張子の前に置く印。
    pub(crate) fn ファイル名の印(self) -> String {
        format!("{印の頭}{}", self.0)
    }

    /// ファイル名の印から読み取る。印の形でなければ無い。
    pub(crate) fn ファイル名の印から読む(印: &str) -> Option<Self> {
        印.strip_prefix(印の頭)?.parse().ok().map(Self)
    }

    /// 今の版と比べる。
    pub(crate) fn 今の版と比べる(self) -> 版の比べた結果 {
        match self.cmp(&Self::今) {
            Ordering::Less => 版の比べた結果::古い,
            Ordering::Equal => 版の比べた結果::今の版,
            Ordering::Greater => 版の比べた結果::新しい,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 印から読み取った版を今の版と比べる() {
        let 読む = キャッシュの形式の版::ファイル名の印から読む;
        assert_eq!(
            読む("v3").map(キャッシュの形式の版::今の版と比べる),
            Some(版の比べた結果::古い)
        );
        assert_eq!(
            読む("v4").map(キャッシュの形式の版::今の版と比べる),
            Some(版の比べた結果::今の版)
        );
        assert_eq!(
            読む("v5").map(キャッシュの形式の版::今の版と比べる),
            Some(版の比べた結果::新しい)
        );
        assert_eq!(読む("x4"), None);
        assert_eq!(読む("v"), None);
        assert_eq!(キャッシュの形式の版::今.ファイル名の印(), "v4");
    }
}
