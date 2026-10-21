mod commands;
mod db;
mod error;
mod models;
mod settings;
mod tray;

use std::sync::Mutex;

use rusqlite::Connection;
use tauri::{Emitter, Manager, WindowEvent};
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};

use settings::Settings;

/// 全局状态：数据库连接 + 当前设置缓存。
/// 桌面单用户场景足够，避免引入连接池的额外复杂度。
pub struct AppState {
    pub conn: Mutex<Connection>,
    pub settings: Mutex<Settings>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // 单实例插件必须最先注册：重复启动时不创建新窗口，
        // 而是聚焦已有实例并通知其前端弹出提示。进程结束锁自动释放。
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            tray::show_main_window(app);
            let _ = app.emit("app://already-running", ());
        }))
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            // 开机自启时带上 --silent：启动后只驻留托盘，不弹出主窗口
            Some(vec!["--silent"]),
        ))
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        // 外部链接统一交给系统默认浏览器打开
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let handle = app.handle().clone();

            // 静默启动（开机自启）：窗口保持隐藏、仅驻留托盘，
            // 用户可通过托盘菜单或全局快捷键唤出。手动启动则正常显示。
            let silent = std::env::args().any(|arg| arg == "--silent");
            if !silent {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                }
            }

            // 启动即完成数据库定位与初始化
            let db_path = db::resolve_db_path(&handle)?;
            let conn = db::init_db(&db_path)?;

            let mut settings = Settings::load(&conn)?;
            // 以系统实际状态为准，纠正开机自启设置项
            if let Some(actual) = commands::sync_autostart(&handle) {
                settings.autostart = actual;
                if actual {
                    // 幂等刷新注册表：让已有的启动项升级为带 --silent 参数
                    let _ = handle.autolaunch().enable();
                }
            }
            let _ = settings.save(&conn);

            // 窗口显示前先应用已保存的主题，避免标题栏出现一次颜色跳变
            let theme = match settings.theme.as_str() {
                "light" => Some(tauri::Theme::Light),
                "dark" => Some(tauri::Theme::Dark),
                _ => None, // system：交还系统决定
            };
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_theme(theme);
            }

            // 托盘失败不阻断启动，应用核心功能仍可用
            if let Err(err) = tray::init(app) {
                eprintln!("[NoteDash] 托盘初始化失败：{err}");
            }

            // 注册全局快捷键；失败不阻断启动（例如被其他程序占用）
            if let Err(err) = commands::apply_shortcut(&handle, &settings.global_shortcut) {
                eprintln!("[NoteDash] 全局快捷键注册失败：{err}");
            }

            let _ = app.manage(AppState {
                conn: Mutex::new(conn),
                settings: Mutex::new(settings),
            });
            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                let app = window.app_handle();
                let close_to_tray = app
                    .state::<AppState>()
                    .settings
                    .lock()
                    .unwrap_or_else(|e| e.into_inner())
                    .close_to_tray;
                // 勾选「关闭保留托盘」时拦截关闭并隐藏窗口
                if close_to_tray {
                    api.prevent_close();
                    let _ = window.hide();
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            commands::list_notes,
            commands::get_note,
            commands::create_note,
            commands::update_note,
            commands::delete_note,
            commands::get_settings,
            commands::update_settings,
            commands::hide_main_window,
        ])
        .run(tauri::generate_context!())
        .expect("NoteDash 启动失败");
}