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
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(win) = app.get_webview_window("main") {
                let _ = win.unminimize();
                let _ = win.show();
                let _ = win.set_focus();
            }
        }))
        .setup(|app| {
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            app.manage(commands::TranslateState::new());

            if let Some(win) = app.get_webview_window("main") {
                let _ = win.show();
                let _ = win.set_focus();
            }
            Ok(())
        })
        .on_window_event(|window, event| {
            match event {
                tauri::WindowEvent::CloseRequested { api, .. } => {
                    api.prevent_close();
                    let _ = window.hide();
                }
                tauri::WindowEvent::Moved(_) => {
                    #[cfg(target_os = "windows")]
                    if window.is_minimized().unwrap_or(false) {
                        let _ = window.hide();
                    }
                }
                _ => {}
            }
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
