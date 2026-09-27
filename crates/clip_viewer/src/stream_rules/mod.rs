//! 流し読みの規則。映像の流し読みと音の流し読みが共有する、開き直すかの判断と、開き直してよいかの区別と、開き直しの最短の間隔と、
//! 開き直しを判断できる流し読み(読んでいるか読めないか・開いた時刻・失敗の写し方)。
//! 映像の供給(`video_feed`)と音の供給(`audio_feed`)の両方が使い、どちらも知らない。参照: _doc/設計/画面.md 判断5

mod progress;
mod reopen;
mod slot;

#[cfg(test)]
pub(crate) use progress::流し読みの進み;
pub(crate) use reopen::{流し読みの開き直し, 開き直しの最短の間隔};
pub(crate) use slot::{流し読みの中身, 開き直しを判断できる流し読み};
