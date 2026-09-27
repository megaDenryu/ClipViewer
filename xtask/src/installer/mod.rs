//! `installer` コマンド: アプリを release でビルドし、第三者のライセンス表示を作り、Inno Setup で Windows のインストーラー(setup.exe)を組み立てる。
//! `notices` コマンド: 第三者のライセンス表示だけを作る。
//! 定義は `installer/ClipViewer.iss`、成果物は `<ビルドの出力先>/installer/ClipViewer-<版>-setup.exe` である。
//! FFmpeg は同梱しない(利用者が入れ、アプリの画面で場所を指定する。README の利用者向けの節)。

mod build_output;
mod inno_setup;
mod notices;
mod release_flags;
mod version;

use crate::verify::{リポジトリルートを求める, 工程を実行する};
use build_output::ビルドの出力先;
use inno_setup::{InnoSetupのコンパイラ, インストーラーの材料};
use notices::ライセンス表示の作り手;
use release_flags::{
    リリースのビルドの指定, 指定を渡す環境変数, 置き換えるフォルダ
};
use version::アプリの版;

/// 第三者のライセンス表示を `<ビルドの出力先>/installer/THIRD-PARTY-NOTICES.html` へ作る。
pub fn ライセンス表示を作る() -> Result<(), String> {
    let ルート = リポジトリルートを求める();
    let 出力先 = ビルドの出力先::環境から決める(&ルート);
    let ファイル =
        ライセンス表示の作り手::探す(&ルート)?.作る(出力先.ライセンス表示のファイル())?;
    println!("第三者のライセンス表示を作った: {}", ファイル.display());
    Ok(())
}

/// ISCC.exe と cargo-about を先に探してから release のビルドを行い、ライセンス表示を作り、setup.exe を組み立てる。
/// 探すのを先にするのは、道具が無いと分かっている状態で長い release のビルドを待たせないためである。
pub fn インストーラーを作る() -> Result<(), String> {
    let ルート = リポジトリルートを求める();
    let コンパイラ = InnoSetupのコンパイラ::探す()?;
    let 表示の作り手 = ライセンス表示の作り手::探す(&ルート)?;
    let 版 = アプリの版::ワークスペースの版();
    let 数字だけの版 = 版.数字だけの版()?;
    let 指定 =
        リリースのビルドの指定::作成する(&置き換えるフォルダ::環境から読む(&ルート));
    工程を実行する(
        &["build", "--release", "--package", "clip_viewer"],
        &ルート,
        Some((指定を渡す環境変数, 指定.環境変数へ渡す値())),
    )?;
    let 出力先 = ビルドの出力先::環境から決める(&ルート);
    let ライセンス表示 = 表示の作り手.作る(出力先.ライセンス表示のファイル())?;
    let 置き場所 = 出力先.インストーラーの置き場所();
    コンパイラ.インストーラーを組み立てる(&インストーラーの材料 {
        定義ファイル: &ルート.join("installer").join("ClipViewer.iss"),
        版: 版.文字列(),
        数字だけの版,
        アプリの実行ファイル: &出力先.アプリの実行ファイル(),
        ライセンス: &ルート.join("LICENSE"),
        ライセンス表示: &ライセンス表示,
        アイコン: &ルート.join("assets").join("icon").join("ClipViewer.ico"),
        置き場所: &置き場所,
    })?;
    println!(
        "インストーラーを組み立てた: {}",
        置き場所
            .join(format!("ClipViewer-{}-setup.exe", 版.文字列()))
            .display()
    );
    Ok(())
}
