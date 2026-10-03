//! 左のサイドバーの幅の試験。置くクリップと置いたクリップに30文字ほどの長い名前があり、置けない理由の文が出ていても、
//! 名前と文を折り返して、サイドバーが既定の幅(280論理画素)のままはみ出さないことを、日本語フォントを当てた egui に描かせて確かめる。
//! スタックの作業場の同じ試験(`view/sidebar/stack_heading_width_tests.rs`)の流儀に倣う。
#![allow(clippy::expect_used)]

use clip_domain::{
    アスペクト比設定, クリップ, クリップスタック, クリップ名, クリップ識別子, 時間の長さ,
    重ね合わせ,
};
use eframe::egui;
use sengen_egui::日本語フォントの候補;

use super::screen_press_support::試験の全体の音量;
use crate::overlay::arrange_test_support::練習の動画;
use crate::overlay::command::置き方の操作;
use crate::overlay::placement_test_support::{
    置いたクリップの識別子を作る, 置き方の操作を当てる
};
use crate::overlay::sound_test_support::装置の無い作業場を作る;
use crate::overlay::state::library_status::重ね合わせのライブラリの様子;
use crate::overlay::state::placement::{
    置くクリップ, 置くクリップの一覧, 置くクリップの動画
};
use crate::overlay::state::今のスタックから並べる状況;
use crate::overlay::test_support::置いた連番の発行元;

const 長い名前: &str = "とても長い名前の振り付けのクリップ・サビの前の八呼間の練習用";

fn 日本語フォントを当てた本体を作る() -> egui::Context {
    let 画面描画の共有状態 = egui::Context::default();
    日本語フォントの候補::標準で入っている候補()
        .最初に読めたものを設定する(&画面描画の共有状態)
        .unwrap_or_else(|失敗| panic!("日本語フォントを読めないため、幅を測れない: {失敗}"));
    let _ = 画面描画の共有状態.run(egui::RawInput::default(), |_| {});
    画面描画の共有状態
}

fn 長い名前のクリップの重ね合わせ() -> 重ね合わせ {
    let 長いクリップ = クリップ::既定値で作成する(
        クリップ識別子::文字列から作成する("長".to_string()).expect("識別子"),
        クリップ名::作成する(長い名前.to_string()),
    );
    重ね合わせ::スタックから並べる(
        &クリップスタック::一覧から作成する(vec![長いクリップ]).expect("スタック"),
        練習の動画(),
        時間の長さ::作成する(60.0).expect("長さ"),
        アスペクト比設定::ワイド16対9,
        &mut 置いた連番の発行元::default(),
    )
    .expect("並べられる")
    .重ね合わせ
}

#[test]
fn 長い名前と置けない理由を折り返し_サイドバーは既定の幅のままである() {
    let mut 作業場 = 装置の無い作業場を作る();
    作業場.重ね合わせを開く(長い名前のクリップの重ね合わせ(), None);
    置き方の操作を当てる(
        &mut 作業場,
        置き方の操作::置いたクリップを選ぶ(置いたクリップの識別子を作る("置-1")),
    );
    let 一覧 = 置くクリップの一覧 {
        並び: vec![置くクリップ {
            識別子: クリップ識別子::文字列から作成する("長".to_string()).expect("識別子"),
            名前: クリップ名::作成する(長い名前.to_string()),
        }],
        動画: 置くクリップの動画::動画が違う,
    };
    let 画面描画の共有状態 = 日本語フォントを当てた本体を作る();
    for 回 in 0..2 {
        let _ = 画面描画の共有状態.run(egui::RawInput::default(), |画面描画の共有状態| {
            egui::CentralPanel::default().show(画面描画の共有状態, |ui| {
                let _ = super::画面(
                    作業場.状態(),
                    今のスタックから並べる状況::並べられる,
                    Some(&一覧),
                    試験の全体の音量,
                    None,
                    重ね合わせのライブラリの様子::書ける,
                )
                .描画して集める(ui);
            });
        });
        let 幅 = egui::containers::panel::PanelState::load(
            &画面描画の共有状態,
            egui::Id::new("重ね合わせのサイドバー"),
        )
        .expect("パネルの幅を記録した")
        .rect
        .width();
        assert_eq!(幅, 280.0, "{回}回目");
    }
}
