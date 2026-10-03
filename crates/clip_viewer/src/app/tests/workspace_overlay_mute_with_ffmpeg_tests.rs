//! 全体の消音の結合試験。スタックの作業場で音付きの動画を開いて「同時再生」から並べて再生し、2本目の流れへ渡した行の指示の音量が、
//! スタックの作業場で消音すると0になることを、配線の経路(重ね合わせの側が全体の鳴らす音量を選んで渡すこと)を一続きに通して確かめる。
//! 2本目の流れは渡した指示を覚える受け口に差し替え、装置は開かない。FFmpeg が要るため `#[ignore]` にしてある。
#![allow(clippy::expect_used)]

use std::time::{Duration, Instant};

use audio_output::行ごとの再生の指示;
use clip_domain::{クリップスタック, 音量};

use super::super::workspace_response::作業場の応答;
use super::instruction_receiver_test_support::{控えへ開く, 渡した指示の控え};
use super::launch_requests_test_support::試験のビューアーを二本目の流れの手立てで作る;
use super::workspace_ffmpeg_test_support::{
    区間を決めたクリップを作る, 試験のffmpegを見つけて動画を開く,
};
use crate::command::{主ボタンの様子, 再生の操作, 応答};
use crate::overlay::sound_test_support::音付きの試験動画;
use crate::overlay::{重ね合わせの作業場の応答, 重ね合わせの操作};
use crate::state::並びの出どころ;

fn 当てる(
    ビューアー: &mut super::super::クリップビューアー, 応答一覧: Vec<作業場の応答>
) {
    let _ =
        ビューアー.応答を適用する(応答一覧, 主ボタンの様子::押していない, Instant::now());
}

#[test]
#[ignore = "FFmpeg が要る。cargo xtask verify が FFmpeg を見つけたときだけ流す"]
fn スタックの作業場で消音すると_重ね合わせの行へ渡す音量が0になる() {
    let 二本目 = 渡した指示の控え::<行ごとの再生の指示>::空();
    let mut ビューアー = 試験のビューアーを二本目の流れの手立てで作る(
        Box::new(控えへ開く(二本目.clone())),
    );
    let 動画 = 音付きの試験動画::作る("全体の消音");
    試験のffmpegを見つけて動画を開く(&mut ビューアー, &動画.0);
    let 並び = クリップスタック::一覧から作成する(vec![区間を決めたクリップを作る(
        "甲", 0.5, 1.5,
    )])
    .expect("スタック");
    ビューアー
        .状態
        .並びを置き換える(並び, 並びの出どころ::新しく作った);
    当てる(
        &mut ビューアー,
        vec![作業場の応答::スタックの応答(
            応答::重ね合わせの作業場へ移る,
        )],
    );
    当てる(
        &mut ビューアー,
        vec![
            作業場の応答::重ね合わせの応答(
                重ね合わせの作業場の応答::今のスタックから並べる,
            ),
            作業場の応答::重ね合わせの応答(重ね合わせの作業場の応答::操作(
                重ね合わせの操作::再生を切り替える,
            )),
        ],
    );
    let 期限 = Instant::now() + Duration::from_secs(20);
    let 鳴らしている = loop {
        ビューアー.フレームを進める(Instant::now());
        let 奥 = 二本目
            .最後に渡された行の指示(0)
            .filter(|指示| 指示.区間.is_some());
        if 奥.is_some() || Instant::now() > 期限 {
            break 奥.expect("奥の行の音の区間を渡す");
        }
        std::thread::sleep(Duration::from_millis(20));
    };
    let 鳴らす音量 = ビューアー.状態.再生.音量.鳴らす音量();
    assert_ne!(鳴らす音量, 音量::無音, "試験の前提: 消音していない");
    assert_eq!(
        鳴らしている.音量, 鳴らす音量,
        "置いたクリップの音量1に全体の鳴らす音量を掛ける"
    );
    ビューアー
        .状態
        .再生の操作を適用する(再生の操作::消音を切り替える);
    ビューアー.フレームを進める(Instant::now());
    let 消した後 = 二本目.最後に渡された行の指示(0).expect("渡した");
    assert_eq!(
        消した後.音量,
        音量::無音,
        "スタックの作業場の消音(M キーと同じ操作)で行の音量は0になる"
    );
}
