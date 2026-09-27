//! サムネイルを撮る ffmpeg の引数。

use std::ffi::OsString;

use clip_domain::サムネイルの撮り方;

use crate::read_position::コマを読む位置;
use crate::video_info::動画の情報;

/// JPEG の品質の指定(ffmpeg の -q:v。2が最も良く31が最も悪い)。一覧で小さく見る画像なので、容量を小さくする側へ寄せる。
/// 注意: この値やフィルタを変えたら、thumbnail_cache のキャッシュの形式の版を上げる(同じ撮り方で別の画像になるため)。
const JPEGの品質: u8 = 8;

/// JPEG の符号化器へ渡す画素の形式を、全範囲の YUV の3つに限るフィルタ。3つの中からは元の動画の色の間引き方に近いものを
/// ffmpeg が選ぶため、コマがあるときの画像は限らないときと同じになる。
/// 注意: 限らないと、FFmpeg 9 はコマが1つも来ないとき(時刻が動画の終わりより後ろ)に符号化器を狭い範囲の YUV で開こうとして
/// 異常終了し、「コマが無い」と区別できなくなる。4.4 はそのときも正常に終わる。
const 全範囲のYUVに限る: &str = "format=yuvj420p|yuvj422p|yuvj444p";

/// ffmpeg へ渡す引数を並べる。撮り方の時刻に映っているコマを、区間を溜めるデコードと流し読みと同じ規則(`コマを読む位置`)で選ぶ。
/// 読む映像の流れは、動画の情報が調べた流れを番号で指定する。1コマだけ出させる。
/// クロップは百分率を元の寸法への割合の式で渡す。左端+幅が100を超えるクロップ範囲(移植元の設定ファイルが作りうる)は、
/// はみ出さないよう左端と上端を寄せる。切り抜いた顔は、撮り方の大きさの枠に接するまで縦横比を保って拡大か縮小する。
pub(crate) fn サムネイルの引数を並べる(
    撮り方: &サムネイルの撮り方,
    動画: &動画の情報,
) -> Vec<OsString> {
    let 速さ = 動画.コマの速さ();
    let 読む位置 = コマを読む位置::求める(速さ, 速さ.時刻を含むコマ番号(撮り方.時刻()));
    let クロップ = 撮り方.クロップ();
    let 切り抜き = format!(
        r"crop=w=iw*{幅}:h=ih*{高さ}:x=min(iw*{左端}\,iw-ow):y=min(ih*{上端}\,ih-oh)",
        幅 = クロップ.幅().割合(),
        高さ = クロップ.高さ().割合(),
        左端 = クロップ.左端().割合(),
        上端 = クロップ.上端().割合(),
    );
    let 大きさ = 撮り方.大きさ();
    let 収める = format!(
        "scale=w={}:h={}:force_original_aspect_ratio=decrease",
        大きさ.幅(),
        大きさ.高さ()
    );
    let mut 引数: Vec<OsString> = ["-hide_banner", "-nostdin", "-loglevel", "error", "-ss"]
        .into_iter()
        .map(OsString::from)
        .collect();
    引数.push(読む位置.シーク位置の表記().into());
    引数.extend(["-i".into(), OsString::from(撮り方.動画のパス().文字列())]);
    引数.extend(["-map".into(), 動画.映像の流れ().読む流れの指定().into()]);
    引数.extend(["-an", "-sn", "-dn", "-frames:v", "1", "-vf"].map(OsString::from));
    let そろえる = 読む位置.コマをそろえるフィルタ();
    引数.push(format!("{そろえる},{切り抜き},{収める},{全範囲のYUVに限る}").into());
    引数.extend(["-q:v".into(), JPEGの品質.to_string().into()]);
    引数.extend(["-f", "image2pipe", "-c:v", "mjpeg", "-"].map(OsString::from));
    引数
}
