use tauri::{
    menu::{CheckMenuItem, Menu, MenuItem},
    tray::TrayIconBuilder,
    AppHandle, Emitter, Manager,
};
use tauri_plugin_autostart::ManagerExt;

pub fn setup(app: &AppHandle) -> anyhow::Result<()> {
    let autostart = app.autolaunch();
    let is_autostart = autostart.is_enabled().unwrap_or(false);

    let is_topmost = app
        .get_webview_window("main")
        .and_then(|w| w.is_always_on_top().ok())
        .unwrap_or(false);

    let settings_i = MenuItem::with_id(app, "settings", "设置", true, None::<&str>)?;
    let autostart_i = CheckMenuItem::with_id(
        app,
        "autostart",
        "开机自启动",
        true,
        is_autostart,
        None::<&str>,
    )?;
    let topmost_i =
        CheckMenuItem::with_id(app, "topmost", "窗口置顶", true, is_topmost, None::<&str>)?;
    let quit_i = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&settings_i, &autostart_i, &topmost_i, &quit_i])?;

    let _tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .on_tray_icon_event(|tray, event| {
            if let tauri::tray::TrayIconEvent::Click {
                button: tauri::tray::MouseButton::Left,
                button_state: tauri::tray::MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(win) = app.get_webview_window("main") {
                    if win.is_visible().unwrap_or(false) {
                        let _ = win.hide();
                    } else {
                        let _ = win.show();
                        let _ = win.set_focus();
                    }
                }
            }
        })
        .on_menu_event(move |app, event| match event.id.as_ref() {
            "settings" => {
                if let Some(win) = app.get_webview_window("main") {
                    let _ = win.show();
                    let _ = win.set_focus();
                    let _ = app.emit("open-settings", ());
                }
            }
            "autostart" => {
                let autostart = app.autolaunch();
                let enabled = !autostart.is_enabled().unwrap_or(false);
                if enabled {
                    let _ = autostart.enable();
                } else {
                    let _ = autostart.disable();
                }
                let _ = autostart_i.set_checked(enabled);
            }
            "topmost" => {
                if let Some(win) = app.get_webview_window("main") {
                    let is_top = win.is_always_on_top().unwrap_or(false);
                    let _ = win.set_always_on_top(!is_top);
                    let _ = topmost_i.set_checked(!is_top);
                }
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .build(app)?;

    Ok(())
}
