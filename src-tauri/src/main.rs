// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod helpers;
mod models;
mod slavart_api;

use config::DzrsConfiguration;
use models::slavart::Search;
use slavart_api::SlavartDownloadItems;

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::api::dialog::blocking::{FileDialogBuilder, MessageDialogBuilder};
use tauri::api::dialog::{MessageDialogButtons, MessageDialogKind};
use tauri::{State, Window};

#[tauri::command]
async fn get_slavart_tracks(query: String) -> Result<SlavartDownloadItems, String> {
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
    let url = format!("https://slavart-api.gamesdrive.net/api/download/track?id={id}");
    let file_path = PathBuf::from(configuration.lock().unwrap().download_path.clone())
        .join(format!("{}.flac", filename));
    if file_path.exists() && (configuration.lock().unwrap().overwrite_downloads == "false") {
        println!("{:?}", configuration.lock().unwrap().overwrite_downloads);
        return Err(format!("File {} already exists", filename));
    }
    let response = match reqwest::get(&url).await {
        Ok(res) => res,
        Err(err) => return Err(err.to_string()),
    };
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
    match configuration.lock().unwrap().save() {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
async fn show_window(window: Window) -> Result<(), String> {
    match window.show() {
        Ok(()) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
async fn open_explorer(path: String) -> Result<(), String> {
    match helpers::open_explorer(path) {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
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
        .manage(Mutex::new(configuration))
        .invoke_handler(tauri::generate_handler![
            show_window,
            get_config_values,
            get_config,
            update_config,
            open_explorer,
            get_slavart_tracks,
            download_track,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
