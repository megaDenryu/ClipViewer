//! 音の供給と再生の指示の結合試験。音付きの試験動画を開き、先読みで区間の音を溜めた後の指示が、溜めた音の区間と
//! 区間の終わりの行き先を指すことと、音の無い動画・出力装置が無いときに鳴らさないことを確かめる。
//! FFmpeg が要るため `#[ignore]` にしてあり、`cargo xtask verify` が FFmpeg を見つけたときだけ `--ignored` で流す。
#![allow(clippy::expect_used)]

use std::time::{Duration, Instant};

use audio_output::終わりで続く音;
use audio_pcm::音の出どころ;
use clip_domain::{再生位置, 時刻};

use super::with_ffmpeg_sound_support::{
    二つのクリップの状態, 区間, 周波数, 溜めてあるか, 試験動画を作る, 開く,
};
use crate::audio_feed::動画の音;
use crate::stream_rules::流し読みの開き直し;
use crate::video_feed::with_ffmpeg_support::{成り立つまで待つ, 秒};

#[test]
#[ignore = "FFmpeg が要る。cargo xtask verify が FFmpeg を見つけたときだけ流す"]
fn 溜めた区間の音と区間の終わりの行き先を指示する() {
    let パス = 試験動画を作る("行き先", true);
    let mut 状態 = 二つのクリップの状態(開く(&パス, Some(周波数())));
    let (前, 後) = (区間(1.0, 2.0), 区間(3.0, 3.5));
    成り立つまで待つ("二つの区間の音を溜め終えること", || {
        状態.フレームを進める(Instant::now());
        溜めてあるか(&状態, &前) && 溜めてあるか(&状態, &後)
    });
    let 指示 = 状態.音の再生の指示(Instant::now());
    let 今の区間 = 指示.区間.expect("区間がある");
    assert_eq!(今の区間.範囲, 周波数().区間の範囲(&前));
    assert!(matches!(今の区間.出どころ, Some(音の出どころ::溜めた音(_))));
    assert!(
        matches!(指示.行き先, 終わりで続く音::開始へ戻る),
        "1周目の終わりは開始へ戻る"
    );
    assert_eq!(指示.位置.値(), 48_000.0);
    let 前の回数 = 指示.飛ばした回数;
    状態.再生.位置を移す(再生位置::シーケンス再生(
        時刻::作成する(1.5).expect("時刻"),
    ));
    let 指示 = 状態.音の再生の指示(Instant::now());
    assert_ne!(指示.飛ばした回数, 前の回数, "位置を移したら回数が進む");
    let 終わりで続く音::別の区間へ進む(次) = 指示.行き先 else {
        panic!("2周目の終わりは次の区間へ進む");
    };
    assert_eq!(次.範囲, 周波数().区間の範囲(&後));
    assert!(matches!(次.出どころ, Some(音の出どころ::溜めた音(_))));
    drop(状態);
    let _ = std::fs::remove_file(パス);
}

#[test]
#[ignore = "FFmpeg が要る。cargo xtask verify が FFmpeg を見つけたときだけ流す"]
fn 音の無い動画と出力装置の無いときは鳴らさない() {
    let パス = 試験動画を作る("音無し", false);
    let 動画 = 開く(&パス, Some(周波数()));
    assert!(matches!(動画.音, 動画の音::音が無い));
    let 状態 = 二つのクリップの状態(動画);
    assert!(状態.音の再生の指示(Instant::now()).区間.is_none());
    drop(状態);
    let 音付き = 試験動画を作る("装置無し", true);
    assert!(matches!(開く(&音付き, None).音, 動画の音::鳴らせない(_)));
    let _ = std::fs::remove_file(パス);
    let _ = std::fs::remove_file(音付き);
}

#[test]
#[ignore = "FFmpeg が要る。cargo xtask verify が FFmpeg を見つけたときだけ流す"]
fn 流し読みを開き直しても新しい方に標本が届くまで前の流し読みを鳴らす() {
    let パス = 試験動画を作る("開き直し", true);
    let mut 動画 = 開く(&パス, Some(周波数()));
    let 供給 = 動画.音.供給を書き換える().expect("音を鳴らせる");
    let 標本位置 = |秒数: f64| 周波数().時刻を含む標本位置(秒(秒数));
    let 始め = Instant::now();
    供給.流し読みを整える(秒(1.0), None, 流し読みの開き直し::してよい, 始め);
    成り立つまで待つ(
        "3.5秒の標本が最初の流し読みに届くこと",
        || {
            供給
                .鳴らす出どころ(None, 秒(1.0))
                .is_some_and(|出どころ| 出どころ.標本(標本位置(3.5)).is_some())
        },
    );
    let 前の出どころ = 供給.鳴らす出どころ(None, 秒(1.0)).expect("出どころ");
    let 後 = 始め + Duration::from_secs(1);
    供給.流し読みを整える(秒(3.5), None, 流し読みの開き直し::してよい, 後);
    assert!(
        !供給.今の流し読みに届いているか(秒(3.5)),
        "試験の前提が崩れた: 開き直した直後に新しい流し読みへ標本が届いていた(ffmpeg の起動が速すぎる)。前の流し読みを使う場面を作れない"
    );
    let 直後 = 供給.鳴らす出どころ(None, 秒(3.5)).expect("出どころ");
    assert!(
        直後.同じ入れ物か(&前の出どころ),
        "新しい方に届くまで前の流し読みを使う"
    );
    成り立つまで待つ(
        "新しい流し読みに3.5秒の標本が届くこと",
        || {
            供給.流し読みを整える(秒(3.5), None, 流し読みの開き直し::してよい, 後);
            供給
                .鳴らす出どころ(None, 秒(3.5))
                .is_some_and(|出どころ| !出どころ.同じ入れ物か(&前の出どころ))
        },
    );
    drop(動画);
    let _ = std::fs::remove_file(パス);
}
