//! キーの設定で Escape を割り当てない規則の試験。キーを待っている間に Escape のキーを含む組(Escape・Shift+Escape・Ctrl+Escape)を押すと、
//! ダイアログを閉じず、キーを変えずに待ちをやめることを確かめる。参照: _doc/設計/画面.md 判断18「Escape の扱い」

use eframe::egui;

use super::keys_settings::{待っている状態, 押して当てる};
use crate::state::キーの一覧のダイアログ;
use crate::viewer_settings::{キーで行う操作, キーの割り当て};

#[test]
fn 待っている間にエスケープを含む組を押すとダイアログを閉じず_キーを変えずに待ちをやめる() {
    let ctrl = egui::Modifiers {
        ctrl: true,
        command: true,
        ..egui::Modifiers::NONE
    };
    for 修飾キー in [egui::Modifiers::NONE, egui::Modifiers::SHIFT, ctrl] {
        let mut 状態 = 待っている状態(キーで行う操作::シアターと編集を切り替える);
        押して当てる(&mut 状態, egui::Key::Escape, 修飾キー);
        assert_eq!(
            状態.キーの一覧,
            キーの一覧のダイアログ::開いている,
            "{修飾キー:?}"
        );
        assert_eq!(状態.キーの割り当て, キーの割り当て::既定(), "{修飾キー:?}");
    }
}
