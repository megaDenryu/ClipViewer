//! 期限の試験。1つ目のアプリが応答しないときと、案内がまだ無いときに、送り手が期限を過ぎたら諦めることを確かめる。
#![allow(clippy::expect_used)]

use std::net::{Ipv4Addr, SocketAddr, TcpListener};
use std::time::{Duration, Instant};

use super::guide::{受け口の案内, 合言葉};
use super::sender::渡せなかった理由;
use super::test_support::一時の案内ファイル;
use super::{起動の頼み, 起動の頼みの送り手};

#[test]
fn 応答しない受け口には期限を過ぎたら諦める() {
    let (フォルダ, 案内ファイル) = 一時の案内ファイル("応答しない");
    let 受け付けない聞き手 =
        TcpListener::bind(SocketAddr::from((Ipv4Addr::LOCALHOST, 0))).expect("待てる");
    let ポート = 受け付けない聞き手.local_addr().expect("番地がある").port();
    案内ファイル
        .書く(&受け口の案内 {
            ポート,
            合言葉: 合言葉::作る(),
        })
        .expect("書ける");
    let 送り手 =
        起動の頼みの送り手::作成する(案内ファイル, Duration::from_millis(300));
    let 始め = Instant::now();
    let 結果 = 送り手.送る(&起動の頼み::前に出る);
    assert!(
        matches!(結果, Err(渡せなかった理由::やり取りできない(_))),
        "{結果:?}"
    );
    assert!(
        始め.elapsed() < Duration::from_secs(2),
        "期限を大きく過ぎて待たない"
    );
    let _ = std::fs::remove_dir_all(&フォルダ);
}

#[test]
fn 案内が無ければ期限まで待って諦める() {
    let (_フォルダ, 案内ファイル) = 一時の案内ファイル("案内が無い");
    let 送り手 =
        起動の頼みの送り手::作成する(案内ファイル, Duration::from_millis(250));
    let 始め = Instant::now();
    let 結果 = 送り手.送る(&起動の頼み::前に出る);
    assert!(
        matches!(結果, Err(渡せなかった理由::案内を読めない(_))),
        "{結果:?}"
    );
    assert!(
        始め.elapsed() >= Duration::from_millis(100),
        "期限までは渡し直す"
    );
}
