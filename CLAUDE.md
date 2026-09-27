# ClipViewer コーディングルール

グローバル規約(`~/.claude/CLAUDE.md`)に全面的に準拠する。本ファイルはこのリポジトリ固有の差分だけを定める。
層と依存の向きの正本は `_doc/設計/アーキテクチャ.md` である。

## このリポジトリは何か

ClipViewer は、1本の動画から切り出した短い区間(クリップ)を並べ、繰り返し・クロップ・キー待ちを付けて
再生するデスクトップアプリである。ブラウザ版(TypeScript + SengenUI)を Rust + egui(SengenEgui)へ移植している。
移植元は開発機の `%USERPROFILE%\python_dev\VoiroStudio\RefucteringVoiroStudio\app-ts\src\AppPage\ClipViewer\` にあり、
読むだけで書き換えない。

## 用語の正本

用語の正本は `crates/clip_domain` の rustdoc の定義文(「Xとは〜のことである」の形の文書コメント)である。
型定義がユビキタス言語の辞書を兼ねるため、別の用語集ファイルを置かない。新しい用語を使うときは、先にその型または
項目の文書コメントへ定義文を書く。

## 基底規約との差分

- **unsafe は全クレートで全面禁止**(`[workspace.lints.rust] unsafe_code = "forbid"`)。FFmpeg はライブラリとして
  リンクせず外部プロセスとして呼ぶため、unsafe を要する境界が無い
- **ツールはすべて `cargo xtask` から実行する**(実行場所はリポジトリのルート。引数なしで全コマンドの一覧を表示する)。アプリの起動は `cargo xtask run` である。
  ビルド・実行・検証のツールはすべて xtask クレートへ登録する。シェルスクリプトを散らさない。登録なきツール作成禁止
- **検証列は `cargo xtask verify`**(`cargo fmt --check` → `cargo clippy --workspace --all-targets -- -D warnings`
  → `cargo test --workspace` → FFmpeg の結合試験 → 音声出力装置の確認)。作業の区切りごとに実行し、全通過させてからコミットする。
  FFmpeg の結合試験(`crates/video_source/tests/with_ffmpeg/` と `crates/clip_viewer` の `video_feed/with_ffmpeg_tests.rs` と `state/with_ffmpeg_sound_tests.rs` と `state/with_ffmpeg_band_tests.rs` と `command/tests/with_ffmpeg_library.rs` と `command/tests/with_ffmpeg_thumbnail.rs` と `command/tests/with_ffmpeg_thumbnail_time.rs` と `command/tests/with_ffmpeg_edit.rs` と `command/tests/with_ffmpeg_unsaved.rs` と `command/tests/with_ffmpeg_open_note.rs` と `command/tests/with_ffmpeg_playback.rs`)は `#[ignore]` にしてあり、verify が環境変数
  `CLIPVIEWER_FFMPEG_DIR` → PATH の順に ffmpeg と ffprobe を探して、見つかったときだけ `--ignored` で流す。見つからなければ
  「実行しなかった」と表示して残りを続け、最終行が「検証列は FFmpeg の結合試験(FFmpeg が見つからない)を除いて通過した」になる。音声出力装置が無いときも同じく「音声出力装置の確認(音声出力装置が無い)を除いて」と並ぶ。除いた工程は未検証である(FFmpeg が PATH に無い開発機では、
  `CLIPVIEWER_FFMPEG_DIR` に ffmpeg.exe と ffprobe.exe のあるフォルダを渡す。例: `CLIPVIEWER_FFMPEG_DIR=C:\ffmpeg\bin`)
- **egui / SengenEgui への依存は `crates/clip_viewer` だけに閉じる。** `clip_domain` と `video_source` に書かない
- **FFmpeg への依存(外部プロセスの呼び出し)は `crates/video_source` だけに閉じる。** `clip_domain` に書かない
- **`clip_domain` は egui にも FFmpeg にも依存しない。** 依存してよいのは serde / serde_json / thiserror だけである
- **`video_source` は egui / SengenEgui にも cpal にも依存しない。** 依存してよいのは `clip_domain`・`audio_pcm` と serde / serde_json / thiserror だけであり、
  FFmpeg は `std::process` で起動する。`video_source` はクリップスタックやタイムライン・再生の規則を使わない(Cargo で強制できないため検収で確かめる)
