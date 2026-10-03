//! 再生の指示の受け口の試験の道具。配線が渡した指示を覚える受け口と、2本目の流れを開けなかったものとして開く手立てと、覚えた受け口で開く手立て。
//! 装置を開かずに、配線が1本目と2本目の流れへ渡した指示を観測するために使う。
#![allow(clippy::expect_used)]

use std::cell::RefCell;
use std::rc::Rc;

use audio_output::{再生の指示, 行ごとの再生の指示, 音声出力のエラー};
use audio_pcm::サンプリング周波数;

use super::super::instruction_receiver::{
    二本目の流れを開く手立て, 再生の指示の受け口, 行ごとの再生の指示の受け口,
};

/// 渡した指示の控えとは、受け口へ渡された指示を渡された順に覚えるものであり、試験が持つ写しと受け口が同じ並びを共有する。
#[derive(Clone)]
pub(in crate::app) struct 渡した指示の控え<指示>(Rc<RefCell<Vec<指示>>>);

impl<指示: Clone> 渡した指示の控え<指示> {
    pub(in crate::app) fn 空() -> Self {
        Self(Rc::new(RefCell::new(Vec::new())))
    }

    /// これまでに渡された指示の写し(渡された順)。
    pub(in crate::app) fn 渡された指示の並び(&self) -> Vec<指示> {
        self.0.borrow().clone()
    }

    fn 覚える(&self, 指示: 指示) {
        self.0.borrow_mut().push(指示);
    }
}

impl 再生の指示の受け口 for 渡した指示の控え<再生の指示> {
    fn 指示を渡す(&self, 指示: 再生の指示) {
        self.覚える(指示);
    }

    fn 使えなくなった理由(&self) -> Option<音声出力のエラー> {
        None
    }
}

impl 行ごとの再生の指示の受け口 for 渡した指示の控え<行ごとの再生の指示> {
    fn 指示を渡す(&self, 指示: 行ごとの再生の指示) {
        self.覚える(指示);
    }

    fn 使えなくなった理由(&self) -> Option<音声出力のエラー> {
        None
    }

    fn 周波数(&self) -> サンプリング周波数 {
        サンプリング周波数::作成する(48_000).expect("周波数")
    }
}

/// 開けなかったものとして開く手立てとは、装置を開かずに、持っている理由で開けなかったとする2本目の流れを開く手立てのことである。
pub(in crate::app) struct 開けなかったものとして開く(
    pub(in crate::app) 音声出力のエラー,
);

impl 二本目の流れを開く手立て for 開けなかったものとして開く {
    fn 開く(
        &self,
    ) -> Result<Box<dyn 行ごとの再生の指示の受け口>, 音声出力のエラー> {
        Err(self.0.clone())
    }
}

/// 控えへ開く手立てとは、装置を開かずに、渡した指示の控えを2本目の流れとして返す手立てのことである。
pub(in crate::app) struct 控えへ開く(pub(in crate::app) 渡した指示の控え<行ごとの再生の指示>);

impl 二本目の流れを開く手立て for 控えへ開く {
    fn 開く(
        &self,
    ) -> Result<Box<dyn 行ごとの再生の指示の受け口>, 音声出力のエラー> {
        Ok(Box::new(self.0.clone()))
    }
}
