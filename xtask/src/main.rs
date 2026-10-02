//! xtaskのエントリポイント。このリポジトリのツールは、すべてこの xtask から実行する。

mod audio_device;
mod command;
mod ffmpeg_tests;
mod installer;
mod local_sengen;
mod overlay_deps;
mod run;
mod verify;

use command::{コマンド, 引数の解釈結果};

fn main() -> std::process::ExitCode {
    let 引数一覧: Vec<String> = std::env::args().skip(1).collect();
    match 引数の解釈結果::引数から解釈する(&引数一覧) {
        引数の解釈結果::使い方を表示する => {
            command::使い方を表示する();
            std::process::ExitCode::SUCCESS
        }
        引数の解釈結果::不明な引数(名前) => {
            eprintln!("不明なコマンド「{名前}」");
            command::使い方を表示する();
            std::process::ExitCode::FAILURE
        }
        引数の解釈結果::実行する(コマンド名) => {
            match コマンドを実行する(コマンド名) {
                Ok(()) => std::process::ExitCode::SUCCESS,
                Err(理由) => {
                    eprintln!("失敗: {理由}");
                    std::process::ExitCode::FAILURE
                }
            }
        }
    }
}

fn コマンドを実行する(コマンド名: コマンド) -> Result<(), String> {
    match コマンド名 {
        コマンド::検証 => verify::検証列を実行する(),
        コマンド::重ね合わせの層の依存の向きの検査 => {
            overlay_deps::依存の向きを検査する(&verify::リポジトリルートを求める())
        }
        コマンド::起動 => run::アプリを起動する(),
        コマンド::インストーラー作成 => installer::インストーラーを作る(),
        コマンド::ライセンス表示作成 => installer::ライセンス表示を作る(),
        コマンド::手元のSengenEguiでcargoを実行(cargoの引数) => {
            local_sengen::手元のsengen_eguiへ差し替えてcargoを実行する(&cargoの引数)
        }
        コマンド::依存の固定ファイルの見張り役 => {
            local_sengen::見張り役として待って書き戻す(
                &verify::リポジトリルートを求める(),
            )
        }
    }
}
