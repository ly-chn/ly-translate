use serde_json::Value;
use tauri::{AppHandle, Manager};

use crate::config::AppSettings;
use crate::dict::{self, WordDefinition};
use crate::translate_mod;

fn get_config(app: &AppHandle) -> AppSettings {
    load_settings_inner(app).unwrap_or_default()
}

fn load_settings_inner(app: &AppHandle) -> Option<AppSettings> {
    let dir = app.path().app_config_dir().ok()?;
    let path = dir.join("settings.json");
    if path.exists() {
        let data = std::fs::read_to_string(&path).ok()?;
        serde_json::from_str(&data).ok()
    } else {
        None
    }
}

#[tauri::command]
pub async fn translate(
    app: AppHandle,
    text: String,
    from: String,
    to: String,
    style: String,
) -> Result<String, String> {
    let config = get_config(&app);
    translate_mod::translate(&text, &from, &to, &style, &config.model)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn lookup_word(
    app: AppHandle,
    word: String,
    lang: String,
) -> Result<WordDefinition, String> {
    let config = get_config(&app);
    dict::lookup(&word, &lang, &config.model)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn load_settings(app: AppHandle) -> Option<AppSettings> {
    load_settings_inner(&app)
}

#[tauri::command]
pub fn save_settings(app: AppHandle, settings: Value) -> Result<(), String> {
    let dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let path = dir.join("settings.json");
    let json = serde_json::to_string_pretty(&settings).map_err(|e| e.to_string())?;
    std::fs::write(&path, json).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn export_settings(app: AppHandle) -> Result<String, String> {
    let dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
    let path = dir.join("settings.json");
    if !path.exists() {
        return Err("no settings file".into());
    }
    let json = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
    use base64::Engine;
    Ok(base64::engine::general_purpose::STANDARD.encode(json.as_bytes()))
}

#[tauri::command]
pub fn import_settings(app: AppHandle, data: String) -> Result<(), String> {
    use base64::Engine;
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(data.trim())
        .map_err(|_| "invalid base64")?;
    let json_str = String::from_utf8(bytes).map_err(|_| "invalid utf8")?;
    let _: Value = serde_json::from_str(&json_str).map_err(|e| format!("invalid json: {e}"))?;
    let dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let path = dir.join("settings.json");
    std::fs::write(&path, &json_str).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn init_tray(app: AppHandle) -> Result<(), String> {
    crate::tray::setup(&app).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_always_on_top(app: AppHandle, enabled: bool) -> Result<(), String> {
    let win = app.get_webview_window("main").ok_or("window not found")?;
    win.set_always_on_top(enabled).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn toggle_autostart(app: AppHandle, enabled: bool) -> Result<(), String> {
    use tauri_plugin_autostart::ManagerExt;
    let autostart = app.autolaunch();
    if enabled {
        autostart.enable().map_err(|e| e.to_string())
    } else {
        autostart.disable().map_err(|e| e.to_string())
    }
}