- **音声出力装置(cpal)への依存は `crates/audio_output` だけに閉じる。** `audio_output` が依存してよいのは `clip_domain`・`audio_pcm`・cpal・thiserror だけであり、
  `video_source` を知らない。`audio_pcm`(音の値と受け渡しの入れ物)が依存してよいのは `clip_domain` だけである
- **`clip_library`(スタックのライブラリのフォルダの読み書き)が依存してよいのは `clip_domain` と thiserror だけである。** egui / SengenEgui / FFmpeg を知らない。
  試験は一時フォルダを置き場所として渡し、`%APPDATA%` の本物のフォルダを使わない(`_doc/設計/ライブラリ.md`)
- **`thumbnail_cache`(一覧のサムネイルのキャッシュの読み書き)が依存してよいのは `clip_domain` と thiserror だけである。** egui / SengenEgui / FFmpeg / `clip_library` を知らない。
  試験は一時フォルダを置き場所として渡し、`%LOCALAPPDATA%` の本物のフォルダを使わない(`_doc/設計/ライブラリ.md` 判断10)
- **音声のスレッド(cpal が呼ぶ関数)で、メモリの確保・ファイル・FFmpeg の起動をしない。** `Arc` の最後の参照もそこで捨てない(`_doc/設計/アーキテクチャ.md` 判断8-4)
- SengenEgui は GitHub の git 依存として決まった rev で取り込む(`crates/clip_viewer/Cargo.toml`)。SengenEgui を直したときは、SengenEgui を push してから
  rev を上げる。push の前の手元の SengenEgui(リポジトリの1つ上の `SengenEgui`)で試すときだけ `cargo xtask local-sengen <cargo の引数>` を使う。
  このコマンドは cargo の `--config` でその実行の間だけ差し替え、終わったら Cargo.lock を実行の前の中身に戻す。リポジトリ内やその上のフォルダの
  `.cargo/config.toml` に SengenEgui の差し替え(patch)を書かない(Cargo.lock が source の行の無い形に書き換わり、CI の `cargo fetch --locked` が止まる)。SengenEgui に口が足りないときは SengenEgui へ足し、
  利用側に素の egui 呼び出しを書いて回避しない
- 依存クレートの追加時は README の利用ライブラリ表へ採用理由を追記する
- lint は `unwrap_used` / `expect_used` / `as_conversions` を deny にしている。試験ファイルに限り、ファイル先頭で
  `expect_used` を局所的に許可してよい

## 文書配置

- `_doc/使い方.md`(利用者向けの説明書。扱いは下の「利用者向けの説明書」)
- `_doc/設計/`(生存型。実装と一致させる)
- 生存型文書の索引は README の「文書」節である。索引へ登録するまでが作成である

## 利用者向けの説明書

`_doc/使い方.md` は、アプリが利用者に対してどう振る舞うかの約束である。開発向けの文書(`_doc/設計/`)はコードに追従するが、この説明書は、オーナーが承認した後は、コードより上の正本になる。

1. `_doc/使い方.md` は、利用者から見た振る舞いの正本である
2. AI は、オーナーの決定なしに `_doc/使い方.md` の振る舞いの記述を変えない。振る舞いを変えない表記の直し(誤字・リンク)は行ってよく、そのときはコミットメッセージに振る舞いを変えないことを書く
3. コードと `_doc/使い方.md` が食い違ったら、コードを直す
4. 振る舞いを変える依頼を受けたら、AI は先に `_doc/使い方.md` の変更案をオーナーに示し、オーナーが決めてから実装する
5. `_doc/設計/画面.md` は「どう作るか」を書き、`_doc/使い方.md` と食い違ってはならない

条1〜4 は、オーナーが承認して `_doc/使い方.md` の冒頭から草案の印(「状態: 草案(オーナー未承認)」)を消した時点から効く。承認までは、`_doc/使い方.md` を今の振る舞いの写しとして扱い、コードと食い違ったら `_doc/使い方.md` を直す。
