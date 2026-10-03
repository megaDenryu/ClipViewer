//! 錠を試したライブラリの組を落とすときの、錠を放す順の試験。`overlays` の錠を、スタックのライブラリの錠より先に放すことを確かめる。
//! 利用者の %APPDATA% を使わず、一時フォルダを置き場所にする。

use crate::folder::ライブラリのフォルダ;
use crate::library::スタックのライブラリ;
use crate::lock_release_record::記録を取り出す;

#[test]
fn 組を落とすとoverlaysの錠をスタックのライブラリの錠より先に放す() {
    let 一時 = std::env::temp_dir().join(format!("clip_library_錠を放す順_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&一時);
    let フォルダ = ライブラリのフォルダ::作成する(一時.join("library"));
    let 組 = スタックのライブラリ::作成する(フォルダ.clone())
        .錠を試す()
        .重ね合わせの錠も試す();
    assert!(組.スタック.書けるか() && 組.重ね合わせ.書けるか());
    let _ = 記録を取り出す();
    drop(組);
    assert_eq!(
        記録を取り出す(),
        vec![
            フォルダ.重ね合わせのフォルダ().パス().to_path_buf(),
            フォルダ.パス().to_path_buf(),
        ]
    );
    let _ = std::fs::remove_dir_all(&一時);
}
