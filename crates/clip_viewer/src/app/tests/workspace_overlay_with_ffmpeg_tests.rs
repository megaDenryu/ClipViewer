//! 「同時再生」から「今のスタックから並べる」を通して、同じ動画の3つのクリップが格子のます目に並んで映ることを確かめる結合試験。
//! スタックの作業場で動画を開き、配線の応答の経路で重ね合わせの作業場へ移って並べ、行のテクスチャにコマが載った後の重ねる画面に描く行を読む。
//! FFmpeg が要るため `#[ignore]` にしてあり、`cargo xtask verify` が FFmpeg を見つけたときだけ `--ignored` で流す。
#![allow(clippy::expect_used)]

use std::time::{Duration, Instant};

use clip_domain::{
    クリップ, クリップスタック, クリップ名, クリップ識別子, 入力された動画パス, 動画上の区間, 時刻,
};
use video_source::{FFmpegの置き場所の設定, FFmpegを置いたフォルダ};

use super::super::workspace_response::作業場の応答;
use super::launch_requests_test_support::試験のビューアー;
use super::workspace_test_support::{
    スタックの応答を適用する, 前の重ね合わせの作業場
};
use crate::command::{主ボタンの様子, 応答};
use crate::overlay::重ね合わせの作業場の応答;
use crate::state::library::開いたときの添え書き;
use crate::state::並びの出どころ;
use crate::video_feed::with_ffmpeg_support::{実行ファイルを探す, 試験動画を作る};

fn 一秒のクリップ(名前: &str, 開始: f64) -> クリップ {
    let mut クリップ = クリップ::既定値で作成する(
        クリップ識別子::文字列から作成する(名前.to_string()).expect("識別子"),
        クリップ名::作成する(名前.to_string()),
    );
    let 秒 = |値| 時刻::作成する(値).expect("時刻");
    クリップ.区間 = 動画上の区間::作成する(秒(開始), 秒(開始 + 1.0)).expect("区間");
    クリップ
}

#[test]
#[ignore = "FFmpeg が要る。cargo xtask verify が FFmpeg を見つけたときだけ流す"]
fn 同時再生から今のスタックから並べると_3つのクリップが格子のます目に並んで映る() {
    let 実行ファイル = 実行ファイルを探す();
    let フォルダ = 実行ファイル
        .変換のパス()
        .parent()
        .expect("フォルダ")
        .to_path_buf();
    let mut ビューアー = 試験のビューアー();
    ビューアー.状態.ffmpegの状況 =
        ビューアー
            .適用係
            .ffmpegを探す(&FFmpegの置き場所の設定::設定済み(
                FFmpegを置いたフォルダ::作成する(フォルダ),
            ));
    let 動画 = 試験動画を作る(&実行ファイル, "今のスタックから並べる");
    ビューアー.適用係.確かめてから動画を開く(
        &mut ビューアー.状態,
        &入力された動画パス::作成する(動画.文字列().to_string()),
        開いたときの添え書き::無し,
    );
    let 並び = クリップスタック::一覧から作成する(vec![
        一秒のクリップ("甲", 0.0),
        一秒のクリップ("乙", 1.0),
        一秒のクリップ("丙", 2.0),
    ])
    .expect("スタック");
    ビューアー
        .状態
        .並びを置き換える(並び, 並びの出どころ::新しく作った);
    スタックの応答を適用する(&mut ビューアー, vec![応答::重ね合わせの作業場へ移る]);
    let _ = ビューアー.応答を適用する(
        vec![作業場の応答::重ね合わせの応答(
            重ね合わせの作業場の応答::今のスタックから並べる,
        )],
        主ボタンの様子::押していない,
    );
    let 期限 = Instant::now() + Duration::from_secs(20);
    let 左上の並び = loop {
        ビューアー.フレームを進める(Instant::now());
        let 作業場 = 前の重ね合わせの作業場(&mut ビューアー).expect("重ね合わせが前");
        let 並び: Vec<(f64, f64)> = 作業場
            .状態()
            .開いている重ね合わせ()
            .expect("並べた重ね合わせを開いた")
            .重ねる画面に描く行の並び()
            .iter()
            .map(|行| (行.映す矩形.左端().数値(), 行.映す矩形.上端().数値()))
            .collect();
        if 並び.len() == 3 || Instant::now() > 期限 {
            break 並び;
        }
        std::thread::sleep(Duration::from_millis(20));
    };
    assert_eq!(左上の並び, [(0.0, 0.0), (50.0, 0.0), (0.0, 50.0)]);
}
