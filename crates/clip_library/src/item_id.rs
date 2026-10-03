//! ライブラリの保存物の識別子。スタックの識別子と重ね合わせの識別子を、保存物のファイルの読み書き係と読み書きのエラーが同じ規則で使えるようにする。

use std::fmt;

use clip_domain::{スタックの識別子, 重ね合わせの識別子};

/// ライブラリの保存物の識別子とは、ライブラリが保存物(スタックか重ね合わせ)1つを指し、ファイル名の本体(拡張子の前)に使う識別子のことである。
/// どちらの識別子も、ファイル名に使える文字だけからなることを作るメソッドで保証している。
pub trait ライブラリの保存物の識別子:
    Clone + PartialEq + fmt::Display + fmt::Debug + Send + 'static
{
    /// エラーの文で保存物を呼ぶ名前(「スタック」「重ね合わせ」)。
    const 保存物の呼び名: &'static str;

    /// ファイル名の本体にする文字列。
    fn ファイル名の本体(&self) -> &str;

    /// ファイル名の本体から読む。識別子として成立しなければ、成立しない理由の文を返す。
    fn ファイル名の本体から読む(本体: String) -> Result<Self, String>;
}

impl ライブラリの保存物の識別子 for スタックの識別子 {
    const 保存物の呼び名: &'static str = "スタック";

    fn ファイル名の本体(&self) -> &str {
        self.文字列()
    }

    fn ファイル名の本体から読む(本体: String) -> Result<Self, String> {
        Self::文字列から作成する(本体).map_err(|不備| 不備.to_string())
    }
}

impl ライブラリの保存物の識別子 for 重ね合わせの識別子 {
    const 保存物の呼び名: &'static str = "重ね合わせ";

    fn ファイル名の本体(&self) -> &str {
        self.文字列()
    }

    fn ファイル名の本体から読む(本体: String) -> Result<Self, String> {
        Self::文字列から作成する(本体).map_err(|不備| 不備.to_string())
    }
}
