# ClipViewer

ClipViewer は、1本の動画から切り出した短い区間(クリップ)を順に並べ、区間ごとに繰り返しの回数・クロップ(切り出す矩形)・
次へ進む条件(自動・Enterキー・Spaceキー・クリック)を付けて再生する Windows のデスクトップアプリである。
Space は再生と停止に使うため、Spaceキー待ちのクリップは Enter で進む。キーの一覧は F1 で出る(`_doc/設計/画面.md` 判断14)。

## 使う人へ

### 動作環境

- Windows 10 または Windows 11 の x64(64ビット)版。インストーラーは Windows 10 より前の Windows では止まる。
- 日本語フォント(游ゴシック・メイリオ・MS ゴシックのどれか)が入っている Windows。日本語版の Windows には最初から入っている。
  英語版などで日本語フォントが入っていないと、画面の文字が出ない(起動のときにその旨のメッセージが出る)。そのときは Windows の設定の
  「システム」→「オプション機能」から「日本語の補助フォント」(Japanese Supplemental Fonts)を追加する。
- FFmpeg(下の「FFmpeg を入れる」)。

### インストールする

1. [Releases](https://github.com/megaDenryu/ClipViewer/releases) から最新の `ClipViewer-<版>-setup.exe` を取得して実行する。
2. 既定では、管理者の権限なしに利用者のフォルダ(`%LOCALAPPDATA%\Programs\ClipViewer`)へ入る。全利用者向けに `Program Files` へ入れたいときは、
   インストーラーの最初の問いで選ぶ。
3. スタートメニューの「ClipViewer」から起動する。デスクトップのショートカットは、インストールのときに選んだ場合だけ作る。

インストールすると、動画のファイル(mp4・m4v・mkv・webm・mov・avi・wmv・flv・mpg・mpeg・m2ts・mts)を右クリックしたときの
「プログラムから開く」に ClipViewer が出る。Windows の設定の「既定のアプリ」で、ClipViewer を動画の既定のアプリに選ぶこともできる。
インストーラーもアプリも、既定のアプリを勝手には変えない。

インストーラーには電子署名を付けていないため、初めて実行するときに Windows の SmartScreen(発行元の分からないアプリを止める警告)が
出ることがある。そのときは「詳細情報」から「実行」を選ぶ。

### FFmpeg を入れる

ClipViewer は動画の読み込みに FFmpeg(動画と音声を変換する無料の道具)の `ffmpeg.exe` と `ffprobe.exe` を使う。FFmpeg はインストーラーに含まれていないため、
別に入れる。入れ方は次の2つのどちらかである。

- winget で入れる(コマンドプロンプトか PowerShell で実行する): `winget install Gyan.FFmpeg`。
  入れた後に ClipViewer を起動し直すと、PATH から自動で見つかる。見つからなければ一度サインアウトしてから起動し直す。
- 公式の案内(https://ffmpeg.org/download.html)から Windows 用のビルドを取得し、好きなフォルダへ展開する。
  迷ったら、案内の「Windows builds from gyan.dev」から release builds の `ffmpeg-release-essentials.zip` を選ぶ。
  ClipViewer が使う機能(動画と音声の読み出し)は essentials の版で足りる。展開したフォルダの中の `bin` に `ffmpeg.exe` と `ffprobe.exe` がある。

FFmpeg が見つからないときは、アプリの画面の上部に理由と手順が出る。`ffmpeg.exe` と `ffprobe.exe` が入ったフォルダ(展開したフォルダの中の `bin`)を
入力して「保存して探し直す」を押すと、見つかった場合だけその場所が保存され、次の起動からも使われる。
FFmpeg を探す順は、環境変数 `CLIPVIEWER_FFMPEG_DIR` → 画面で保存した場所 → PATH である。

### データの置き場所

| 置き場所 | 中身 |
|---|---|
| `%APPDATA%\ClipViewer\library\` | 名前を付けて保存したスタック(クリップの並び)のライブラリ。動画のパスとクリップの並びだけを保存し、動画は写さない |
| `%APPDATA%\ClipViewer\settings.json` | アプリの設定(FFmpeg の場所と、窓の大きさ・音量・速度・左右反転・アスペクト比・表示サイズ)。形式の名前(`format`)と版(`version`)を持つ(形式は `_doc/設計/ライブラリ.md` の「settings.json の形式」) |
| `%LOCALAPPDATA%\ClipViewer\thumbnails\` | 一覧に出すサムネイルのキャッシュ。消しても必要なときに撮り直す |
| `%LOCALAPPDATA%\ClipViewer\logs\` | 落ちたときの記録(`crash-<日時>.log`)と、依存のライブラリが出した警告の記録(`warnings.log`。起動のたびに前回の分を `warnings.previous.log` へ移す)。不具合を知らせるときに添える |

### アンインストールする

Windows の設定の「アプリ」→「インストールされているアプリ」(Windows 10 では「アプリと機能」)で ClipViewer を選び、「アンインストール」を押す。
アンインストールすると、最後に上のデータも消すかを問う。既定の答えは「いいえ」であり、入れ直したときにそのまま使える。
消すのは、アンインストールを実行したアカウントのデータだけである。全利用者向けに入れた場合も、他の利用者のアカウントのデータは消さない。
動画のファイルとの関連付けの登録は、答えによらず消す。

### ソースからビルドする

Rust(1.89 以上)と Windows SDK を入れ、リポジトリを取得してルートで次を実行する。Windows SDK は、実行ファイルへアイコンを埋め込む rc.exe のために要る(Visual Studio の「C++ によるデスクトップ開発」のビルドツールに含まれる。無いとビルドが失敗する)。FFmpeg は上と同じく別に入れる。

```
git clone https://github.com/megaDenryu/ClipViewer
cd ClipViewer
cargo xtask run         # 開発のビルドで起動する
cargo xtask installer   # インストーラーを組み立てる(Inno Setup 6 と cargo-about が要る: winget install JRSoftware.InnoSetup と cargo install cargo-about --locked --features cli)
```

## 開発する人へ

クリップの並びは JSON の設定ファイルに保存し、ブラウザ版 ClipViewer(v4.0 形式)と双方向で読み書きできる。
作ったスタックは名前を付けてアプリの中のライブラリ(`%APPDATA%\ClipViewer\library\`)へ保存でき、一覧から開き直して見返し・編集できる。
ライブラリに保存するのは動画のパスとクリップの並びだけであり、動画は写さない。一覧に出す顔のサムネイルは、捨ててよいキャッシュ(`%LOCALAPPDATA%\ClipViewer\thumbnails\`)に置く(設計は `_doc/設計/ライブラリ.md`)。

ブラウザ版(TypeScript + SengenUI)から Rust + egui(SengenEgui)への移植の途中である。移植の動機は2つある。
(1) ブラウザではローカルの動画ファイルの扱いが不便である。
(2) 短い区間の繰り返しがブラウザの `<video>` のシーク性能に依存し、継ぎ目で詰まる。

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
cargo xtask lock-without-patch  # SengenEgui の差し替えを外して Cargo.lock を作り直す(リリースの前に使う)
```

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

FFmpeg が見つからないときは、画面の上部にその理由と手順が出る。ffmpeg.exe と ffprobe.exe のあるフォルダを入力して
「保存して探し直す」を押すと、見つかった場合だけ `%APPDATA%\ClipViewer\settings.json` に保存され、次の起動から使われる。

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

1. ワークスペースの版(ルートの `Cargo.toml`)を上げてコミットする。
2. `cargo xtask verify` を通す。
3. 作業ツリーが綺麗な状態(`git status` に何も出ない状態)で `cargo xtask lock-without-patch` を流し、Cargo.lock を作り直す。
   コミットしていない変更があると、このコマンドは理由を示して止まる(複製して確かめたものと、コミットするものを一致させるため)。開発機では SengenEgui を `C:\devs\.cargo\config.toml` の
   差し替え(patch)でローカルのフォルダへ向けており、その状態で cargo を動かすと Cargo.lock の `sengen_egui` が source の行の無い形になる。
   CI は git の rev から依存を解くため、そのままでは検証した依存と配る依存が一致しない。このコマンドはリポジトリを `%TEMP%` へ複製し、
   差し替えの効かない場所で `cargo fetch` を動かして `sengen_egui` を git から解き直し、`cargo check --workspace --all-targets --locked` で
   ピン留めした rev で組めることを確かめ、source の行を除けば元と行の並びがまったく同じであることを確かめてから取り込む。
4. すぐに `git diff Cargo.lock` で `sengen_egui` の `source = "git+https://github.com/megaDenryu/SengenEgui?rev=...` の行が増えていることを確かめ、
   `git add Cargo.lock` で Cargo.lock だけを足してコミットする。取り込んでからコミットするまでの間に、開発機で cargo を動かしてはならない。
   Cargo.lock はまた差し替えの形へ戻る。VS Code の rust-analyzer も裏で cargo を動かすため、開いていると同じように戻しうる。
   戻っていたら 3 からやり直す。
5. `v<版>` のタグ(例: `v0.1.0`)を push する。`.github/workflows/release.yml` が Windows で `cargo fetch --locked`(Cargo.lock の書き換えが要るなら止める)→
   FFmpeg を choco で入れる → `cargo xtask verify` → `cargo xtask installer` を流し、setup.exe を GitHub Releases に置く。タグと版が一致しなければ止まる。

リリースの前に CI で検証とインストーラーの組み立てだけを確かめたいときは、GitHub の Actions の画面から release のワークフローを手動で実行する
(workflow_dispatch)。setup.exe は実行の成果物(artifact)として残り、タグを選んで実行しても Releases には置かない。
CI には音声出力装置が無いと見込んでおり、検証列は音声出力装置の確認を除いて通る(装置が無いと例 `device_check` は終了コード3で終わり、verify は実行しなかったと表示する)。
SengenEgui は CI では GitHub から取る(開発機の差し替えは、リポジトリの外に置いてあるため CI では効かない)。

**配る setup.exe は CI で作ったものだけにする。** 開発機で `cargo xtask installer` を実行して作った setup.exe は、差し替えたローカルの SengenEgui で
ビルドしており、Cargo.lock と一致する保証が無いため、配らない(動作の確かめにだけ使う)。

### 文書

生存型文書の索引である。

| 文書 | 内容 |
|---|---|
| [CLAUDE.md](CLAUDE.md) | このリポジトリ固有のコーディングルール |
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
