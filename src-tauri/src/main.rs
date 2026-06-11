#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod config;
mod dict;
mod translate_mod;
mod tray;

use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .setup(|app| {
            if let Some(win) = app.get_webview_window("main") {
                if let Ok(Some(monitor)) = win.current_monitor() {
                    let size = monitor.size();
                    let w = (size.width as f64 * 0.5) as u32;
                    let h = (size.height as f64 * 0.5) as u32;
                    let _ = win.set_size(tauri::Size::Physical(tauri::PhysicalSize::new(w, h)));
                    let _ = win.center();
                }
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::translate,
            commands::lookup_word,
            commands::load_settings,
            commands::save_settings,
            commands::export_settings,
            commands::import_settings,
            commands::init_tray,
            commands::set_always_on_top,
            commands::toggle_autostart,
        ])
        .run(tauri::generate_context!())
        .expect("failed to run app");
}
