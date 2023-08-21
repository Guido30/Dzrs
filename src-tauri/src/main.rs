// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod helpers;
mod models;
mod slavart_api;

use config::DzrsConfiguration;
use models::slavart::Search;
use slavart_api::SlavartDownloadItems;
use tauri::api::file;

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::api::dialog::blocking::{FileDialogBuilder, MessageDialogBuilder};
use tauri::api::dialog::{MessageDialogButtons, MessageDialogKind};
use tauri::State;
use tauri::WindowBuilder;

#[tauri::command]
async fn get_slavart_tracks(
    query: String,
) -> Result<SlavartDownloadItems, String> {
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
                        Err(err) => return Err(err.to_string()),
                    }
                }
                Err(err) => return Err(err.to_string()),
            }
        }
        Err(err) => return Err(err.to_string()),
    }
}

#[tauri::command]
async fn download_track(
    id: u64,
    filename: String,
    configuration: State<'_, Mutex<DzrsConfiguration>>,
) -> Result<(), String> {
    let url = format!(
        "https://slavart-api.gamesdrive.net/api/download/track?id={id}"
    );
    let response = match reqwest::get(&url).await {
        Ok(res) => res,
        Err(err) => return Err(err.to_string()),
    };
    let file_path =
        PathBuf::from(configuration.lock().unwrap().download_path.clone())
            .join(format!("{}.flac", filename));
    let mut file = match File::create(file_path) {
        Ok(file) => file,
        Err(err) => return Err(err.to_string()),
    };
    let bytes: Vec<u8> = response.bytes().await.unwrap().into();
    match file.write_all(&bytes) {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
async fn get_config_values(
    configuration: State<'_, Mutex<DzrsConfiguration>>,
) -> Result<String, String> {
    let conf = configuration.lock().unwrap().clone();
    match serde_json::to_string(&conf) {
        Ok(res) => Ok(res),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
async fn get_config(
    key: String,
    configuration: State<'_, Mutex<DzrsConfiguration>>,
) -> Result<String, String> {
    match configuration.lock().unwrap().get(key) {
        Ok(res) => Ok(res),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
async fn update_config(
    key: String,
    value: String,
    configuration: State<'_, Mutex<DzrsConfiguration>>,
) -> Result<(), String> {
    configuration.lock().unwrap().update(key, value);
    let _ = configuration.lock().unwrap().save();
    Ok(())
}

fn main() {
    let app_data_path = helpers::app_data_dir();
    let config_path = app_data_path.join("config.json");
    if !config_path.exists() {
        let _ = std::fs::create_dir_all(config_path.parent().unwrap()); //safe unwrap
        let _ = std::fs::write(config_path.clone(), b"");
    }
    let configuration = match DzrsConfiguration::from_file(config_path) {
        Ok(conf) => conf,
        Err(_) => DzrsConfiguration::default(),
    };

    tauri::Builder::default()
        .setup(|app| {
            let menu = helpers::build_menubar();
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
                    let file_dialog = helpers::build_file_dialog();
                    let folder = file_dialog.pick_folder();
                    println!("{:?}", folder);
                }
                "open_files" => {
                    let _ = main_win_.emit("page-change", "/");
                    let file_dialog = helpers::build_file_dialog();
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
        .manage(Mutex::new(configuration))
        .invoke_handler(tauri::generate_handler![
            get_config_values,
            get_config,
            update_config,
            get_slavart_tracks,
            download_track,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
