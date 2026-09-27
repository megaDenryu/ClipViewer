//! 落ちたときの記録と警告の記録。release のビルドはコンソールのウインドウを持たず、panic の文面も依存のライブラリの警告も利用者に見えないため、
//! ローカルのアプリのデータのフォルダ(Windows の %LOCALAPPDATA%)の下の `ClipViewer\logs` へファイルとして残す。
//! 起動の部分(main.rs)が、落ちたときの記録は起動の最初に、警告の記録は起動の決着の後にライブラリの錠を持つアプリだけが、それぞれ1回だけ取り付ける。
//! 警告の記録をそう限るのは、2つ目のアプリ(1つ目へ頼みを渡して終わるもの、読み取り専用で起動したもの)が、1つ目の warnings.log を前回の記録へ移さないためである。

mod panic_hook;
mod timestamp;
mod warning_log;

use std::path::PathBuf;

use panic_hook::落ちたときの記録係;
use timestamp::記録の日時;
use warning_log::警告の記録係;

/// 記録を置くフォルダとは、落ちたときの記録と警告の記録を書くフォルダ(`%LOCALAPPDATA%\ClipViewer\logs`)のことである。
#[derive(Debug, Clone)]
struct 記録を置くフォルダ(PathBuf);

impl 記録を置くフォルダ {
    /// ローカルのアプリのデータのフォルダ(環境変数が無ければ無い)から決める。無ければ記録を置く場所も無い。
    fn ローカルのアプリのデータのフォルダから決める(
        ローカルのアプリのデータのフォルダ: Option<PathBuf>,
    ) -> Option<Self> {
        ローカルのアプリのデータのフォルダ
            .map(|フォルダ| Self(フォルダ.join("ClipViewer").join("logs")))
    }

    /// 落ちた日時を名に含む、落ちたときの記録のファイル(`crash-<日時>.log`)のパス。
    fn 落ちたときの記録のファイル(&self, 日時: &記録の日時) -> PathBuf {
        self.0
            .join(format!("crash-{}.log", 日時.ファイルの名に入れる形()))
    }

    /// 今回の起動の警告の記録のファイルと、前回の起動の警告の記録のファイルのパス。
    fn 警告の記録のファイル(&self) -> (PathBuf, PathBuf) {
        (
            self.0.join("warnings.log"),
            self.0.join("warnings.previous.log"),
        )
    }

    /// フォルダが無ければ作る。
    fn 用意する(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(&self.0)
    }
}

/// 記録の置き場所とは、記録を置くフォルダ(環境変数 LOCALAPPDATA が無ければ無い)のことである。
/// 起動の部分は、これを1回作り、落ちたときの記録と警告の記録をそれぞれの時点で取り付ける。
pub(crate) struct 記録の置き場所(Option<記録を置くフォルダ>);

impl 記録の置き場所 {
    pub(crate) fn ローカルのアプリのデータのフォルダから決める(
        ローカルのアプリのデータのフォルダ: Option<PathBuf>,
    ) -> Self {
        Self(記録を置くフォルダ::ローカルのアプリのデータのフォルダから決める(
            ローカルのアプリのデータのフォルダ,
        ))
    }

    /// panic の処理として、落ちたときの記録を取り付ける。どのスレッドの panic も記録するため、起動の最初に呼ぶ。
    pub(crate) fn 落ちたときの記録を取り付ける(&self) {
        落ちたときの記録係::作成する(self.0.clone()).取り付ける();
    }

    /// 警告の記録を取り付ける。取り付けられなくても起動は続ける(記録が残らないだけで操作はできる)。
    pub(crate) fn 警告の記録を取り付ける(&self) {
        if let Some(フォルダ) = &self.0
            && let Err(理由) = 警告の記録係::取り付ける(フォルダ)
        {
            eprintln!("警告の記録を書き始められない: {理由}");
        }
    }
}
