//! 見えている行の経路の試験。ライブラリの構えの画面を描くと一覧の仮想縦スクロールが見えている行を応答として発し、
//! 応答の適用が状態へ置き、次のフレームのサムネイルの仕事が取り出す。画面は状態を書き換えない。
#![allow(clippy::expect_used)]

use eframe::egui;

use super::library_check::終えるまで待って当てる;
use super::library_support::試験のライブラリ;
use super::thumbnail_place::動画のあるスタックを置く;
use crate::command::{ライブラリの操作, 応答};
use crate::state::library::見えている行の範囲;
use crate::state::{アプリの状態, 画面の構え};
use crate::view;

/// 画面を1フレーム描き、発した応答を返す。
fn 画面を描いて集める(
    eguiの本体: &egui::Context, 状態: &アプリの状態
) -> Vec<応答> {
    let mut 集まり = Vec::new();
    let _ = eguiの本体.run(egui::RawInput::default(), |eguiの本体| {
        egui::CentralPanel::default().show(eguiの本体, |ui| {
            集まり = view::画面(状態).描画して集める(ui);
        });
    });
    集まり
}

fn 見えている行の知らせ(集まり: &[応答]) -> Vec<見えている行の範囲> {
    集まり
        .iter()
        .filter_map(|応答| match 応答 {
            応答::ライブラリ(ライブラリの操作::一覧の見えている行を知らせる(範囲)) => {
                Some(*範囲)
            }
            _ => None,
        })
        .collect()
}

#[test]
fn 画面が発した見えている行を適用で状態へ置き_サムネイルの仕事が取り出す() {
    let mut 試験 = 試験のライブラリ::作る("見えている行の経路");
    for 識別子 in ["stack-1-1", "stack-2-2", "stack-3-3"] {
        動画のあるスタックを置く(&試験, 識別子, None);
    }
    let mut 状態 = 試験.接続した状態(Vec::new());
    状態.一覧を読み直す();
    終えるまで待って当てる(&mut 状態);
    状態.出力.構え = 画面の構え::ライブラリ;
    let eguiの本体 = egui::Context::default();
    let mut 知らせ = Vec::new();
    for _ in 0..3 {
        let 集まり = 画面を描いて集める(&eguiの本体, &状態);
        知らせ = 見えている行の知らせ(&集まり);
        assert!(知らせ.len() <= 1, "1フレームに1回までしか知らせない");
        for 応答 in 集まり {
            試験.適用係.適用する(&mut 状態, 応答);
        }
    }
    let 全部の行 = 見えている行の範囲::両端の行番号から作る(0, 2);
    assert_eq!(知らせ, vec![全部の行], "3行とも見えている");
    assert_eq!(
        状態.ライブラリ.見えている行,
        Some(全部の行),
        "適用で状態へ置く"
    );
    状態.サムネイルを進める();
    assert_eq!(
        状態.ライブラリ.見えている行, None,
        "サムネイルの仕事が取り出す"
    );
}
