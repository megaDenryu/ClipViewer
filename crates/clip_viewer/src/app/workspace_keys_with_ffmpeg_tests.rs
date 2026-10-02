//! 回帰試験(b)を、動画を開いてクリップを並べた状態で確かめる結合試験。動画が無いと効かないキー(I・O・←→・Space 等)の漏れを捕まえる。
//! FFmpeg が要るため `#[ignore]` にしてあり、`cargo xtask verify` が FFmpeg を見つけたときだけ `--ignored` で流す。
#![allow(clippy::expect_used)]

use clip_domain::入力された動画パス;
use video_source::{FFmpegの置き場所の設定, FFmpegを置いたフォルダ};

use super::launch_requests_test_support::試験のビューアー;
use super::workspace_test_keys::重ね合わせが前のときに全部のキーを押してもスタックの作業場が変わらないことを確かめる;
use super::workspace_test_support::スタックの応答を適用する;
use crate::command::{クリップの操作, 応答};
use crate::state::library::開いたときの添え書き;
use crate::video_feed::with_ffmpeg_support::{実行ファイルを探す, 試験動画を作る};

#[test]
#[ignore = "FFmpeg が要る。cargo xtask verify が FFmpeg を見つけたときだけ流す"]
fn 回帰試験b_動画を開いてクリップを並べ再生しているとき_重ね合わせが前ならスタックの作業場のキーを押しても変わらない()
 {
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
    let 動画 = 試験動画を作る(&実行ファイル, "作業場とキー");
    ビューアー.適用係.確かめてから動画を開く(
        &mut ビューアー.状態,
        &入力された動画パス::作成する(動画.文字列().to_string()),
        開いたときの添え書き::無し,
    );
    assert!(ビューアー.状態.動画.is_some(), "動画を開けた");
    スタックの応答を適用する(
        &mut ビューアー,
        vec![
            応答::クリップ(クリップの操作::追加する),
            応答::クリップ(クリップの操作::追加する),
        ],
    );
    assert!(ビューアー.状態.並び.一覧().len() >= 2, "クリップを並べた");
    ビューアー.状態.再生.再生を始める();
    重ね合わせが前のときに全部のキーを押してもスタックの作業場が変わらないことを確かめる(
        &mut ビューアー,
    );
}
