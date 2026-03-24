//! プレゼンテーション用 DTO 定義

use std::path::PathBuf;

/// ダイアログ表示状態 DTO
///
/// @property title ダイアログ見出し
/// @property message ダイアログ本文
/// @property next_has_key ダイアログ終了後のキー保持状態
#[derive(Debug, Clone)]
pub enum DialogState {
    /// 情報ダイアログ
    Info { title: String, message: String, next_has_key: bool },
    /// エラーダイアログ
    Error { title: String, message: String, next_has_key: bool },
    /// 実行中ジョブ切り替え確認ダイアログ
    ConfirmSwitch { path: PathBuf },
}

impl DialogState {
    /// 次キー保持状態取得処理
    ///
    /// @return 次キー保持状態
    pub fn next_has_key(&self) -> Option<bool> {
        match self {
            Self::Info { next_has_key, .. } | Self::Error { next_has_key, .. } => {
                Some(*next_has_key)
            }
            Self::ConfirmSwitch { .. } => None,
        }
    }
}
