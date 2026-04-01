//! UI イベントメッセージ定義

use std::path::PathBuf;

use crate::presentation::intent::{InspectContext, InspectionOutcome};

/// UI メッセージ
#[derive(Debug, Clone)]
pub enum Message {
    /// タイマー tick
    Tick,
    /// ファイルドロップ
    FileDropped(PathBuf),
    /// ファイル検査完了
    FileInspectionCompleted { path: PathBuf, context: InspectContext, outcome: InspectionOutcome },
    /// 情報／エラー確認ダイアログ OK
    DialogAcknowledged,
    /// 確認ダイアログ YES
    DialogConfirmed,
    /// 確認ダイアログ NO
    DialogDismissed,
    /// キー入力変更
    KeyInputChanged(String),
}
