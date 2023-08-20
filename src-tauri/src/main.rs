// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod models;
mod slavart_api;

use models::slavart::Search;
use slavart_api::SlavartDownloadItems;

use std::path::{Path, PathBuf};
use tauri::api::dialog::blocking::FileDialogBuilder;
use tauri::{
    CustomMenuItem, Menu, MenuItem, Submenu, WindowBuilder, WindowEvent,
};

#[tauri::command]
async fn get_slavart_tracks(query: String) -> Result<SlavartDownloadItems, ()> {
    let response = reqwest::get(format!(
        "https://slavart.gamesdrive.net/api/search?q={query}"
    ))
    .await;
    match response {
        Ok(r) => {
            let body = r.text().await;
            match body {
                Ok(b) => {
                    let search: Result<Search, serde_json::Error> =
                        serde_json::from_str(b.as_str());
                    match search {
                        Ok(s) => {
                            let tracks = SlavartDownloadItems::from(s);
                            Ok(tracks)
                        }
                        Err(_) => return Err(()),
                    }
                }
                Err(_) => return Err(()),
            }
        }
        Err(_) => return Err(()),
    }
}

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

            let main_win_ = main_win.clone();
            main_win.on_menu_event(move |e| match e.menu_item_id() {
                "open_folder" => {
                    let _ = main_win_.emit("page-change", "/");
                    let file_dialog = build_file_dialog();
                    let folder = file_dialog.pick_folder();
                    println!("{:?}", folder);
                }
                "open_files" => {
                    let _ = main_win_.emit("page-change", "/");
                    let file_dialog = build_file_dialog();
                    let files = file_dialog.pick_files();
                    println!("{:?}", files);
                }
                "download" => {
                    let _ = main_win_.emit("page-change", "download");
                }
                "options" => {
                    let _ = main_win_.emit("page-change", "settings");
                }
                "about" => {
                    let _ = main_win_.emit("page-change", "about");
                }
                _ => (),
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_slavart_tracks])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
