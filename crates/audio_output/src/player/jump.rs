//! 跳ぶ先。位置を合わせるときに、音を下げ切ってから移る読み位置と区間と行き先を預かり、下げ切ったら移る。

use audio_pcm::小数の標本位置;

use super::音の再生器;
use crate::instruction::再生の指示;
use crate::reading::速さを変える読み方;
use crate::retired::退いた出どころの置き場;
use crate::section::{終わりで続く音, 音の区間};

/// 跳ぶ先とは、音を下げ切ったら移る読み位置と、そこでの音の区間と行き先の組のことである。
#[derive(Debug, Clone)]
pub(super) struct 跳ぶ先 {
    位置: 小数の標本位置,
    区間: Option<音の区間>,
    行き先: 終わりで続く音,
}

impl<読み方: 速さを変える読み方> 音の再生器<読み方> {
    /// 跳ぶ先を決める。同じ区間と行き先へ跳ぶ先が既にあれば、位置だけを新しくし、置き場へ預けるものを増やさない。
    pub(super) fn 跳ぶ先を決める(
        &mut self,
        指示: &再生の指示,
        位置: 小数の標本位置,
        置き場: &mut 退いた出どころの置き場,
    ) {
        if let Some(今の先) = self.跳ぶ先.as_mut()
            && 音の区間::無い場合を含めて同じ中身か(
                今の先.区間.as_ref(),
                指示.区間.as_ref(),
            )
            && 今の先.行き先.同じ中身か(&指示.行き先)
        {
            今の先.位置 = 位置;
            return;
        }
        let 新しい先 = 跳ぶ先 {
            位置,
            区間: 指示.区間.clone(),
            行き先: 指示.行き先.clone(),
        };
        if let Some(古い先) = self.跳ぶ先.replace(新しい先) {
            置き場.区間を預かる(古い先.区間);
            置き場.行き先を預かる(Some(古い先.行き先));
        }
    }

    /// 跳ぶ先へ移る。音を下げ切った後に呼ぶ。
    pub(super) fn 跳ぶ(&mut self, 置き場: &mut 退いた出どころの置き場) {
        let Some(先) = self.跳ぶ先.take() else {
            return;
        };
        self.読み位置 = 先.位置;
        置き場.区間を預かる(std::mem::replace(&mut self.区間, 先.区間));
        置き場.行き先を預かる(self.行き先.replace(先.行き先));
    }
}
