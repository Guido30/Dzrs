// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod models;
mod types;

use crate::config::DzrsConfiguration;
use crate::models::slavart_api::Search;
use crate::types::files::{self, DzrsTrackObject, DzrsTrackObjectTagState, DzrsTrackObjectWrapper};
use crate::types::slavart::SlavartDownloadItems;
use crate::types::tags::{DeezerTagger, DzrsTrackObjectTags};

use notify::{recommended_watcher, RecommendedWatcher, RecursiveMode, Watcher};
use std::env;
use std::env::consts::OS;
use std::fs::File;
use std::io::Write;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Arc, Mutex};
use tauri::{Manager, State, Window};

pub fn platform_app_dir() -> PathBuf {
    let mut path = PathBuf::new();
    match OS {
        "linux" => {
            if let Ok(xdg_data_home) = env::var("XDG_DATA_HOME") {
                path.push(xdg_data_home);
            } else if let Ok(home) = env::var("HOME") {
                path.push(home);
                path.push(".local/share");
            }
            path.push("Dzrs");
        }
        "macos" => {
            if let Ok(home) = env::var("HOME") {
                path.push(home);
                path.push("Library/Application Support");
                path.push("Dzrs");
            }
        }
        "windows" => {
            if let Ok(appdata) = env::var("APPDATA") {
                path.push(appdata);
                path.push("Dzrs");
            }
        }
        _ => (),
    }
    path
}

