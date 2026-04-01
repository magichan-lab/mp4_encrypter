//! reducer 用意図と副作用命令定義

use std::path::PathBuf;

use crate::domain::entities::{DecryptionResult, LaunchRequest};
use crate::domain::errors::AppError;
use crate::domain::value_objects::DecryptionKey;
use crate::presentation::state::KeyInputMode;

/// ファイル検査文脈
#[derive(Debug, Clone, Copy)]
pub enum InspectContext {
    /// キー未保持検査
    WithoutKey,
    /// キー保持中検査
    WithKey,
}

/// ファイル検査結果
#[derive(Debug, Clone)]
pub enum InspectionOutcome {
    /// 暗号化状態
    Encrypted,
    /// 非暗号化状態
    Plain,
    /// 検査失敗結果
    Failed(AppError),
}

/// reducer 解釈対象意図
///
/// @property path 対象ファイルパス
/// @property context ファイル検査文脈
/// @property outcome ファイル検査結果
/// @property job_id 復号ジョブ識別子
/// @property filename 進捗表示用ファイル名
/// @property ratio 0.0..=1.0 の進捗率
/// @property result 復号完了結果
#[derive(Debug, Clone)]
pub enum Intent {
    /// 起動引数解析結果
    LaunchParsed(Result<LaunchRequest, AppError>),
    /// Tick 通知
    Tick,
    /// ファイルドロップ
    FileDropped(PathBuf),
    /// ファイル検査完了
    FileInspected { path: PathBuf, context: InspectContext, outcome: InspectionOutcome },
    /// 復号進捗更新
    WorkerProgress { job_id: u64, filename: String, ratio: f32 },
    /// 復号完了通知
    WorkerFinished { job_id: u64, result: DecryptionResult },
    /// ダイアログ OK 押下
    DialogAcknowledged,
    /// ダイアログ YES 押下
    DialogConfirmed,
    /// ダイアログ NO 押下
    DialogDismissed,
    /// キー入力変更
    KeyInputChanged(String),
    /// キー入力方式切り替え
    KeyInputModeChanged(KeyInputMode),
}

/// reducer 返却副作用命令
///
/// @property path 対象ファイルパス
/// @property context ファイル検査文脈
/// @property job_id 復号ジョブ識別子
/// @property key 復号キー
#[derive(Debug, Clone)]
pub enum Effect {
    /// ファイル暗号化判定要求
    InspectFile { path: PathBuf, context: InspectContext },
    /// 復号ジョブ開始要求
    StartDecryption { job_id: u64, path: PathBuf, key: DecryptionKey },
    /// ワーカー一時停止要求
    PauseWorker,
    /// ワーカー再開要求
    ResumeWorker,
    /// ワーカーキャンセル要求
    CancelWorker,
}
