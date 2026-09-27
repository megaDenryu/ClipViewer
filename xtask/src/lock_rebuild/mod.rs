//! `lock-without-patch` コマンド: SengenEgui の差し替え(開発機の `C:\devs\.cargo\config.toml` の patch)が効かない場所で
//! Cargo.lock を解き直し、リポジトリへ取り込む。
//! 差し替えが効く場所で cargo を動かすと、Cargo.lock の sengen_egui は source の行の無い(ローカルのフォルダを指す)形になり、
//! CI が git の rev から解き直した依存と一致しなくなる。配る実行ファイルと検証した依存を一致させるため、リリースの前にこのコマンドで作り直す。
//! 注意: 取り込んだ後に差し替えが効く場所で cargo を動かすと、Cargo.lock はまた差し替えの形へ戻る。取り込んだらすぐにコミットする。

mod copy;
mod lock_file;

use std::path::Path;
use std::process::Command;

use crate::verify::{リポジトリルートを求める, 工程を実行する};
use copy::作業の複製;
use lock_file::依存の固定ファイル;

/// 差し替えを外して作り直す依存の名。Cargo.toml の git 依存(rev 固定)であり、開発機ではローカルのフォルダへ差し替えている。
const 差し替えている依存: &str = "sengen_egui";

/// 作り直した Cargo.lock で、差し替えている依存が git から取られていることを示す source の行の書き出し。
const GITから取った印: &str = "source = \"git+https://github.com/megaDenryu/SengenEgui";

/// リポジトリを一時フォルダへ複製し、そこで cargo fetch を動かして Cargo.lock を解き直し、cargo check --locked で組めることを確かめてから
/// リポジトリへ取り込む。cargo fetch は Cargo.lock に合わない依存(差し替えの形の sengen_egui)だけを解き直し、他の依存の版は動かさない
/// (動いていたら取り込まずに止める)。
pub fn 差し替えなしで依存の固定ファイルを作り直す() -> Result<(), String> {
    let リポジトリルート = リポジトリルートを求める();
    作業ツリーが綺麗かを確かめる(&リポジトリルート)?;
    let 複製 = 作業の複製::一時フォルダへ作る(&リポジトリルート)?;
    println!("リポジトリを複製した: {}", 複製.フォルダ().display());
    工程を実行する(&["fetch"], 複製.フォルダ(), None)?;
    // ピン留めした rev の SengenEgui で組めるかを、差し替えの効かない複製の中で確かめる。出力先はリポジトリの target の下に固定し、
    // 次に実行したときに依存のビルドを使い回す。
    let 確かめる出力先 = リポジトリルート.join("target").join("lock-without-patch");
    工程を実行する(
        &["check", "--workspace", "--all-targets", "--locked"],
        複製.フォルダ(),
        Some(("CARGO_TARGET_DIR", 確かめる出力先.into_os_string())),
    )?;
    let 元 = 依存の固定ファイル::読む(&リポジトリルート)?;
    let 作り直し = 依存の固定ファイル::読む(複製.フォルダ())?;
    if !作り直し.含むか(GITから取った印) {
        return Err(format!(
            "作り直した Cargo.lock でも {差し替えている依存} が git から取られていない。複製先({})の上のフォルダか CARGO_HOME の config.toml に差し替えが無いかを確かめる",
            複製.フォルダ().display()
        ));
    }
    作り直し.差し替えの行のほかは同じかを確かめる(&元, GITから取った印)?;
    作り直し.書く(&リポジトリルート)?;
    println!(
        "Cargo.lock を取り込んだ。ここ(差し替えが効く場所)で cargo を動かす前にコミットする: git add Cargo.lock && git commit"
    );
    Ok(())
}

/// 作業ツリーにコミットしていない変更(追跡していないファイルを含む)があれば、理由を示して止める。
/// 複製して解き直し組めると確かめた状態と、取り込んだ Cargo.lock と一緒にコミットに残る状態を一致させるためである。
fn 作業ツリーが綺麗かを確かめる(
    リポジトリルート: &Path
) -> Result<(), String> {
    let 結果 = Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(リポジトリルート)
        .output()
        .map_err(|原因| format!("git の起動に失敗した: {原因}"))?;
    if !結果.status.success() {
        return Err(format!("git status が失敗した ({})", 結果.status));
    }
    let 変更 = String::from_utf8_lossy(&結果.stdout);
    if 変更.trim().is_empty() {
        Ok(())
    } else {
        Err(format!(
            "作業ツリーにコミットしていない変更がある。コミットするか退けてから流し直す(複製して確かめたものとコミットするものを一致させるため):
{変更}"
        ))
    }
}
