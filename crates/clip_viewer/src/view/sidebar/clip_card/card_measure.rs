//! カードを描いて測る試験の道具。日本語フォントと画面のテーマを当てたカードを、サイドバーの中身に使える幅の中に描き、
//! カードを描いた幅と、見出しの行の押せる部品の矩形と、名前の欄の矩形を egui が記録した部品の矩形から読み取る。
//! 注意: 名前の欄は、押せてドラッグもできる部品のうち最も上にあるものとして見分ける。見出しの行で押せてドラッグもできる部品は名前の欄だけであり、
//! 下の行の秒の欄も同じ部品の種類のため、最も上であることで見分ける。

use clip_domain::{クリップ, クリップ名, クリップ識別子};
use eframe::egui;
use sengen_egui::日本語フォントの候補;

use super::{カードの様子, クリップカード};
use crate::view::styles;

/// サイドバーの内余白の左右の合計。
pub(super) const 内余白の左右: f32 = 24.0;

/// 名前の欄の枠の内側の余白の左右の合計(SengenEgui の入力欄の枠の内側の余白)。
pub(super) const 名前の欄の余白の左右: f32 = 8.0;

/// 描いたカードとは、カードを描いた幅と、見出しの行の押せる部品の矩形(egui が記録した順。Tab キーでフォーカスが移る順と同じ)と、名前の欄の矩形の組のことである。
pub(super) struct 描いたカード {
    pub(super) 幅: f32,
    pub(super) 見出しの行の部品: Vec<egui::Rect>,
    pub(super) 名前の欄: egui::Rect,
}

/// 日本語フォントとテーマを当て、サイドバーの中身に使える幅の中にカードを描いて測る。フォントは次のフレームから効くため2回描く。
pub(super) fn カードを描いて測る(
    サイドバーの幅: f32,
    様子: &カードの様子,
) -> 描いたカード {
    let eguiの本体 = egui::Context::default();
    日本語フォントの候補::標準で入っている候補()
        .最初に読めたものを設定する(&eguiの本体)
        .unwrap_or_else(|失敗| {
            panic!("日本語フォントを読めないため、カードの幅を測れない: {失敗}")
        });
    styles::画面のテーマ.適用する(&eguiの本体);
    let 識別子 = クリップ識別子::文字列から作成する("甲".to_string())
        .unwrap_or_else(|不正| panic!("識別子が不正: {不正}"));
    let クリップ = クリップ::既定値で作成する(
        識別子,
        クリップ名::作成する("Clip Mod 12 のコピー".to_string()),
    );
    let mut 幅 = 0.0;
    for _ in 0..2 {
        let _ = eguiの本体.run(egui::RawInput::default(), |eguiの本体| {
            egui::CentralPanel::default().show(eguiの本体, |ui| {
                let 大きさ = egui::vec2(サイドバーの幅 - 内余白の左右, 10_000.0);
                ui.allocate_ui(大きさ, |ui| {
                    let 始め = ui.cursor().min.x;
                    let _ = クリップカード(&クリップ, 様子).描画して集める(ui);
                    幅 = ui.min_rect().max.x - 始め;
                });
            });
        });
    }
    let (見出しの行の部品, 名前の欄) = 見出しの行を読み取る(&eguiの本体);
    描いたカード {
        幅,
        見出しの行の部品,
        名前の欄,
    }
}

/// egui が前の回に記録した部品の矩形から、見出しの行の押せる部品の矩形(egui が記録した順。Tab キーでフォーカスが移る順と同じ)と、名前の欄の矩形を読み取る。
fn 見出しの行を読み取る(eguiの本体: &egui::Context) -> (Vec<egui::Rect>, egui::Rect) {
    let 押せる部品: Vec<egui::WidgetRect> = eguiの本体.viewport(|ビューポート| {
        ビューポート
            .prev_pass
            .widgets
            .layers()
            .flat_map(|(_, 部品)| 部品.to_vec())
            .collect()
    });
    let 名前の欄 = 押せる部品
        .iter()
        .filter(|部品| 部品.sense == egui::Sense::click_and_drag())
        .map(|部品| 部品.rect)
        .min_by(|甲, 乙| 甲.top().total_cmp(&乙.top()))
        .unwrap_or_else(|| panic!("名前の欄が見つからない"));
    let 見出しの行の部品: Vec<egui::Rect> = 押せる部品
        .iter()
        .filter(|部品| 部品.sense.senses_click() || 部品.sense.senses_drag())
        .map(|部品| 部品.rect)
        .filter(|矩形| 矩形.height() <= 名前の欄.height() * 1.5)
        .filter(|矩形| 名前の欄.y_range().contains(矩形.center().y))
        .collect();
    (見出しの行の部品, 名前の欄)
}
