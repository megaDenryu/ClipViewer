//! xtaskが受け付けるコマンドの語彙と、コマンド行引数の解釈と、使い方の表示。
//! 語彙と一覧表示を同じファイルに置くのは、コマンドを足すときに片方だけ更新する事故を避けるためである。

/// コマンドとは、`cargo xtask` が実行できる操作の区別のことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum コマンド {
    検証,
    起動,
    インストーラー作成,
    ライセンス表示作成,
    差し替えなしの依存の固定ファイル作成,
}

/// 引数の解釈結果とは、コマンド行引数を解釈した結果の区別のことである。
pub enum 引数の解釈結果 {
    使い方を表示する,
    実行する(コマンド),
    不明な引数(String),
}

impl 引数の解釈結果 {
    /// コマンド行引数を解釈する。引数が無ければ使い方の表示を選ぶ。
    pub fn 引数から解釈する(引数一覧: &[String]) -> Self {
        let Some(名前) = 引数一覧.first() else {
            return Self::使い方を表示する;
        };
        match 名前.as_str() {
            "verify" => Self::実行する(コマンド::検証),
            "run" => Self::実行する(コマンド::起動),
            "installer" => Self::実行する(コマンド::インストーラー作成),
            "notices" => Self::実行する(コマンド::ライセンス表示作成),
            "lock-without-patch" => {
                Self::実行する(コマンド::差し替えなしの依存の固定ファイル作成)
            }
            _ => Self::不明な引数(名前.clone()),
        }
    }
}

/// 全コマンドの一覧と使い方を標準出力へ表示する。後続の作業者がツールを発見する唯一の手段である。
pub fn 使い方を表示する() {
    println!("cargo xtask <コマンド>  (実行場所: リポジトリのルート)");
    println!();
    println!("コマンド一覧:");
    for (名前, 説明) in コマンド説明一覧() {
        println!("  {名前:<18} {説明}");
    }
}

fn コマンド説明一覧() -> [(&'static str, &'static str); 5] {
    [
        (
            "verify",
            "cargo fmt --check → cargo clippy --workspace --all-targets -- -D warnings → cargo test --workspace → FFmpeg の結合試験(video_source と clip_viewer。FFmpeg が見つかったときだけ) → 音声出力装置の確認(audio_output の例 device_check。装置が無ければ実行しなかったと表示する) を順に実行し、落ちたら止める",
        ),
        (
            "run",
            "ClipViewer のアプリ(crates/clip_viewer)をビルドして起動する。FFmpeg は環境変数 CLIPVIEWER_FFMPEG_DIR → 保存した設定 → PATH の順に探す",
        ),
        (
            "installer",
            "アプリを release でビルドし(開発機のパスを実行ファイルから消す)、第三者のライセンス表示を作り、Inno Setup 6 の ISCC.exe で Windows のインストーラー(ClipViewer-<版>-setup.exe)をビルドの出力先の installer フォルダへ組み立てる。FFmpeg は同梱しない",
        ),
        (
            "notices",
            "cargo-about で第三者のライセンス表示(THIRD-PARTY-NOTICES.html)をビルドの出力先の installer フォルダへ作る。ライセンスを決められないクレートがあれば失敗する。cargo-about が要る: cargo install cargo-about --locked --features cli",
        ),
        (
            "lock-without-patch",
            "リポジトリを一時フォルダへ複製し、SengenEgui の差し替えが効かない場所で Cargo.lock の sengen_egui を git から解き直して取り込む。リリースの前に使い、取り込んだらすぐにコミットする",
        ),
    ]
}
