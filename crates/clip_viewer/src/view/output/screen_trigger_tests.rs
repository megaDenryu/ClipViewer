//! トリガー待ちの札の出方の試験。シアターでは札を操作の欄と同じ組で出し入れし、マウスを動かすか ← → を押すと出し、
//! 止めて2.5秒で隠す。編集の構えでは札をマウスに依らず出したままにする。待っていない間はどちらの構えでも出さない。
//! 出し入れの規則そのもの(欄の上・ドラッグの間・ウインドウの外)は SengenEgui の tests/overlay_idle*.rs が確かめる。
//! 各試験の最初の回は、重ねる子の大きさを測る回であり、札を描かないことがあるため、結果を見ない。

use eframe::egui;
use sengen_egui::画素の組;

use super::screen::出力の画面の材料;
use super::screen_test_support::試験の材料の選び方;
use crate::state::{左右の向き, 画面の構え};

const 札の見出し: &str = "トリガー待機中";

/// 出力の上の、札とも「編集へ戻る」とも重ならない位置。
const 出力の真ん中: egui::Pos2 = egui::pos2(400.0, 300.0);

/// 札の試験の場とは、1つの画面描画の共有状態の上で、時刻と出来事を変えながら同じ材料を描き続けるための値の組のことである。
struct 札の試験の場 {
    画面描画の共有状態: egui::Context,
    材料: 出力の画面の材料,
}

impl 札の試験の場 {
    fn 作成する(構え: 画面の構え, 待ちの案内: Option<&str>) -> Self {
        let 画面描画の共有状態 = egui::Context::default();
        let 材料 = 試験の材料の選び方 {
            構え,
            左右: 左右の向き::そのまま,
            待ちの案内: 待ちの案内.map(str::to_string),
        }
        .材料を作る(&画面描画の共有状態);
        Self {
            画面描画の共有状態,
            材料,
        }
    }

    /// 時刻(起動からの秒)と出来事を決めて1フレーム描き、札の見出しを描いたかを返す。
    fn 描いて札を出したか(&self, 秒: f64, 出来事一覧: Vec<egui::Event>) -> bool {
        let 入力 = egui::RawInput {
            time: Some(秒),
            events: 出来事一覧,
            ..Default::default()
        };
        let 出力 = self.画面描画の共有状態.run(入力, |画面描画の共有状態| {
            egui::CentralPanel::default().show(画面描画の共有状態, |ui| {
                let _ = self.材料.組む(画素の組(800.0, 600.0)).描画して集める(ui);
            });
        });
        出力.shapes.iter().any(|切り抜いた形| {
            matches!(&切り抜いた形.shape, egui::Shape::Text(文字の形) if 文字の形.galley.text() == 札の見出し)
        })
    }
}

fn マウスを動かす(右へ: f32) -> Vec<egui::Event> {
    vec![egui::Event::PointerMoved(
        出力の真ん中 + egui::vec2(右へ, 0.0),
    )]
}

fn キーの出来事(キー: egui::Key, 押した: bool) -> Vec<egui::Event> {
    vec![egui::Event::Key {
        key: キー,
        physical_key: None,
        pressed: 押した,
        repeat: false,
        modifiers: egui::Modifiers::NONE,
    }]
}

const 案内: Option<&str> = Some("Enterキーを押すか、Nextボタンで進めてください");

#[test]
fn シアターで待っている間はマウスを動かすと札が出て止めて2秒半で隠れる() {
    let 場 = 札の試験の場::作成する(画面の構え::シアター, 案内);
    let _ = 場.描いて札を出したか(0.0, マウスを動かす(0.0));
    assert!(場.描いて札を出したか(0.1, マウスを動かす(10.0)));
    assert!(場.描いて札を出したか(2.5, vec![]));
    assert!(!場.描いて札を出したか(2.7, vec![]));
    assert!(場.描いて札を出したか(3.0, マウスを動かす(20.0)));
}

#[test]
fn シアターで待っている間は左右キーを押すと札が出てほかのキーでは出ない() {
    let 場 = 札の試験の場::作成する(画面の構え::シアター, 案内);
    let _ = 場.描いて札を出したか(0.0, マウスを動かす(0.0));
    assert!(!場.描いて札を出したか(3.0, vec![]));
    assert!(!場.描いて札を出したか(3.1, キーの出来事(egui::Key::Enter, true)));
    assert!(場.描いて札を出したか(3.2, キーの出来事(egui::Key::ArrowRight, true)));
}

#[test]
fn 待っていない間はシアターでも編集の構えでも札を出さない() {
    for 構え in [画面の構え::シアター, 画面の構え::編集] {
        let 場 = 札の試験の場::作成する(構え, None);
        let _ = 場.描いて札を出したか(0.0, マウスを動かす(0.0));
        assert!(
            !場.描いて札を出したか(0.1, マウスを動かす(10.0)),
            "{構え:?}"
        );
    }
}

#[test]
fn 編集の構えではマウスを止めていても札を出したままにする() {
    let 場 = 札の試験の場::作成する(画面の構え::編集, 案内);
    let _ = 場.描いて札を出したか(0.0, vec![]);
    assert!(場.描いて札を出したか(0.1, vec![]));
    assert!(場.描いて札を出したか(10.0, vec![]));
}
