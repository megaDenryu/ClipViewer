//! 動画の形の試験。ffprobe の flat の出力から、アプリと同じ規則(添付の画像を除いた最初の映像の流れ・回転・コマの速さの選び方)で
//! 読むことを確かめる。参照: 規則の本体は crates/video_source/src/probe/stream_shape.rs の試験と同じ場合を見る。
#![allow(clippy::expect_used)]

use super::frame_size::画素の寸法;
use super::video_shape::動画の形;

fn 画素の寸法を作る(幅: u32, 高さ: u32) -> 画素の寸法 {
    画素の寸法::作成する(幅, 高さ).expect("作れる")
}

#[test]
fn 添付の画像を飛ばして最初の再生する映像の流れを読む() {
    let 出力 = "streams.stream.0.index=0\nstreams.stream.0.codec_type=\"video\"\n\
        streams.stream.0.width=600\nstreams.stream.0.height=600\nstreams.stream.0.avg_frame_rate=\"0/0\"\n\
        streams.stream.0.r_frame_rate=\"90000/1\"\nstreams.stream.0.disposition.attached_pic=1\n\
        streams.stream.1.index=1\nstreams.stream.1.codec_type=\"audio\"\n\
        streams.stream.2.index=2\nstreams.stream.2.codec_type=\"video\"\nstreams.stream.2.width=1920\n\
        streams.stream.2.height=1080\nstreams.stream.2.avg_frame_rate=\"30000/1001\"\n\
        streams.stream.2.r_frame_rate=\"60/1\"\nstreams.stream.2.disposition.attached_pic=0\r\n";
    let 形 = 動画の形::ffprobeの出力から読む(出力).expect("読める");
    assert_eq!(形.読む流れの指定(), "0:2");
    assert_eq!(形.表示される寸法(), 画素の寸法を作る(1920, 1080));
    assert_eq!(形.コマの速さ().分数の表記(), "30000/1001");
}

#[test]
fn 縦向きの回転の情報があれば幅と高さを入れ替えて縮める() {
    let 出力 = "streams.stream.0.index=0\nstreams.stream.0.codec_type=\"video\"\n\
        streams.stream.0.width=1920\nstreams.stream.0.height=1080\nstreams.stream.0.avg_frame_rate=\"30/1\"\n\
        streams.stream.0.tags.rotate=\"270\"\nstreams.stream.0.side_data_list.side_data.0.rotation=90\n";
    let 形 = 動画の形::ffprobeの出力から読む(出力).expect("読める");
    assert_eq!(形.表示される寸法(), 画素の寸法を作る(1080, 1920));
    assert_eq!(形.出力の寸法(), 画素の寸法を作る(720, 1280));
}

#[test]
fn 平均の速さが読めないか上限を超えるならr_frame_rateを使う() {
    let 平均がこれのとき = |平均: &str| {
        let 出力 = format!(
            "streams.stream.0.index=0\nstreams.stream.0.codec_type=\"video\"\nstreams.stream.0.width=640\n\
             streams.stream.0.height=360\nstreams.stream.0.avg_frame_rate=\"{平均}\"\nstreams.stream.0.r_frame_rate=\"25/1\"\n"
        );
        動画の形::ffprobeの出力から読む(&出力)
            .expect("読める")
            .コマの速さ()
            .分数の表記()
    };
    assert_eq!(平均がこれのとき("0/0"), "25/1");
    assert_eq!(平均がこれのとき("90000/1"), "25/1");
    assert_eq!(平均がこれのとき("5000000/83333"), "5000000/83333");
}

#[test]
fn 読めない出力は理由を付けて失敗にする() {
    let 映像の無い出力 = "streams.stream.0.index=0\nstreams.stream.0.codec_type=\"audio\"\n";
    let 寸法の無い出力 = "streams.stream.0.index=0\nstreams.stream.0.codec_type=\"video\"\n\
        streams.stream.0.avg_frame_rate=\"30/1\"\n";
    for (出力, 理由に含む語) in [
        ("", "映像の流れが無い"),
        ("width=1920\n", "行「width=1920」"),
        (映像の無い出力, "映像の流れが無い"),
        (寸法の無い出力, "寸法"),
    ] {
        let 理由 = 動画の形::ffprobeの出力から読む(出力).expect_err("失敗する");
        assert!(理由.contains(理由に含む語), "{出力}: {理由}");
    }
}
