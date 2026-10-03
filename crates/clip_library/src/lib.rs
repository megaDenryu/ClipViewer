//! ライブラリの置き場所(フォルダ)の読み書き。スタック1つをライブラリのフォルダの1ファイルに、重ね合わせ1つをサブフォルダ `overlays` の1ファイルに書き、
//! 一覧はフォルダを読んで作る。
//!
//! 書き込みは一時ファイルへ書いてから置き換え、途中で落ちても元のファイルを壊さない。読めないファイルは黙って飛ばさず、
//! 一覧に理由付きで出す。画面のスレッドを止めないよう、書き込みと走査を裏のスレッドで行う係(スタックと重ね合わせに1つずつ)と、
//! 書けるアプリを1つに限る錠(ライブラリのフォルダと `overlays` に1つずつ)も持つ。`overlays` の錠はライブラリの錠を持つアプリだけが取る。
//! サムネイルは知らない(ライブラリの外のキャッシュに置く)。以前の版がこのフォルダに置いたサムネイルと、1日より前から置き去りの書きかけのファイルは、
//! 書けるアプリが一覧を作る前に片付ける。
//! 参照: _doc/設計/ライブラリ.md、_doc/設計/同時再生.md 4-2

#![warn(missing_docs)]

mod background;
mod background_threads;
mod background_worker;
mod changes;
mod error;
mod file_name_id;
mod folder;
mod item_folder;
mod legacy_thumbnail;
mod library;
mod listing;
mod lock;
mod lock_file;
mod overlay_background;
mod overlay_error;
mod overlay_folder;
mod overlay_library;
mod overlay_listing;
mod overlay_lock;
mod overlay_request;
mod overlay_tried_lock;
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
pub use overlay_background::裏で動く重ね合わせのライブラリ;
pub use overlay_error::重ね合わせのライブラリの操作エラー;
pub use overlay_folder::重ね合わせのフォルダ;
pub use overlay_library::重ね合わせのライブラリ;
pub use overlay_listing::{
    読めた重ね合わせ, 重ね合わせの一覧, 重ね合わせの一覧の項目
};
pub use overlay_lock::{重ね合わせの錠, 重ね合わせの錠を取れない理由};
pub use overlay_request::{
    重ね合わせのライブラリからの知らせ, 重ね合わせのライブラリへの頼み
};
pub use overlay_tried_lock::{
    重ね合わせの書き込みの許し, 錠を試した重ね合わせのライブラリ
};
pub use permission::書き込みの許し;
pub use request::{
    ライブラリからの知らせ, ライブラリへの頼み, 保存の番号, 変更の種類
};
pub use tried_lock::錠を試したライブラリ;
