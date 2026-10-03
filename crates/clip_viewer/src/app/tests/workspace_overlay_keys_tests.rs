//! 重ね合わせの作業場のキーの配線の試験。重ね合わせが前のとき、Ctrl+Z と Ctrl+Y は重ね合わせの編集を取り消してやり直し、Space は重ね合わせの再生を切り替え、
//! どれもスタックの作業場の値を変えないことを、画面の殻と同じ順の1フレームで確かめる。参照: _doc/設計/同時再生.md 2-8
#![allow(clippy::expect_used)]

use clip_domain::{アスペクト比設定, 重ねる画面の縦横比};
use eframe::egui;

use super::super::クリップビューアー;
use super::launch_requests_test_support::試験のビューアー;
use super::workspace_close_test_support::重ね合わせを開いて縦横比を変える;
use super::workspace_test_frames::{
    キーを押して適用する, 何も押さずに一フレーム進める
};
use super::workspace_test_snapshot::スタックの作業場の写し;
use super::workspace_test_support::前の重ね合わせの作業場;

fn 開いている重ね合わせの縦横比(
    ビューアー: &mut クリップビューアー,
) -> 重ねる画面の縦横比 {
    前の重ね合わせの作業場(ビューアー)
        .expect("重ね合わせが前")
        .状態()
        .開いている重ね合わせ()
        .expect("開いている")
        .重ねる画面の縦横比()
}

#[test]
fn 重ね合わせが前ならctrl_zとctrl_yで重ね合わせを取り消してやり直し_spaceで再生を切り替える() {
    let mut ビューアー = 試験のビューアー();
    重ね合わせを開いて縦横比を変える(&mut ビューアー);
    何も押さずに一フレーム進める(&mut ビューアー);
    let 前 = スタックの作業場の写し::写しを取る(&ビューアー);
    キーを押して適用する(&mut ビューアー, egui::Key::Z, egui::Modifiers::COMMAND);
    assert_eq!(
        開いている重ね合わせの縦横比(&mut ビューアー),
        重ねる画面の縦横比::ワイド16対9
    );
    キーを押して適用する(&mut ビューアー, egui::Key::Y, egui::Modifiers::COMMAND);
    assert_eq!(
        開いている重ね合わせの縦横比(&mut ビューアー),
        重ねる画面の縦横比::作成する(アスペクト比設定::正方形).expect("固定の比")
    );
    キーを押して適用する(&mut ビューアー, egui::Key::Space, egui::Modifiers::NONE);
    let 作業場 = 前の重ね合わせの作業場(&mut ビューアー).expect("重ね合わせが前");
    assert!(作業場.状態().再生しているか());
    assert_eq!(スタックの作業場の写し::写しを取る(&ビューアー), 前);
}
