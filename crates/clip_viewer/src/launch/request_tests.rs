//! 起動の引数の読み方の試験。

use std::ffi::OsString;

use super::test_support::動画を開く頼み;
use super::起動の頼み;

#[test]
fn 引数が無ければ前に出る頼みで_複数なら最初の1つと残りの数にする() {
    let 引数 = |並び: &[&str]| 並び.iter().map(OsString::from).collect::<Vec<_>>();
    assert_eq!(起動の頼み::引数から読む(引数(&[])), 起動の頼み::前に出る);
    assert_eq!(
        起動の頼み::引数から読む(引数(&[r"C:\動画\a.mp4"])),
        動画を開く頼み(r"C:\動画\a.mp4", 0)
    );
    assert_eq!(
        起動の頼み::引数から読む(引数(&[r"C:\a.mp4", r"C:\b.mp4", r"C:\c.mp4"])),
        動画を開く頼み(r"C:\a.mp4", 2)
    );
}
