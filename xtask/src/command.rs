//! xtaskが受け付けるコマンドの語彙と、コマンド行引数の解釈と、使い方の表示。
//! 語彙と一覧表示を同じファイルに置くのは、コマンドを足すときに片方だけ更新する事故を避けるためである。

/// コマンドとは、`cargo xtask` が実行できる操作の区別のことである。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum コマンド {
    検証,
    層の依存の向きの検査,
    起動,
    インストーラー作成,
    ライセンス表示作成,
    /// デコードの負荷の測定。持つのは decode-load へ渡す引数である。
    デコードの負荷の測定(Vec<String>),
    /// 手元の SengenEgui へ差し替えた cargo の実行。持つのは cargo へ渡す引数である。
    手元のSengenEguiでcargoを実行(Vec<String>),
    /// local-sengen が内部で起こす Cargo.lock の見張り役。人が直接使うものではない。
    依存の固定ファイルの見張り役,
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
            "check-overlay-deps" => Self::実行する(コマンド::層の依存の向きの検査),
            "run" => Self::実行する(コマンド::起動),
            "installer" => Self::実行する(コマンド::インストーラー作成),
            "notices" => Self::実行する(コマンド::ライセンス表示作成),
            "decode-load" => Self::実行する(コマンド::デコードの負荷の測定(
                引数一覧[1..].to_vec(),
            )),
            "local-sengen" => Self::実行する(コマンド::手元のSengenEguiでcargoを実行(
                引数一覧[1..].to_vec(),
            )),
            crate::local_sengen::見張り役のコマンド名 => {
                Self::実行する(コマンド::依存の固定ファイルの見張り役)
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
        println!("  {名前:<26} {説明}");
    }
}

fn コマンド説明一覧() -> [(&'static str, &'static str); 8] {
    [
        (
            "verify",
            "cargo fmt --check → check-overlay-deps → cargo clippy --workspace --all-targets -- -D warnings → cargo test --workspace → FFmpeg の結合試験(video_source と clip_viewer。FFmpeg が見つかったときだけ) → 音声出力装置の確認(audio_output の例 device_check。装置が無ければ実行しなかったと表示する) を順に実行し、落ちたら止める",
        ),
        (
            "check-overlay-deps",
            "重ね合わせの作業場の層(crates/clip_viewer/src/overlay の下)が、スタックの作業場の crate::state・crate::command・crate::view を使えないことと、ライブラリの部品の共有の置き場(crates/clip_viewer/src/library_common の下)と取り消しの履歴の共有の置き場(crates/clip_viewer/src/edit_history の下)がそれに加えて crate::overlay・crate::app も使えないことを検査し、層ごとに調べたファイルの数を出す。クレートルートを * で全部取り込む書き方と、クレートルートに as で別名を付ける書き方も報告する。読めない・解析できないファイル(閉じていない波括弧を含む)も見つけたこととして報告する。extern crate self による別名も報告する。マクロが組み立てるパスと、#[path]・include! で読むファイルと、検査する層の外のモジュールが再公開したもの(crate::他::state のような経由)は調べない",
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
            "decode-load [<動画のパス>] [--streams <本数,...>] [--seconds <秒数>]",
            "同じ動画を n 本の ffmpeg で同時に、アプリの流し読みと同じ形(fps フィルタ・長辺1280画素・RGBA)で先頭から読み切る時間を測り、本数ごとの合計の速さ・1本あたりの速さ・再生の速さに対する倍率を表で出す。本数の既定は 1,2,4,8、秒数の既定は30。動画を渡さなければ ffmpeg の合成画像(testsrc2、1920x1080、30コマ/秒)を一時フォルダに作って測り、終わったら消す。FFmpeg は環境変数 CLIPVIEWER_FFMPEG_DIR → PATH の順に探す。時間がかかり結果が揺れるため verify には入れない",
        ),
        (
            "local-sengen <cargo の引数>",
            "push する前の手元の SengenEgui で試すときだけ使う。普段は SengenEgui を push して crates/clip_viewer/Cargo.toml の rev を上げる。cargo の --config でその実行の間だけ sengen_egui をリポジトリの1つ上の SengenEgui(環境変数 CLIPVIEWER_SENGEN_EGUI_DIR で変えられる)へ差し替えて cargo を実行し、終わったら(失敗・Ctrl+C でも)Cargo.lock を実行の前の中身に戻す。例: cargo xtask local-sengen run --package clip_viewer",
        ),
        (
            "local-sengen-guard",
            "local-sengen が内部で起こす Cargo.lock の見張り役。直接使わない",
        ),
    ]
}
