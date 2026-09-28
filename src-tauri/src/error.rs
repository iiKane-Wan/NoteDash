use serde::ser::{Serialize, Serializer};

/// 统一错误类型：所有命令层错误最终都收敛到这里
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("数据库操作失败：{0}")]
    Db(#[from] rusqlite::Error),
    #[error("文件系统操作失败：{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Message(String),
}

/// Tauri 要求命令错误可序列化后回传前端，这里转为字符串
impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