pub fn browse<P: AsRef<Path>>(path: P) -> Result<(), String> {
    let path = path.as_ref().to_str().unwrap();
    let cmd = match OS {
        "linux" => "xdg-open",
        "macos" => "open",
        "windows" => "explorer",
        _ => return Err("Unsupported platform".into()),
    };
    match Command::new(cmd).arg(path).spawn() {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

// Commands for manipulating the inner DzrsTrackObjectWrapper from front-end
#[tauri::command]
async fn tracks_clear(tracks: State<'_, Mutex<DzrsTrackObjectWrapper>>) -> Result<(), ()> {
    let mut t = tracks.lock().unwrap();
    t.clear();
    Ok(())
}

#[tauri::command]
async fn tracks_replace(
    path: String,
    tracks: State<'_, Mutex<DzrsTrackObjectWrapper>>,
    configuration: State<'_, Mutex<DzrsConfiguration>>,
) -> Result<(), String> {
    let conf = configuration.lock().unwrap().parsed();
    let mut t = tracks.lock().unwrap();
    t.replace_track(&path)?;
    match t.get_track_obj_mut(&path) {
        Some(tr) => {
            // Try loading tags, error is ignored for non-flac
            let _ = tr.load_tags(&conf);
        }
        None => (),
    }
    Ok(())
}

#[tauri::command]
async fn tracks_insert(
    path: String,
    tracks: State<'_, Mutex<DzrsTrackObjectWrapper>>,
    configuration: State<'_, Mutex<DzrsConfiguration>>,
) -> Result<(), String> {
    let conf = configuration.lock().unwrap().parsed();
    let mut t = tracks.lock().unwrap();
    t.insert_track(&path)?;
    match t.get_track_obj_mut(&path) {
        Some(tr) => {
            // Try loading tags, error is ignored for non-flac
            let _ = tr.load_tags(&conf);
        }
        None => (),
    }
    Ok(())
}

#[tauri::command]
async fn tracks_remove(path: String, tracks: State<'_, Mutex<DzrsTrackObjectWrapper>>) -> Result<(), String> {
    let mut t = tracks.lock().unwrap();
    t.remove_track(path)?;
    Ok(())
}

#[tauri::command]
async fn tracks_get(
    paths: Option<Vec<String>>,
    tracks: State<'_, Mutex<DzrsTrackObjectWrapper>>,
) -> Result<Vec<DzrsTrackObject>, ()> {
    match paths {
        Some(ps) => {
            let t = tracks.lock().unwrap();
            let mut trs = Vec::new();
            for p in ps {
                match t.get_track_obj(&p).map(|tr| tr.to_owned()) {
                    Some(tr) => trs.push(tr),
                    None => (),
                }
            }
            Ok(trs)
        }
        None => Ok(tracks.lock().unwrap().items.clone()),
    }
}

#[tauri::command]
async fn tracks_get_dir(
    dir: Option<String>,
    tracks: State<'_, Mutex<DzrsTrackObjectWrapper>>,
    configuration: State<'_, Mutex<DzrsConfiguration>>,
) -> Result<Vec<DzrsTrackObject>, String> {
    let conf = configuration.lock().unwrap().parsed();
    let mut t = tracks.lock().unwrap();
    let dir = match dir {
        Some(p) => p,
        None => conf.directory_view_path.clone(),
    };
    match DzrsTrackObjectWrapper::new(dir) {
        Ok(mut tr) => {
            tr.iter_mut().for_each(|track| {
                if track.file_extension == "flac" {
                    let _ = track.load_tags(&conf);
                }
            });
            *t = tr.clone();
            Ok(tr.items)
        }
        Err(err) => Err(err),
    }
}

#[tauri::command]
async fn tracks_object() -> Result<DzrsTrackObject, ()> {
    Ok(DzrsTrackObject::default())
}

// Commands for manipulating the inner DzrsConfiguration from front-end
#[tauri::command]
async fn config_get(configuration: State<'_, Mutex<DzrsConfiguration>>) -> Result<String, String> {
    let conf = configuration.lock().unwrap().clone();
    serde_json::to_string(&conf).map_err(|err| err.to_string())
}

#[tauri::command]
async fn config_set(key: String, value: String, config: State<'_, Mutex<DzrsConfiguration>>) -> Result<(), String> {
    let mut conf = config.lock().unwrap();
    conf.update(key, value);
    conf.save().map_err(|err| err.to_string())
}

// Fetch tags from deezer and apply them into the inner DzrsTrackObjects for each loaded path
// Errors with a vector of each request that failed
#[tauri::command]
async fn tracks_fetch(
    paths: Vec<String>,
    tracks: State<'_, Mutex<DzrsTrackObjectWrapper>>,
    tagger: State<'_, DeezerTagger>,
    config: State<'_, Mutex<DzrsConfiguration>>,
) -> Result<(), Vec<String>> {
    let t = tracks.lock().unwrap().clone();
    let conf = config.lock().unwrap().parsed();
    let mut errors = Vec::new();
    let mut trs = Vec::new();

    // For each path fetch tags from deezer and create an owned updated version of DzrsTrackObject, stored into trs
    // to later update the inner DzrsTrackObjectWrapper
    // NOTE The mutex is immediately released and we work on a cloned version of DzrsTrackObjectWrapper, this allows us
    // to call async methods, in this case fetch_by_query, and after the async calls have been made, the mutex gets locked again
    // and the inner DzrsTrackObjectWrapper gets updated with the new values from deezer
    for p in paths {
        match t.get_track_obj(&p) {
            Some(tr) => {
                // Fetch tags from deezer using the track metadata
                let mut tr = tr.to_owned();
                let tr_meta = (tr.tags.title.deref(), tr.tags.album.deref(), tr.tags.artist.deref());
                let payload = tagger.fetch_by_query(tr_meta.0, tr_meta.1, tr_meta.2).await;
                // Update the DzrsTrackObject using the fetched tags
                match payload {
                    Ok(payload) => {
                        tr.tags_deezer.apply_deezer(payload.0.clone(), &conf);
                        tr.tags_to_save.apply_deezer(payload.0, &conf);
                        tr.tags_status = DzrsTrackObjectTagState::Matched;
                        tr.tags_sources = payload.1;
                        trs.push(tr);
                    }
                    Err(err) => {
                        tr.tags_status = DzrsTrackObjectTagState::Unsuccessfull;
                        trs.push(tr);
                        errors.push(err);
                    }
                };
            }
            _ => (),
        };
    }

    // Swap inner DzrsTrackObjects with updated ones
    let mut t = tracks.lock().unwrap();
    trs.into_iter()
        .for_each(|tr| t.replace_track_obj(tr).map_err(|err| errors.push(err)).unwrap());

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

// Saves given tags into a file and updates the inner DzrsTrackObject to match the saved file
#[tauri::command]
async fn save_tags(
    path: String,
    tags: DzrsTrackObjectTags,
    tracks: State<'_, Mutex<DzrsTrackObjectWrapper>>,
    config: State<'_, Mutex<DzrsConfiguration>>,
) -> Result<(), String> {
    let conf = config.lock().unwrap().parsed();
    // Save tags into the flac
    files::save_tags(&path, &tags)?;
    // Update the loaded track to match the saved file
    let mut track_ = DzrsTrackObject::new(&path)?;
    track_.load_tags(&conf)?;
    let mut t = tracks.lock().unwrap();
    let tr = match t.get_track_obj_mut(&path) {
        Some(tr) => tr,
        None => return Err(format!("Cannot find inner DzrsTrackObject for {}", path)),
    };
    tr.tags = track_.tags;
    tr.tags_pictures = track_.tags_pictures;
    tr.tags_to_save = track_.tags_to_save;
    tr.tags_status = DzrsTrackObjectTagState::Finalized;
    Ok(())
}

#[tauri::command]
async fn get_slavart_tracks(query: String) -> Result<SlavartDownloadItems, String> {
    let response = reqwest::get(format!("https://slavart.gamesdrive.net/api/search?q={query}")).await;
    match response {
        Ok(r) => {
            let body = r.text().await;
            match body {
                Ok(b) => {
                    let search: Result<Search, serde_json::Error> = serde_json::from_str(b.as_str());
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
    let file_path =
        PathBuf::from(configuration.lock().unwrap().download_path.clone()).join(format!("{}.flac", filename));
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
    if !response.status().is_success() {
        return Err(format!("Error {} for {}", response.status().as_str(), url));
    }
    let content_length: u64 = response.content_length().unwrap_or_default();
    let bytes = response.bytes().await.unwrap();
    let res_length = bytes.len();
    if content_length != res_length as u64 || res_length <= 1024 {
        return Err(format!("Download failed, received {} / {} bytes", res_length, content_length).into());
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

// Reinitializes a new file watcher using a given directory
// To avoid unnecessary triggers when manipulating the inner DzrsTrackObjects from the frontend
// the event kind of Modify will be ignored, the downside is that if the user manually renames
// a file or modifies it in any way other than using dzrs, the changes will not be
// immediately reflected in the frontend
#[tauri::command]
async fn watch_dir(
    window: Window,
    dir: String,
    watcher: State<'_, Arc<Mutex<Option<RecommendedWatcher>>>>,
) -> Result<(), String> {
    let mut w: RecommendedWatcher =
        match recommended_watcher(move |res: Result<notify::Event, notify::Error>| match res {
            Ok(e) => {
                let _ = window.emit("watcher_triggered", e);
            }
            _ => (),
        }) {
            Ok(w) => w,
            Err(err) => return Err(err.to_string()),
        };

    w.watch(Path::new(&dir), RecursiveMode::NonRecursive).unwrap();

    let mut guard = watcher.lock().unwrap();
    *guard = Some(w);
    Ok(())
}

#[tauri::command]
async fn browse_cmd(path: String) -> Result<(), String> {
    match browse(path) {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

fn main() {
    let app_dir = platform_app_dir();
    let config_path = app_dir.join("config.json");
    if !config_path.exists() {
        let _ = std::fs::create_dir_all(config_path.parent().unwrap()); //safe unwrap
        let _ = std::fs::write(config_path.clone(), b"");
    }
    let config: Mutex<DzrsConfiguration> = Mutex::new(DzrsConfiguration::load(config_path));
    let tracks_obj: Mutex<DzrsTrackObjectWrapper> = Mutex::new(DzrsTrackObjectWrapper::default());
    let tagger: DeezerTagger = DeezerTagger::new();
    let watcher: Arc<Mutex<Option<RecommendedWatcher>>> = Arc::new(Mutex::new(None));

    tauri::Builder::default()
        .setup(|app| {
            // Open Devtools in debug builds
            #[cfg(debug_assertions)]
            {
                app.get_window("main").unwrap().open_devtools();
            }
            Ok(())
        })
        .manage(config)
        .manage(tracks_obj)
        .manage(tagger)
        .manage(watcher.clone())
        .invoke_handler(tauri::generate_handler![
            tracks_clear,
            tracks_replace,
            tracks_insert,
            tracks_remove,
            tracks_get,
            tracks_get_dir,
            tracks_object,
            config_get,
            config_set,
            tracks_fetch,
            save_tags,
            get_slavart_tracks,
            download_track,
            watch_dir,
            browse_cmd,
        ])
        .run(tauri::generate_context!())
        .expect("error while running dzrs");
}
