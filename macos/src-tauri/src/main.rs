// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::menu::{MenuBuilder, MenuItemBuilder, SubmenuBuilder};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let quit_i = MenuItemBuilder::with_id("quit", "Quit")
                .accelerator("CmdOrCtrl+Q")
                .build(app)?;

            let app_submenu = SubmenuBuilder::new(app, "DeepTutor")
                .item(&quit_i)
                .build()?;

            let menu = MenuBuilder::new(app)
                .item(&app_submenu)
                .build()?;

            app.set_menu(menu)?;

            Ok(())
        })
        .on_menu_event(|app, event| {
            if event.id().as_ref() == "quit" {
                app.exit(0);
            }
        })
        .plugin(tauri_plugin_log::Builder::new().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
