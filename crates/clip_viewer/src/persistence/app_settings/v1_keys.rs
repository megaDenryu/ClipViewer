//! settings.json 第1版のキーの割り当ての項目(keyBindings)と、最新との両方向の変換。キーの組を文字列にするのは、この境界だけである。
//! 項目は、操作の名前(ファイル形式が決めている英語の名前)から、キーの組の表記(「Ctrl+Shift+Z」の形)の並びへの対応であり、既定と違う操作だけを書く。
//! 知らない操作の名前の行と、読めない表記・1つか2つでない並びの行は、その行だけを無いものとして読む。読んだ行どうしでキーが重なれば、項目の全体を無いものとして読む。
//! どちらも既定のキーになる(新しいアプリが同じ版に操作を足しても、ファイルを壊れたとみなさない)。参照: _doc/設計/ライブラリ.md「settings.json の形式」

use std::collections::BTreeMap;

use sengen_egui::キーの組;
use serde::{Deserialize, Serialize};

use crate::viewer_settings::{キーで行う操作, キーの割り当て, 操作のキー};

/// 第1版のキーの割り当てとは、settings.json 第1版の keyBindings の項目の中身のことである。名前の順に書く。
#[derive(Serialize, Deserialize, Default, PartialEq)]
#[serde(transparent)]
pub(super) struct 第1版のキーの割り当て(BTreeMap<String, Vec<String>>);

impl 第1版のキーの割り当て {
    /// 最新のキーの割り当てへ変換する。項目が無ければ既定である。
    pub(super) fn キーの割り当てへ変換する(
        項目: Option<Self>,
    ) -> キーの割り当て {
        let Some(Self(行)) = 項目 else {
            return キーの割り当て::既定();
        };
        let 変更 = 行.iter().filter_map(|(名前, 表記の並び)| {
            Some((名前から読む(名前)?, 表記の並びから読む(表記の並び)?))
        });
        キーの割り当て::保存した変更から作る(変更).unwrap_or(キーの割り当て::既定())
    }

    /// 最新のキーの割り当てから作る。既定と違う操作が無ければ、項目を書かない。
    pub(super) fn キーの割り当てから作る(
        割り当て: キーの割り当て
    ) -> Option<Self> {
        let 行: BTreeMap<String, Vec<String>> = 割り当て
            .既定と違う操作()
            .map(|(操作, キー)| {
                let 表記の並び = キー.組の並び().into_iter().map(キーの組::表記).collect();
                (名前にする(操作).to_string(), 表記の並び)
            })
            .collect();
        (!行.is_empty()).then_some(Self(行))
    }
}

/// 操作の名前。ファイル形式が決めており、名前を変えない(変えると古いアプリが読めない行になる)。
pub(super) fn 名前にする(操作: キーで行う操作) -> &'static str {
    match 操作 {
        キーで行う操作::再生と停止を切り替える => "togglePlayback",
        キーで行う操作::次のクリップへ進めるか知らせる => "nextClipOrNotify",
        キーで行う操作::前のクリップへ戻る => "previousClip",
        キーで行う操作::次のクリップへ進める => "nextClip",
        キーで行う操作::一コマ戻す => "stepFrameBackward",
        キーで行う操作::一コマ進める => "stepFrameForward",
        キーで行う操作::五秒戻す => "skipBackward5Seconds",
        キーで行う操作::五秒進める => "skipForward5Seconds",
        キーで行う操作::速度を一段下げる => "slowDown",
        キーで行う操作::速度を一段上げる => "speedUp",
        キーで行う操作::消音を切り替える => "toggleMute",
        キーで行う操作::シアターと編集を切り替える => "toggleTheater",
        キーで行う操作::全画面かシアターを抜ける => "leaveFullscreenOrTheater",
        キーで行う操作::全画面を切り替える => "toggleFullscreen",
        キーで行う操作::ライブラリへ保存する => "saveToLibrary",
        キーで行う操作::キーの一覧を開け閉めする => "toggleKeyList",
        キーで行う操作::編集を取り消す => "undo",
        キーで行う操作::編集をやり直す => "redo",
        キーで行う操作::開始を今の位置にする => "setStartToCurrentPosition",
        キーで行う操作::終了を今の位置にする => "setEndToCurrentPosition",
    }
}

/// 操作の名前から操作を読む。知らない名前なら無い。
fn 名前から読む(名前: &str) -> Option<キーで行う操作> {
    キーで行う操作::すべて
        .into_iter()
        .find(|操作| 名前にする(*操作) == 名前)
}

fn 表記の並びから読む(表記の並び: &[String]) -> Option<操作のキー> {
    let 組の並び = 表記の並び
        .iter()
        .map(|表記| キーの組::表記から読む(表記).ok())
        .collect::<Option<Vec<_>>>()?;
    操作のキー::組の並びから作る(&組の並び)
}
