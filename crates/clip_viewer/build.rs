//! ビルドの前に動く手順。アプリのアイコン(`assets/icon/ClipViewer.ico`)を Windows のリソースとして実行ファイルへ埋め込み、
//! エクスプローラー・タスクバー・関連付けた動画のファイルの表示にアイコンを出す。版の情報(製品名と説明)も同じリソースに入れる。
//! Windows 以外へ向けたビルドでは何もしない。
//! リソースの組み立てには Windows SDK の rc.exe を使う(winresource が探す)。参照: README「利用ライブラリ」

use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let パッケージのフォルダ = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);
    let アイコン = パッケージのフォルダ
        .join("..")
        .join("..")
        .join("assets")
        .join("icon")
        .join("ClipViewer.ico");
    println!("cargo:rerun-if-changed={}", アイコン.display());
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return Ok(());
    }
    let アイコンの文字列 = アイコン
        .to_str()
        .ok_or("アイコンのパスが UTF-8 でない")?
        .to_string();
    let mut リソース = winresource::WindowsResource::new();
    リソース.set_icon(&アイコンの文字列);
    // 版の情報の名前は、パッケージの名(clip_viewer)でなくアプリの名にする。タスクマネージャーは FileDescription を出すためである。
    リソース.set("ProductName", "ClipViewer");
    リソース.set("FileDescription", "ClipViewer");
    リソース.compile()?;
    Ok(())
}
