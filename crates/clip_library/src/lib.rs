//! ライブラリの置き場所(フォルダ)の読み書き。スタック1つをライブラリのフォルダの1ファイルに、重ね合わせ1つをサブフォルダ `overlays` の1ファイルに書き、
//! 一覧はフォルダを読んで作る。
//!
//! 書き込みは一時ファイルへ書いてから置き換え、途中で落ちても元のファイルを壊さない。読めないファイルは黙って飛ばさず、
//! 一覧に理由付きで出す。画面のスレッドを止めないよう、書き込みと走査を裏のスレッドで行う係(スタックと重ね合わせに1つずつ)と、
//! 書けるアプリを1つに限る錠(ライブラリのフォルダと `overlays` に1つずつ)も持つ。`overlays` の錠はスタックのライブラリの錠を持つアプリだけが取る。
//! スタックと重ね合わせは、名前の規則・読み書きの手順・錠の取り方・裏のスレッド・書き込みの許しを同じ型で共に使い、型ごとに違うのは
//! 識別子・項目・頼み・知らせ・錠の型だけである。
//! サムネイルは知らない(ライブラリの外のキャッシュに置く)。以前の版がこのフォルダに置いたサムネイルと、1日より前から置き去りの書きかけのファイルは、
//! 書けるアプリが一覧を作る前に片付ける。
//! 参照: _doc/設計/ライブラリ.md、_doc/設計/同時再生.md 4-2

#![warn(missing_docs)]

mod background;
mod background_library;
mod background_threads;
mod changes;
mod folder;
mod item_error;
mod item_folder;
mod item_id;
mod item_kind;
mod item_read;
mod legacy_thumbnail;
mod library;
mod listing;
mod lock;
mod lock_file;
mod overlay_background;
mod overlay_folder;
mod overlay_library;
mod overlay_listing;
mod overlay_lock;
mod overlay_request;
mod permission;
mod pre_scan_cleanup;
mod request;
mod request_refusal;
mod scan;
mod scan_thread;
mod tried_lock;
mod tried_lock_pair;
mod write_thread;

pub use background::{裏で動くライブラリ, 裏で動く保存物のライブラリ};
pub use background_library::{裏で行うライブラリ, 頼みの扱い};
pub use folder::ライブラリのフォルダ;
pub use item_error::保存物の操作エラー;
pub use item_id::ライブラリの保存物の識別子;
pub use library::{スタックのライブラリ, ライブラリの操作エラー};
pub use listing::{
    ライブラリのファイル名, ライブラリの一覧, 一覧の項目, 動画の有無, 読めたスタック,
    読めないファイル,
};
pub use lock::{ライブラリの錠, 錠を取れない理由};
pub use lock_file::錠のファイルを取れない理由;
pub use overlay_background::裏で動く重ね合わせのライブラリ;
pub use overlay_folder::重ね合わせのフォルダ;
pub use overlay_library::{
    重ね合わせのライブラリ, 重ね合わせのライブラリの操作エラー
};
pub use overlay_listing::{
    読めた重ね合わせ, 重ね合わせの一覧, 重ね合わせの一覧の項目
};
pub use overlay_lock::{
    重ね合わせの書き込みの許し, 重ね合わせの錠, 重ね合わせの錠を取れない理由
};
pub use overlay_request::{
    重ね合わせのライブラリからの知らせ, 重ね合わせのライブラリへの頼み
};
pub use permission::{書き込みの許し, 錠による書き込みの許し};
pub use request::{
    ライブラリからの知らせ, ライブラリへの頼み, 保存の番号, 変更の種類
};
pub use request_refusal::頼めない理由;
pub use tried_lock::{
    錠を試したライブラリ, 錠を試した保存物のライブラリ, 錠を試した重ね合わせのライブラリ,
};
pub use tried_lock_pair::錠を試したライブラリの組;
