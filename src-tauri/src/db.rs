use std::fs;
use std::path::PathBuf;

use rusqlite::Connection;
use tauri::{AppHandle, Manager};

use crate::error::{AppError, Result};

/// 解析数据库文件位置：系统应用数据目录。
/// 放这里可随用户隔离，且不受安装目录写权限限制。
pub fn resolve_db_path(app: &AppHandle) -> Result<PathBuf> {
    let dir = app
        .path()
        .app_data_dir()
        .map_err(|e| AppError::Message(format!("无法获取应用数据目录：{e}")))?;
    fs::create_dir_all(&dir)?;
    Ok(dir.join("notedash.db"))
}

/// 打开数据库并完成建表/索引，首次运行自动初始化
pub fn init_db(path: &PathBuf) -> Result<Connection> {
    let conn = Connection::open(path)?;
    conn.execute_batch(
        "PRAGMA journal_mode = WAL;
         CREATE TABLE IF NOT EXISTS notes (
             id         INTEGER PRIMARY KEY AUTOINCREMENT,
             title      TEXT NOT NULL DEFAULT '',
             content    TEXT NOT NULL DEFAULT '',
             created_at TEXT NOT NULL,
             updated_at TEXT NOT NULL
         );
         CREATE INDEX IF NOT EXISTS idx_notes_updated_at ON notes(updated_at DESC);
         CREATE TABLE IF NOT EXISTS settings (
             key   TEXT PRIMARY KEY,
             value TEXT NOT NULL
         );",
    )?;
    Ok(conn)
}
