use chrono::Local;
use rusqlite::params;
use tauri::{AppHandle, Manager, State};
use tauri_plugin_autostart::ManagerExt;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

use crate::error::{AppError, Result};
use crate::models::{Note, NoteSummary};
use crate::settings::Settings;
use crate::AppState;

/// 本地时间戳（ISO8601，毫秒精度），用于排序与展示
fn now() -> String {
    Local::now().format("%Y-%m-%dT%H:%M:%S%.3f").to_string()
}

/// 从 Mutex 取连接；即使锁中毒也继续复用数据，避免应用整体崩溃
fn lock(state: &AppState) -> std::sync::MutexGuard<'_, rusqlite::Connection> {
    state.conn.lock().unwrap_or_else(|e| e.into_inner())
}

/// 列表查询：keyword 为空返回全部，否则模糊匹配标题或正文
#[tauri::command]
pub fn list_notes(state: State<'_, AppState>, keyword: Option<String>) -> Result<Vec<NoteSummary>> {
    let conn = lock(&state);
    let kw = keyword.unwrap_or_default().trim().to_string();
    let like = format!("%{kw}%");
    let mut stmt = conn.prepare(
        "SELECT id, title, updated_at
           FROM notes
          WHERE ?1 = '' OR title LIKE ?2 OR content LIKE ?2
          ORDER BY updated_at DESC",
    )?;
    let rows = stmt.query_map(params![kw, like], |r| {
        Ok(NoteSummary {
            id: r.get(0)?,
            title: r.get(1)?,
            updated_at: r.get(2)?,
        })
    })?;
    let mut out = Vec::new();
    for row in rows {
        out.push(row?);
    }
    Ok(out)
}

/// 读取单条完整笔记
#[tauri::command]
pub fn get_note(state: State<'_, AppState>, id: i64) -> Result<Option<Note>> {
    let conn = lock(&state);
    let mut stmt = conn.prepare(
        "SELECT id, title, content, created_at, updated_at FROM notes WHERE id = ?1",
    )?;
    let mut rows = stmt.query_map(params![id], |r| {
        Ok(Note {
            id: r.get(0)?,
            title: r.get(1)?,
            content: r.get(2)?,
            created_at: r.get(3)?,
            updated_at: r.get(4)?,
        })
    })?;
    match rows.next() {
        Some(row) => Ok(Some(row?)),
        None => Ok(None),
    }
}

/// 新建空笔记：标题留空，由前端提示用户输入
#[tauri::command]
pub fn create_note(state: State<'_, AppState>) -> Result<Note> {
    let conn = lock(&state);
    let ts = now();
    conn.execute(
        "INSERT INTO notes (title, content, created_at, updated_at) VALUES ('', '', ?1, ?1)",
        params![ts],
    )?;
    let id = conn.last_insert_rowid();
    Ok(Note {
        id,
        title: String::new(),
        content: String::new(),
        created_at: ts.clone(),
        updated_at: ts,
    })
}

/// 更新标题与正文，并刷新 updated_at（列表据此重排）
#[tauri::command]
pub fn update_note(
    state: State<'_, AppState>,
    id: i64,
    title: String,
    content: String,
) -> Result<NoteSummary> {
    let conn = lock(&state);
    let ts = now();
    let affected = conn.execute(
        "UPDATE notes SET title = ?1, content = ?2, updated_at = ?3 WHERE id = ?4",
        params![title, content, ts, id],
    )?;
    if affected == 0 {
        return Err(crate::error::AppError::Message(format!("笔记不存在：{id}")));
    }
    Ok(NoteSummary {
        id,
        title,
        updated_at: ts,
    })
}

/// 删除笔记
#[tauri::command]
pub fn delete_note(state: State<'_, AppState>, id: i64) -> Result<()> {
    let conn = lock(&state);
    conn.execute("DELETE FROM notes WHERE id = ?1", params![id])?;
    Ok(())
}

/* ==================== 设置 ==================== */

/// 读取全部设置
#[tauri::command]
pub fn get_settings(state: State<'_, AppState>) -> Result<Settings> {
    let conn = lock(&state);
    Settings::load(&conn)
}

