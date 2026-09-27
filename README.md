# ClipViewer

ClipViewer は、1本の動画から切り出した短い区間(クリップ)を順に並べ、区間ごとに繰り返しの回数・クロップ(切り出す矩形)・
次へ進む条件(自動・Enterキー・Spaceキー・クリック)を付けて再生する Windows のデスクトップアプリである。
Space は再生と停止に使うため、Spaceキー待ちのクリップは Enter で進む。キーの一覧は F1 で出る(`_doc/使い方.md` 12節)。

## 使う人へ

インストール・FFmpeg の入れ方・画面の見かた・キー操作・データの置き場所・アンインストールは、[使い方の説明書(_doc/使い方.md)](_doc/使い方.md)に書いてある。

- 動作環境: Windows 10 または Windows 11 の x64(64ビット)版。ClipViewer とは別に FFmpeg が要る(入れ方は説明書の2.3節)。
- 入手先: [Releases](https://github.com/megaDenryu/ClipViewer/releases) の `ClipViewer-<版>-setup.exe`。

## 開発する人へ

クリップの並びは JSON の設定ファイルに保存し、ブラウザ版 ClipViewer(v4.0 形式)と双方向で読み書きできる。
作ったスタックは名前を付けてアプリの中のライブラリ(`%APPDATA%\ClipViewer\library\`)へ保存でき、一覧から開き直して見返し・編集できる。
ライブラリに保存するのは動画のパスとクリップの並びだけであり、動画は写さない。一覧に出す顔のサムネイルは、捨ててよいキャッシュ(`%LOCALAPPDATA%\ClipViewer\thumbnails\`)に置く(設計は `_doc/設計/ライブラリ.md`)。

ブラウザ版(TypeScript + SengenUI)から Rust + egui(SengenEgui)への移植の途中である。移植の動機は2つある。
(1) ブラウザではローカルの動画ファイルの扱いが不便である。
(2) 短い区間の繰り返しがブラウザの `<video>` のシーク性能に依存し、継ぎ目で詰まる。

### ソースからビルドする

Rust(rustup)と Windows SDK を入れ、リポジトリを取得してルートで次を実行する。Windows SDK は、実行ファイルへアイコンを埋め込む rc.exe のために要る(Visual Studio の「C++ によるデスクトップ開発」のビルドツールに含まれる。無いとビルドが失敗する)。FFmpeg は別に入れる(`_doc/使い方.md` 2.3節)。Rust の版はルートの `rust-toolchain.toml`(1.94.0)で固定してあり、rustup が自動で取ってくる。

```
git clone https://github.com/megaDenryu/ClipViewer
cd ClipViewer
cargo xtask run         # 開発のビルドで起動する
cargo xtask installer   # インストーラーを組み立てる(Inno Setup 6 と cargo-about が要る: winget install JRSoftware.InnoSetup と cargo install cargo-about --locked --features cli)
```

### 確かめた FFmpeg の版

ClipViewer の動作を確かめた FFmpeg の版は、4.4 系(4.4.1)と 9.0 系(9.0.2。gyan.dev の essentials)である。今から入れるなら 9.0 系でよい。
GitHub Actions の検証列は choco で 9.0.2 に固定して入れる(`.github/workflows/release.yml`)。版を上げるときは、その版で `cargo xtask verify` を通してから
release.yml の版と、この節と、`_doc/使い方.md` 2.3節の版を一緒に直す。

### 移植の段階

| 段階 | 内容 | 状態 |
|---|---|---|
| 第1段階 | `crates/clip_domain`(型と規則)と、リポジトリの土台と設計文書 | 完了(検収の指摘を是正済み) |
| 第2段階 | SengenEgui への必要な口の追加(このリポジトリの外、`C:\devs\SengenEgui` で行った) | 完了(rev e368c92) |
| 第3段階 | `crates/video_source`(FFmpeg を外部プロセスとして呼ぶ境界。裏のスレッドでのデコード、溜めたコマを管理するコマの倉庫、上限を超える区間と元動画再生の流し読み) | 完了 |
| 第4段階 | `crates/clip_viewer`(画面・再生の時計・操作の応答・映像の供給。設計は `_doc/設計/画面.md`) | 試用可能(機械検証済み。画面の操作はオーナーの試用待ち) |
| 第5段階 | 音声(`crates/audio_pcm`・`crates/audio_output`、`video_source` の音の読み出し、`clip_viewer` の音の供給。設計は `_doc/設計/アーキテクチャ.md` 判断8) | 試用可能(機械検証済み。耳での確認はオーナーの試用待ち) |

### 検証と実行

すべてのツールは `cargo xtask` から使う。実行場所はリポジトリのルートである。

```
cargo xtask          # コマンドの一覧を表示する
cargo xtask verify   # fmt --check → clippy -D warnings → test → FFmpeg の結合試験 → 音声出力装置の確認 を順に実行する
cargo xtask run      # アプリを開発のビルドで起動する
cargo xtask installer  # アプリを release でビルドし、第三者のライセンス表示を作り、Windows のインストーラーを組み立てる
cargo xtask notices    # 第三者のライセンス表示(THIRD-PARTY-NOTICES.html)だけを作る
cargo xtask local-sengen <cargo の引数>  # push する前の手元の SengenEgui で試すときだけ使う(下の「SengenEgui を直したとき」)
```

### SengenEgui を直したとき

`crates/clip_viewer` は SengenEgui を GitHub の git 依存として、決まった rev で取り込む(`crates/clip_viewer/Cargo.toml`)。
Cargo.lock はその rev を指す形のままコミットする。SengenEgui を直したときの普段の流れは、SengenEgui を push してから
`crates/clip_viewer/Cargo.toml` の `rev` を上げ、`cargo xtask verify` を通して、Cargo.toml と Cargo.lock を一緒にコミットすることである。

**push の前の手元の SengenEgui で試すときだけ** `cargo xtask local-sengen <cargo の引数>` を使う(実行場所はリポジトリのルート)。
普段は使わず、rev を上げる。このコマンドは cargo の `--config` で、その実行の間だけ `sengen_egui` を手元のフォルダへ差し替えて cargo を実行する。
手元のフォルダの既定はリポジトリの1つ上の `SengenEgui` であり、環境変数 `CLIPVIEWER_SENGEN_EGUI_DIR` で変えられる。フォルダが無ければ理由を示して止まる。

```
cargo xtask local-sengen run --package clip_viewer    # 手元の SengenEgui でアプリを起動する
cargo xtask local-sengen test --workspace             # 手元の SengenEgui で試験を流す
cargo xtask local-sengen clippy --workspace --all-targets -- -D warnings
```

差し替えた cargo は Cargo.lock の `sengen_egui` から source の行を消すため、このコマンドは実行の前の Cargo.lock の中身を別のプロセス(見張り役)に覚えさせ、
cargo が終わったら書き戻す。失敗して終わったときと、Ctrl+C で止めたときも書き戻す(`xtask/src/local_sengen/guard.rs`)。
`cargo xtask verify` のような xtask のコマンドは中で別の cargo を起こし、そこへ差し替えが届かないため、`local-sengen` の引数には渡せない(渡すと理由を示して止まる)。
リポジトリの中の `.cargo/config.toml` や、リポジトリの上のフォルダの `.cargo/config.toml` に SengenEgui の差し替え(patch)を書かない。
書くと、その下で動かしたすべての cargo が Cargo.lock を書き換え、CI の `cargo fetch --locked` が止まる形の Cargo.lock をコミットしうるためである。

`crates/video_source` の結合試験(`tests/with_ffmpeg/`)は FFmpeg を実際に起動するため、`#[ignore]` にしてあり
`cargo test --workspace` では流れない。`cargo xtask verify` は最後の工程で、環境変数 `CLIPVIEWER_FFMPEG_DIR` → PATH の順に
ffmpeg と ffprobe が両方あるフォルダを探し、見つかればその場所を `CLIPVIEWER_FFMPEG_DIR` に入れて `--ignored` で流す。
探す順と、ffmpeg と ffprobe が同じフォルダにそろう場所だけを選ぶ規則は `video_source` の探索の口と同じである。
xtask は `video_source` に依存せず標準ライブラリだけで判定し、実際に使う組は結合試験の側が `video_source` の口で探し直す。
見つからなければ「FFmpeg の結合試験を実行しなかった」と理由と手順を表示し、最終行を
「検証列は FFmpeg の結合試験(FFmpeg が見つからない)を除いて通過した」にして終える。最終行が「検証列はすべて通過した」のときだけ、
結合試験と音声出力装置の確認まで通っている。

音声出力装置の確認は、`crates/audio_output` の例(`examples/device_check.rs`)を実行し、既定の出力装置を開いて音量0で0.3秒鳴らし、
装置が音を求めたことを確かめる。装置が無ければ例は終了コード3で終わり、verify は「音声出力装置の確認を実行しなかった」と表示して、
最終行の「除いて通過した」に音声出力装置の確認を並べる。音の再生器(継ぎ目・同期・速度・音量の計算)の試験は装置を使わず、
`cargo test --workspace` で流れる。
FFmpeg が PATH に入っていないシェルでは、次のように FFmpeg のフォルダを渡して実行する(実行場所はリポジトリのルート。`C:\ffmpeg\bin` は例である)。

```
CLIPVIEWER_FFMPEG_DIR='C:\ffmpeg\bin' cargo xtask verify    # Git Bash
$env:CLIPVIEWER_FFMPEG_DIR='C:\ffmpeg\bin'; cargo xtask verify  # PowerShell
```

`cargo xtask verify` の結合試験の工程は、`crates/clip_viewer` の映像の供給の結合試験(`--ignored`)も同じ条件で流す。

アプリ(`crates/clip_viewer`)は次のように起動する(実行場所はリポジトリのルート)。

```
cargo xtask run                                                  # 保存した設定 → PATH の順に FFmpeg を探す
CLIPVIEWER_FFMPEG_DIR='C:\ffmpeg\bin' cargo xtask run          # Git Bash。その起動だけ FFmpeg の場所を指定する
$env:CLIPVIEWER_FFMPEG_DIR='C:\ffmpeg\bin'; cargo xtask run    # PowerShell
```

FFmpeg が見つからないときに画面で置き場所を指定する手順と、FFmpeg を探す順は、`_doc/使い方.md` 2.3節に書いてある。

### インストーラーとリリース

`cargo xtask installer` は、アプリを release でビルドし(C の実行時ライブラリを静的に取り込み、Visual C++ の再頒布パッケージを要らなくする)、
第三者のライセンス表示を作り、Inno Setup 6 の `ISCC.exe` で `installer/ClipViewer.iss` から `<ビルドの出力先>/installer/ClipViewer-<版>-setup.exe` を組み立てる。
release のビルドには `--remap-path-prefix` を渡し、実行ファイルに埋まるソースのパス(panic の位置情報)から、利用者のフォルダ(`~` にする)と
リポジトリの置き場所(`.` と `..` にする)を消す(`xtask/src/installer/release_flags.rs`)。リポジトリの1つ上のフォルダは、それが利用者のフォルダか
`CARGO_HOME` を含むとき(リポジトリを `C:\ClipViewer` に置いたとき等)は置き換えない。置き換えると利用者の名が残るためである。
ビルドの出力先は環境変数 `CARGO_TARGET_DIR` があればそこ、無ければ `target` である。版はワークスペースの版(ルートの `Cargo.toml`)である。
インストーラーに入るのは `clip_viewer.exe` と `LICENSE.txt` と `THIRD-PARTY-NOTICES.html` の3つであり、FFmpeg は含めない。動画のファイルとの関連付け(「プログラムから開く」と「既定のアプリ」への登録)は `installer/file_association.iss` にあり、利用者ごとのインストールでは HKCU、全利用者向けでは HKLM に書き、アンインストールで消す。アプリのアイコン(`assets/icon/`)は、実行ファイルのリソース(`crates/clip_viewer/build.rs` が埋め込む。エクスプローラー・タスクバー・関連付けた動画のファイルの表示)と、窓の題名の帯と Alt+Tab(`main.rs` が `ClipViewer-256.png` を窓へ渡す)と、setup.exe とアプリの追加と削除の一覧に出す。

インストーラーの版の情報は、版の文字列(`AppVersion`)と、そこから先行版の印(`-beta.1` 等)を除いた数字だけの版(`NumericVersion`。
実行ファイルの版の情報 `VersionInfoVersion` は数字とピリオドだけを受けるため)の2つを渡す。

第三者のライセンス表示は `cargo xtask notices` でも単独で作れる(`<ビルドの出力先>/installer/THIRD-PARTY-NOTICES.html`)。
cargo-about(`cargo install cargo-about --locked --features cli`)が、`crates/clip_viewer` の Windows の x64 で使う依存(ビルドの時だけ動くものと試験の依存を除く)の
ライセンスを集め、`installer/notices/about.hbs` の雛形で書く。受け入れるライセンスは `installer/notices/about.toml` に並べてある。
ライセンスを決められないクレートがあれば失敗する(`xtask/src/installer/notices.rs`)。installer は、長い release のビルドの前に cargo-about があるかを確かめる。

release のビルドはコンソールの窓を出さない(`windows_subsystem = "windows"`)。起動の途中の失敗(窓を作れない・日本語フォントを読めない)は、
標準エラーの代わりに OS のメッセージボックスで見せる。どのスレッドの panic も `%LOCALAPPDATA%\ClipViewer\logs\crash-<日時>.log` へ書き、
メッセージボックスで知らせる。依存のライブラリが `log` へ出す警告は同じフォルダの `warnings.log` へ書く。開発のビルド(`cargo xtask run`)はコンソールを出す。

リリースは次の順に行う(実行場所はリポジトリのルート)。

1. ワークスペースの版(ルートの `Cargo.toml`)を上げ、`cargo xtask verify` を通す。版を上げると Cargo.lock のワークスペースのクレートの版も変わるため、
   Cargo.toml と Cargo.lock を一緒にコミットする。
2. `git status` に何も出ないことを確かめる。Cargo.lock はリポジトリのものがそのまま CI で使われ、書き換えが要る形なら CI が止まる。
3. `v<版>` のタグ(例: `v0.1.0`)を push する。`.github/workflows/release.yml` が Windows で `cargo fetch --locked`(Cargo.lock の書き換えが要るなら止める)→
   FFmpeg を choco で入れる → `cargo xtask verify` → `cargo xtask installer` を流し、setup.exe を GitHub Releases に置く。タグと版が一致しなければ止まる。

リリースの前に CI で検証とインストーラーの組み立てだけを確かめたいときは、GitHub の Actions の画面から release のワークフローを手動で実行する
(workflow_dispatch)。setup.exe は実行の成果物(artifact)として残り、タグを選んで実行しても Releases には置かない。
CI には音声出力装置が無いと見込んでおり、検証列は音声出力装置の確認を除いて通る(装置が無いと例 `device_check` は終了コード3で終わり、verify は実行しなかったと表示する)。
SengenEgui は開発機と同じく、CI でも GitHub から Cargo.lock の rev のとおりに取る。

**配る setup.exe は CI で作ったものだけにする。** 開発機で `cargo xtask installer` を実行して作った setup.exe は、動作の確かめにだけ使い、配らない。
配るものを、タグの付いたコミットと GitHub の上の同じ手順で作ったものに限るためである。

### 文書

生存型文書の索引である。

| 文書 | 内容 |
|---|---|
| [CLAUDE.md](CLAUDE.md) | このリポジトリ固有のコーディングルール |
| [_doc/使い方.md](_doc/使い方.md) | 利用者向けの使い方の説明書。オーナーが承認した後は、利用者から見た振る舞いの正本になる(承認までは草案。扱いの規則は CLAUDE.md の「利用者向けの説明書」) |
| [_doc/設計/アーキテクチャ.md](_doc/設計/アーキテクチャ.md) | 層と依存の向き、主要な設計判断、この段階でやらないこと |
| [_doc/設計/画面.md](_doc/設計/画面.md) | `crates/clip_viewer` の内側の層、映像の供給と先読みの規則、移植元との操作の対応表、正準の一仕事 |
| [_doc/設計/ライブラリ.md](_doc/設計/ライブラリ.md) | スタックのライブラリ(保存したスタックの一覧)の層、保存の形式と置き場所、自動保存の流れ、一覧の画面 |

### 利用ライブラリ

| クレート | 使う層 | 採用理由 | 差し替えの費用 |
|---|---|---|---|
| serde | clip_domain, video_source, clip_viewer | 設定ファイル(JSON)の型との対応を derive で書くため。Rust の直列化の事実上の標準である | 小さい。clip_domain で使う場所は `settings/v4.rs` の型だけであり、video_source で使う場所は ffprobe の出力の形(`probe/json_shape.rs`)だけであり、clip_viewer で使う場所はアプリの設定の読み書き(`persistence/app_settings/`。版ごとの形は `v0.rs`・`v1.rs`)と、起動の受け口の受け渡しの形(`launch/guide.rs`・`launch/wire.rs`)だけである |
| serde_json | clip_domain, video_source, clip_viewer | ブラウザ版と同じ JSON 形式を読み書きするため。video_source では ffprobe の JSON 出力(`-of json`)を読むため。2文字字下げの出力がブラウザ版の `JSON.stringify(値, null, 2)` と揃う | 小さい。使う場所は clip_domain の `settings/` と video_source の `probe/json.rs` と clip_viewer の `persistence/app_settings/`・`launch/` だけであり、どれも公開の型(エラーを含む)に serde_json の型を出さない |
| thiserror | clip_domain, video_source, clip_library, thumbnail_cache, clip_viewer | 型付きエラーの `Display` と `Error` の実装を derive で書くため。公開APIの型に現れない | 小さい。手書きの実装へ置き換えられる |
| eframe | clip_viewer | 窓を作って egui を毎フレーム描く土台。SengenEgui の egui と同じマイナー版(0.32)に揃える(`cargo tree -i egui` で1つの版であることを確かめる) | 中くらい。使う場所は起動の部分(`main.rs`・`screen_shell.rs`)と、テクスチャの登録に egui の本体を渡す箇所だけである |
| sengen_egui | clip_viewer | 画面を宣言的に組む。操作を応答として集めて描画の後に適用する形が、状態と画面を分ける設計(`_doc/設計/画面.md` 判断1)と一致する。git 依存を rev で固定する | 大きい。画面のコード(`view/`)の全体が依存する |
| chrono | clip_viewer | ライブラリの一覧に更新日時をこの計算機の時間帯で出すため。標準ライブラリだけでは時間帯の差を得られない。`default-features = false` で `clock` だけを使う | 小さい。使う場所は一覧の日時の表示(`view/library/`)と、壊れた settings.json の移し先の名前(`settings.json.broken-<日時>-<プロセスの番号>`)を作るところ(`persistence/app_settings/broken_move.rs`)の2つである。ライブラリの保存の形式は協定世界時のミリ秒の整数で chrono を知らない |
| image | clip_viewer | ライブラリの一覧のサムネイル(JPEG)を画素へ戻してテクスチャにするため。`default-features = false` で `jpeg` だけを使う。ffmpeg で1枚ずつ RGBA へ変換する方法より数百件で10倍以上速く(1枚ごとのプロセスの起動が無い)、同じ版を eframe が既に依存の木に入れているため、足す重さは jpeg の復号器だけである(`ライブラリ.md` 判断10) | 小さい。使う場所は `thumbnail_feed/pixels.rs` だけである |
| log | clip_viewer | 依存のライブラリ(eframe・winit・glow・rfd)が `log` の口へ出す警告を、release のビルドでも見えるようにファイル(`%LOCALAPPDATA%\ClipViewer\logs\warnings.log`)へ残すため。同じ版を eframe が既に依存の木に入れているため、足す重さは無い。書き出しは自前の小さな実装(`crash_record/warning_log.rs`)であり、env_logger 等の書き出しのクレートは足さない | 小さい。使う場所は `crash_record/warning_log.rs` だけである |
| rfd | clip_viewer | 設定ファイルと動画を選ぶ・保存先を選ぶ OS のファイルダイアログと、起動の途中の失敗と落ちたことを見せる OS のメッセージボックスを出すため | 小さい。使う場所は `persistence/stack_file.rs` と `startup_notice.rs` だけである |
| winresource | clip_viewer(ビルドの時だけ) | アプリのアイコン(`assets/icon/ClipViewer.ico`)と版の情報を Windows のリソースとして実行ファイルへ埋め込むため(`crates/clip_viewer/build.rs`)。Windows SDK の rc.exe を探して呼ぶ手順を持ち、.rc のファイルを手で書かずに済む。Windows 以外へ向けたビルドでは build.rs が呼ばない。ビルドの時だけ動くため、配る実行ファイルには入らず、第三者のライセンス表示の対象にもならない | 小さい。使う場所は build.rs だけであり、embed-resource と .rc のファイルへ置き換えられる |
| cpal | audio_output | 音声出力装置(Windows の WASAPI 等)を開き、装置が音を求めるたびに呼ばれる関数を登録するため。Rust で装置を直接扱う事実上の標準であり、OS ごとの音声の API の癖(経験的な知識)を自作では再獲得できない。unsafe は cpal の内部に閉じ、本リポジトリのコードは unsafe を書かない | 小さい。使う場所は `crates/audio_output/src/device/` だけであり、音の再生器(`player/`)は cpal を知らない |

## ライセンス

ClipViewer は MIT License で配布する(本文は [LICENSE](LICENSE))。著作権者は megaDenryu である。
配る実行ファイルに組み込んだ第三者のライブラリのライセンス表示は、インストール先の `THIRD-PARTY-NOTICES.html` にある(作り方は「インストーラーとリリース」の節)。
