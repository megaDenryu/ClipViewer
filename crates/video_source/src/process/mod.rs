//! ffmpeg を子プロセスとして起動し、RGBA の生データを1コマずつ読む境界。期限と取り消しの合図を見回りながら終わりを待つ子プロセスも持つ。

mod cancel;
mod child;
mod deadline;
mod deadline_outcome;
#[cfg(test)]
mod deadline_tests;
mod error;
mod frame_reader;
mod launch;
mod spec;
mod stderr_tail;
mod thread;

pub use cancel::取り消しの合図;
pub(crate) use child::起動した子プロセス;
pub(crate) use deadline::期限付きの子プロセス;
pub(crate) use deadline_outcome::期限付きの待ちの失敗;
pub use error::デコードの失敗;
pub(crate) use frame_reader::コマの読み手;
pub(crate) use launch::起動したデコード;
pub(crate) use spec::デコードの指定;
pub(crate) use stderr_tail::標準エラーの収集;
pub use stderr_tail::標準エラーの末尾;
pub(crate) use thread::終わりを待つスレッド;