/// 保存设置并即时应用副作用（开机自启、全局快捷键）。
/// 任一步骤失败都会直接返回错误且不做持久化，前端据此提示用户。
#[tauri::command]
pub fn update_settings(
    app: AppHandle,
    state: State<'_, AppState>,
    settings: Settings,
) -> Result<Settings> {
    validate(&settings)?;

    let previous = state
        .settings
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .clone();

    // 仅在变化时重新注册，避免无谓地注销/注册引发瞬时失效
    if previous.global_shortcut != settings.global_shortcut {
        apply_shortcut(&app, &settings.global_shortcut)?;
    }
    if previous.autostart != settings.autostart {
        apply_autostart(&app, settings.autostart)?;
    }

    {
        let conn = lock(&state);
        settings.save(&conn)?;
    }
    {
        let mut cached = state.settings.lock().unwrap_or_else(|e| e.into_inner());
        *cached = settings.clone();
    }
    Ok(settings)
}

/// 隐藏主窗口：由应用内快捷键触发（前端捕获到按键后调用）
#[tauri::command]
pub fn hide_main_window(app: AppHandle) -> Result<()> {
    if let Some(window) = app.get_webview_window("main") {
        window
            .hide()
            .map_err(|e| AppError::Message(format!("隐藏窗口失败：{e}")))?;
    }
    Ok(())
}

/// 校验设置取值合法性
fn validate(settings: &Settings) -> Result<()> {
    if !matches!(settings.theme.as_str(), "light" | "dark" | "system") {
        return Err(AppError::Message(format!("未知的主题选项：{}", settings.theme)));
    }
    // 应用内快捷键允许不带修饰键（默认 Esc），仅做基本合法性校验
    if settings.hide_shortcut.len() > 64 {
        return Err(AppError::Message("隐藏快捷键过长".into()));
    }
    if settings.hide_shortcut.chars().any(|c| c.is_control()) {
        return Err(AppError::Message("隐藏快捷键包含非法字符".into()));
    }
    Ok(())
}

/// 应用全局快捷键：先清空旧注册，再注册新快捷键。
/// 解析失败或注册被占用都会返回可读错误，供前端提示冲突。
pub fn apply_shortcut(app: &AppHandle, shortcut: &str) -> Result<()> {
    let global = app.global_shortcut();
    global
        .unregister_all()
        .map_err(|e| AppError::Message(format!("清理旧快捷键失败：{e}")))?;

    let value = shortcut.trim();
    if value.is_empty() {
        return Ok(());
    }

    let parsed: Shortcut = value
        .parse()
        .map_err(|_| AppError::Message(format!("快捷键格式无效：{value}")))?;

    // 纯字母键会被全局劫持，必须搭配修饰键
    if parsed.mods.is_empty() {
        return Err(AppError::Message(
            "全局快捷键需搭配 Ctrl、Alt 或 Shift 使用".into(),
        ));
    }

    global
        .on_shortcut(parsed, |app, _shortcut, event| {
            if event.state == ShortcutState::Pressed {
                crate::tray::show_main_window(app);
            }
        })
        .map_err(|e| AppError::Message(format!("快捷键已被其他程序占用：{e}")))?;

    Ok(())
}

/// 应用开机自启
fn apply_autostart(app: &AppHandle, enabled: bool) -> Result<()> {
    let manager = app.autolaunch();
    let outcome = if enabled {
        manager.enable()
    } else {
        manager.disable()
    };
    outcome.map_err(|e| AppError::Message(format!("设置开机自启失败：{e}")))
}

/// 启动时以系统实际状态校正开机自启，避免设置与实际不一致
pub fn sync_autostart(app: &AppHandle) -> Option<bool> {
    app.autolaunch().is_enabled().ok()
}

#[cfg(test)]
mod tests {
    use tauri_plugin_global_shortcut::Shortcut;

    /// 前端 ShortcutInput 生成的组合必须都能被后端解析，
    /// 否则用户设置后会静默失效。
    #[test]
    fn frontend_shortcut_formats_are_valid() {
        let samples = [
            "Ctrl+Shift+N",
            "Ctrl+Alt+Space",
            "Alt+F1",
            "Ctrl+Shift+`",
            "Ctrl+Shift+=",
            "Ctrl+Shift+Up",
            "Super+Shift+D",
            "Ctrl+Shift+\\",
            "Ctrl+Shift+;",
            "Ctrl+Shift+[",
        ];
        for value in samples {
            let parsed: Shortcut = value
                .parse()
                .unwrap_or_else(|_| panic!("无法解析快捷键：{value}"));
            assert!(!parsed.mods.is_empty(), "快捷键缺少修饰键：{value}");
        }
    }
}