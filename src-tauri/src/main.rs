// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod models;

use std::path::{Path, PathBuf};
use tauri::api::dialog::blocking::FileDialogBuilder;
use tauri::{
    CustomMenuItem, Menu, MenuItem, Submenu, WindowBuilder, WindowEvent,
};

// #[tauri::command]
// async fn get_slavart_items(
//     query: &str,
// ) -> Vec<models::slavart::SlavartDownloadItem> {
// }

fn build_menubar() -> Menu {
    Menu::new()
        .add_submenu(Submenu::new(
            "File",
            Menu::new()
                .add_item(
                    CustomMenuItem::new("open_folder", "Open Folder")
                        .accelerator("O"),
                )
                .add_item(
                    CustomMenuItem::new("open_files", "Open Files")
                        .accelerator("F"),
                )
                .add_item(
                    CustomMenuItem::new("download", "Download")
                        .accelerator("D"),
                ),
        ))
        .add_submenu(Submenu::new(
            "Edit",
            Menu::new()
                .add_native_item(MenuItem::Copy)
                .add_native_item(MenuItem::Cut)
                .add_native_item(MenuItem::Paste),
        ))
        .add_submenu(Submenu::new(
            "Options",
            Menu::new().add_item(CustomMenuItem::new("options", "Options")),
        ))
        .add_submenu(Submenu::new(
            "Help",
            Menu::new().add_item(CustomMenuItem::new("about", "About")),
        ))
}

fn build_file_dialog() -> FileDialogBuilder {
    let dialog_path: PathBuf;
    match tauri::api::path::audio_dir() {
        Some(audio_path) => dialog_path = audio_path,
        None => dialog_path = tauri::api::path::home_dir().unwrap(),
    }

    let file_dialog = FileDialogBuilder::new()
        .add_filter("", &["mp3", "flac"])
        .set_directory(dialog_path);

    file_dialog
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();
            let path_resolver = app_handle.path_resolver();
            let config_path =
                path_resolver.app_config_dir().unwrap().join("config.ini");

            if !config_path.exists() {
                let _ = std::fs::create_dir_all(config_path.parent().unwrap()); //safe unwrap
                let _ = std::fs::write(config_path.clone(), b"");
            }

            let app_config = config::Config::builder()
                .set_default("default", "1")?
                .add_source(config::File::new(
                    config_path.to_str().unwrap(),
                    config::FileFormat::Ini,
                ))
                .build()
                .unwrap();

            let menu = build_menubar();
            let main_win = WindowBuilder::new(
                app,
                "main".to_string(),
                tauri::WindowUrl::App("".into()),
            )
            .menu(menu)
            .inner_size(1200.0, 1000.0)
            .min_inner_size(700.0, 600.0)
            .fullscreen(false)
            .resizable(true)
            .title("Dzrs")
            .build()?;

            main_win.on_window_event(move |event| match event {
                WindowEvent::CloseRequested { .. } => app_handle.exit(0),
                _ => (),
            });

            let download_win = WindowBuilder::new(
                app,
                "download".to_string(),
                tauri::WindowUrl::App("download".into()),
            )
            .min_inner_size(665.0, 520.0)
            .fullscreen(false)
            .resizable(true)
            .visible(false)
            .title("Download")
            .build()?;

            let download_win_ = download_win.clone();
            download_win.on_window_event(move |event| match event {
                WindowEvent::CloseRequested { api, .. } => {
                    api.prevent_close();
                    let _ = download_win_.hide();
                }
                _ => (),
            });

            let settings_win = WindowBuilder::new(
                app,
                "settings".to_string(),
                tauri::WindowUrl::App("settings".into()),
            )
            .min_inner_size(600.0, 400.0)
            .fullscreen(false)
            .resizable(true)
            .visible(false)
            .title("Settings")
            .build()?;

            let settings_win_ = settings_win.clone();
            settings_win.on_window_event(move |event| match event {
                WindowEvent::CloseRequested { api, .. } => {
                    api.prevent_close();
                    let _ = settings_win_.hide();
                }
                _ => (),
            });

            let about_win = WindowBuilder::new(
                app,
                "about".to_string(),
                tauri::WindowUrl::App("about".into()),
            )
            .inner_size(400.0, 200.0)
            .fullscreen(false)
            .resizable(false)
            .visible(false)
            .title("About")
            .build()?;

            let about_win_ = about_win.clone();
            about_win.on_window_event(move |event| match event {
                WindowEvent::CloseRequested { api, .. } => {
                    api.prevent_close();
                    let _ = about_win_.hide();
                }
                _ => (),
            });

            main_win.on_menu_event(move |e| match e.menu_item_id() {
                "open_folder" => {
                    let file_dialog = build_file_dialog();
                    let folder = file_dialog.pick_folder();
                    println!("{:?}", folder);
                }
                "open_files" => {
                    let file_dialog = build_file_dialog();
                    let files = file_dialog.pick_files();
                    println!("{:?}", files);
                }
                "download" => {
                    download_win.show().unwrap();
                }
                "options" => {
                    settings_win.show().unwrap();
                }
                "about" => {
                    about_win.show().unwrap();
                }
                _ => (),
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
