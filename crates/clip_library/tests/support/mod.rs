//! 試験の共有部品。試験ごとに重ならない一時フォルダと、試験用のスタックを作る。
#![allow(dead_code, clippy::expect_used)]

use std::path::PathBuf;

use clip_domain::*;
use clip_library::{スタックのライブラリ, ライブラリのフォルダ};

/// 試験ごとに重ならない一時フォルダの下のライブラリ。試験の名前とプロセス番号で分け、前の実行の残りを消してから使う。
pub struct 一時のライブラリ {
    pub 一時フォルダ: PathBuf,
    pub ライブラリ: スタックのライブラリ,
}

impl 一時のライブラリ {
    pub fn 作る(名前: &str) -> Self {
        let 一時フォルダ =
            std::env::temp_dir().join(format!("clip_library_試験_{名前}_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&一時フォルダ);
        std::fs::create_dir_all(&一時フォルダ).expect("一時フォルダを作れる");
        let ライブラリ = スタックのライブラリ::作成する(
            ライブラリのフォルダ::作成する(一時フォルダ.join("library")),
        );
        Self {
            一時フォルダ,
            ライブラリ,
        }
    }

    /// 一時フォルダの中に空の動画のファイルを作り、その正規化した動画パスを返す。
    pub fn 動画を置く(&self, 名前: &str) -> 正規化した動画パス {
        let パス = self.一時フォルダ.join(名前);
        std::fs::write(&パス, b"").expect("動画のファイルを書ける");
        入力された動画パス::作成する(パス.display().to_string()).正規化する()
    }

    /// ライブラリのフォルダの中のファイル名を並べる。
    pub fn ファイル名の一覧(&self) -> Vec<String> {
        let mut 名前 = std::fs::read_dir(self.ライブラリ.フォルダ().パス())
            .expect("フォルダを読める")
            .map(|フォルダの中の項目| {
                フォルダの中の項目
                    .expect("フォルダの中の項目")
                    .file_name()
                    .to_string_lossy()
                    .into_owned()
            })
            .collect::<Vec<_>>();
        名前.sort();
        名前
    }
}

impl Drop for 一時のライブラリ {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.一時フォルダ);
    }
}

pub fn 識別子(文字列: &str) -> スタックの識別子 {
    スタックの識別子::文字列から作成する(文字列.to_string()).expect("識別子")
}

pub fn 日時(ミリ秒: u64) -> ライブラリの日時 {
    ライブラリの日時::紀元からのミリ秒で作成する(ミリ秒).expect("日時")
}

/// 1つのクリップ(1〜2秒)を持つ、指定の識別子と名前と動画のスタック。
pub fn スタックを作る(
    識別子: &str,
    名前: &str,
    動画: 正規化した動画パス,
) -> ライブラリのスタック {
    let クリップの識別子 =
        クリップ識別子::文字列から作成する(format!("{識別子}-c")).expect("識別子");
    let mut クリップ =
        クリップ::既定値で作成する(クリップの識別子, クリップ名::作成する("一".into()));
    let 秒 = |値| 時刻::作成する(値).expect("時刻");
    クリップ.区間 = 動画上の区間::作成する(秒(1.0), 秒(2.0)).expect("区間");
    let 並び = クリップスタック::一覧から作成する(vec![クリップ]).expect("並び");
    ライブラリのスタック::新しく作る(
        self::識別子(識別子),
        スタックの名前::作成する(名前.to_string()).expect("名前"),
        動画,
        並び,
        日時(1_000),
    )
}
