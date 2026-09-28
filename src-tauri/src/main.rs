// 发布版隐藏控制台窗口，开发版保留日志输出
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    notedash_lib::run()
}
