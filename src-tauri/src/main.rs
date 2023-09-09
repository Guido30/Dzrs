// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod helpers;
mod models;

use config::DzrsConfiguration;
use models::dzrs_files::DzrsFiles;
use models::dzrs_tracks::DzrsTracks;
use models::slavart::SlavartDownloadItems;
use models::slavart_api::Search;

use notify::{recommended_watcher, RecommendedWatcher, RecursiveMode, Watcher};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tauri::{State, Window};

#[tauri::command]
async fn get_dzrs_tracks(paths: Vec<String>) -> Result<DzrsTracks, ()> {
    let flacs = DzrsTracks::from(paths);
    println!("{:?}", flacs);
    Ok(flacs)
}

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
        return Err(format!(
            "File {:?} already exists",
            file_path.file_name().unwrap_or_default()
        ));
    }
    let response = match reqwest::get(&url).await {
        Ok(res) => res,
        Err(err) => return Err(err.to_string()),
    };
    let content_length: u64 = response.content_length().unwrap_or_default();
    let bytes = response.bytes().await.unwrap();
    let res_length = bytes.len();
    if content_length != res_length as u64 {
        return Err(format!(
            "Download request failed, received {} / {}  bytes",
            res_length, content_length
        )
        .into());
    };
    let mut file = match File::create(file_path) {
        Ok(file) => file,
        Err(err) => return Err(err.to_string()),
    };
    let bytes: Vec<u8> = bytes.into();
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

#[tauri::command]
async fn watch_directory(
    window: Window,
    path: String,
    watcher_state: State<'_, Arc<Mutex<Option<RecommendedWatcher>>>>,
) -> Result<(), String> {
    let path = PathBuf::from(path);
    let mut watcher: RecommendedWatcher = recommended_watcher(move |res| match res {
        Ok(_) => {
            let _ = window.emit("watcher_fired", "");
        }
        Err(e) => println!("watch error: {:?}", e),
    })
    .unwrap();

    watcher
        .watch(path.as_path(), RecursiveMode::NonRecursive)
        .unwrap();

    let mut guard = watcher_state.lock().unwrap();
    *guard = Some(watcher);
    Ok(())
}

#[tauri::command]
async fn watcher_get_files(
    path: Option<String>,
    configuration: State<'_, Mutex<DzrsConfiguration>>,
) -> Result<DzrsFiles, ()> {
    let path = match path {
        Some(p) => p,
        None => configuration.lock().unwrap().directory_view_path.clone(),
    };
    let files = DzrsFiles::from(path);
    Ok(files)
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
    let watcher: Arc<Mutex<Option<RecommendedWatcher>>> = Arc::new(Mutex::new(None));

    tauri::Builder::default()
        .manage(Mutex::new(configuration))
        .manage(watcher.clone())
        .invoke_handler(tauri::generate_handler![
            show_window,
            watch_directory,
            watcher_get_files,
            get_config_values,
            get_config,
            update_config,
            open_explorer,
            get_slavart_tracks,
            download_track,
            get_dzrs_tracks,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
