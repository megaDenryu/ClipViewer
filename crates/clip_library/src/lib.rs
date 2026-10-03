//! スタックのライブラリの置き場所(フォルダ)の読み書き。スタック1つを1ファイルに書き、一覧はフォルダを読んで作る。
//!
//! 書き込みは一時ファイルへ書いてから置き換え、途中で落ちても元のファイルを壊さない。読めないファイルは黙って飛ばさず、
//! 一覧に理由付きで出す。画面のスレッドを止めないよう、書き込みと走査を裏のスレッドで行う係と、書けるアプリを1つに限る錠も持つ。
//! サムネイルは知らない(ライブラリの外のキャッシュに置く)。以前の版がこのフォルダに置いたサムネイルと、1日より前から置き去りの書きかけのファイルは、
//! 書けるアプリが一覧を作る前に片付ける。
//! 参照: _doc/設計/ライブラリ.md

#![warn(missing_docs)]

mod background;
mod background_threads;
mod background_worker;
mod changes;
mod error;
mod folder;
mod item_folder;
mod legacy_thumbnail;
mod library;
mod listing;
mod lock;
mod lock_file;
mod permission;
mod pre_scan_cleanup;
mod request;
mod scan;
mod scan_thread;
mod tried_lock;

pub use background::{裏で動くライブラリ, 頼めない理由};
pub use error::ライブラリの操作エラー;
pub use folder::ライブラリのフォルダ;
pub use library::スタックのライブラリ;
pub use listing::{
    ライブラリのファイル名, ライブラリの一覧, 一覧の項目, 動画の有無, 読めたスタック,
    読めないファイル,
};
pub use lock::{ライブラリの錠, 錠を取れない理由};
pub use permission::書き込みの許し;
pub use request::{
    ライブラリからの知らせ, ライブラリへの頼み, 保存の番号, 変更の種類
};
pub use tried_lock::錠を試したライブラリ;
