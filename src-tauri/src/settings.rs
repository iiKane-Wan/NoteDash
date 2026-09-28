use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

use crate::error::Result;

/// 默认全局快捷键（唤起主窗口）
pub const DEFAULT_SHORTCUT: &str = "Ctrl+Shift+N";

/// 默认应用内快捷键（隐藏主界面）
pub const DEFAULT_HIDE_SHORTCUT: &str = "Escape";

/// 应用设置，持久化在 settings 表（键值对形式，便于后续扩展）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Settings {
    /// 开机自启
    pub autostart: bool,
    /// 全局快捷键，空串表示未设置
    pub global_shortcut: String,
    /// 隐藏主界面快捷键（应用内生效），空串表示未设置
    pub hide_shortcut: String,
    /// 关闭窗口时最小化到托盘
    pub close_to_tray: bool,
    /// 主题：light | dark | system
    pub theme: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            autostart: false,
            global_shortcut: DEFAULT_SHORTCUT.to_string(),
            hide_shortcut: DEFAULT_HIDE_SHORTCUT.to_string(),
            close_to_tray: true,
            theme: "system".to_string(),
        }
    }
}

impl Settings {
    /// 读取设置；从未写入过的键使用默认值
    pub fn load(conn: &Connection) -> Result<Self> {
        let mut settings = Settings::default();
        let mut stmt = conn.prepare("SELECT key, value FROM settings")?;
        let rows = stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?;
        for row in rows {
            let (key, value) = row?;
            match key.as_str() {
                "autostart" => settings.autostart = value == "true",
                "global_shortcut" => settings.global_shortcut = value,
                "hide_shortcut" => settings.hide_shortcut = value,
                "close_to_tray" => settings.close_to_tray = value == "true",
                "theme" => settings.theme = value,
                _ => {}
            }
        }
        Ok(settings)
    }

    /// 全量写回，键存在则更新
    pub fn save(&self, conn: &Connection) -> Result<()> {
        let entries = [
            ("autostart", self.autostart.to_string()),
            ("global_shortcut", self.global_shortcut.clone()),
            ("hide_shortcut", self.hide_shortcut.clone()),
            ("close_to_tray", self.close_to_tray.to_string()),
            ("theme", self.theme.clone()),
        ];
        for (key, value) in entries {
            conn.execute(
                "INSERT INTO settings (key, value) VALUES (?1, ?2)
                 ON CONFLICT(key) DO UPDATE SET value = excluded.value",
                params![key, value],
            )?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_include_hide_shortcut() {
        let settings = Settings::default();
        assert_eq!(settings.hide_shortcut, DEFAULT_HIDE_SHORTCUT);
        assert_eq!(settings.global_shortcut, DEFAULT_SHORTCUT);
    }

    /// 前端依赖 camelCase 字段名，遗漏会直接导致设置项失效
    #[test]
    fn serializes_with_camel_case_keys() {
        let json = serde_json::to_string(&Settings::default()).unwrap();
        for key in [
            "autostart",
            "globalShortcut",
            "hideShortcut",
            "closeToTray",
            "theme",
        ] {
            assert!(json.contains(key), "序列化缺少字段 {key}：{json}");
        }
    }

    #[test]
    fn round_trips_through_sqlite() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            "CREATE TABLE settings (key TEXT PRIMARY KEY, value TEXT NOT NULL);",
        )
        .unwrap();

        let mut settings = Settings::default();
        settings.hide_shortcut = "Ctrl+H".to_string();
        settings.close_to_tray = false;
        settings.save(&conn).unwrap();

        let loaded = Settings::load(&conn).unwrap();
        assert_eq!(loaded.hide_shortcut, "Ctrl+H");
        assert!(!loaded.close_to_tray);
        assert_eq!(loaded.global_shortcut, DEFAULT_SHORTCUT);
    }
}
