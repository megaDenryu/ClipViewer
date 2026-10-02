//! 差し替えた cargo へ渡す引数。cargo の引数のサブコマンドの直後へ `--config <差し替えの設定>` を挟む。
//! サブコマンドより前に置くと、cargo の外の実行ファイルとして動くサブコマンド(clippy)が中で起こす cargo へ設定が届かず、
//! 手元の SengenEgui へ差し替わらないためである。サブコマンドの直後に置けば、組み込みのサブコマンドもその設定を受け取る。

/// cargo の引数のサブコマンドの直後に `--config` と差し替えの設定を挟んだ引数。`+` で始まる道具の版の指定(`+nightly`)は
/// サブコマンドではないため、その次をサブコマンドとみなす。
pub fn 差し替えの設定を挟んだ引数<'a>(
    cargoの引数: &'a [String],
    差し替えの設定: &'a str,
) -> Vec<&'a str> {
    let 版の指定の数 = usize::from(
        cargoの引数
            .first()
            .is_some_and(|最初| 最初.starts_with('+')),
    );
    let サブコマンドまでの数 = (版の指定の数 + 1).min(cargoの引数.len());
    let (前, 後) = cargoの引数.split_at(サブコマンドまでの数);
    前.iter()
        .map(String::as_str)
        .chain(["--config", 差し替えの設定])
        .chain(後.iter().map(String::as_str))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::差し替えの設定を挟んだ引数;

    fn 文字列の並び(並び: &[&str]) -> Vec<String> {
        並び.iter().map(ToString::to_string).collect()
    }

    #[test]
    fn 設定はサブコマンドの直後に挟む() {
        let 引数 = 文字列の並び(&["clippy", "--workspace", "--", "-D", "warnings"]);
        assert_eq!(
            差し替えの設定を挟んだ引数(&引数, "設定"),
            [
                "clippy",
                "--config",
                "設定",
                "--workspace",
                "--",
                "-D",
                "warnings"
            ]
        );
    }

    #[test]
    fn 道具の版の指定があればその次のサブコマンドの直後に挟む() {
        let 引数 = 文字列の並び(&["+nightly", "test", "--workspace"]);
        assert_eq!(
            差し替えの設定を挟んだ引数(&引数, "設定"),
            ["+nightly", "test", "--config", "設定", "--workspace"]
        );
    }
}
