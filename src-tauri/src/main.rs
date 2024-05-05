// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod types;

use crate::config::DzrsConfiguration;
use crate::types::files::{self, DzrsTrackObject, DzrsTrackObjectTagState, DzrsTrackObjectWrapper};
use crate::types::tags::{DeezerTagger, DzrsTrackObjectTags};

use notify::{recommended_watcher, RecommendedWatcher, RecursiveMode, Watcher};
use regex::Regex;
use std::env;
use std::env::consts::OS;
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
    config: State<'_, Mutex<DzrsConfiguration>>,
) -> Result<Vec<DzrsTrackObject>, String> {
    let conf = config.lock().unwrap().parsed();
    let mut t = tracks.lock().unwrap();
    let dir = match dir {
        Some(p) => p,
        None => conf.directory_view_path.clone(),
    };
    match DzrsTrackObjectWrapper::new(dir) {
        Ok(mut tr) => {
            tr.iter_mut().for_each(|track| {
                // Try loading tags, error is ignored for non-flac
                let _ = track.load_tags(&conf);
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
                let _file_name = tr.file_name.to_owned();
                let re_title = Regex::new(r"[\[\(].*?(?:with|feat).*?[\]\)]").unwrap();
                let re_album = Regex::new(r"[\[\(]?(?i:explicit)[\]\)]?").unwrap();
                // Stripping featured artists from the title and explicit from album, messes up the deezer search
                // Also stripping '&' messes up the url
                let _title = tr.tags.title.to_owned().replace("&", "");
                let _album = tr.tags.album.to_owned().replace("&", "");
                let _artist = tr.tags.artist.to_owned().replace("&", "");
                let _title = re_title.replace_all(&_title, "");
                let _album = re_album.replace_all(&_album, "");
                let file_name = _file_name.strip_suffix(".flac").unwrap();
                let tr_meta = (_title.deref(), _album.deref(), _artist.deref());

                let queries = match (
                    tr_meta.0.is_empty() && tr_meta.1.is_empty() && tr_meta.2.is_empty(),
                    conf.tag_fetch_with_filename,
                ) {
                    (true, true) => (file_name.to_owned(), file_name.to_owned()),
                    _ => (
                        format!(r#"track:"{}" album:"{}" artist:"{}""#, tr_meta.0, tr_meta.1, tr_meta.2),
                        format!(r#"track:"{}" album:"{}""#, tr_meta.0, tr_meta.1),
                    ),
                };
                let payload = match tagger.fetch_by_query(&queries.0).await {
                    Ok(p) => Ok(p),
                    Err(_) => tagger.fetch_by_query(&queries.1).await,
                };
                // Update the DzrsTrackObject using the fetched tags
                match payload {
                    Ok(payload) => {
                        tr.tags_deezer.apply_deezer(payload.0.clone(), &conf);
                        tr.tags_to_save.apply_deezer(payload.0, &conf);
                        match payload.1.len() {
                            l if l > 1 => tr.tags_status = DzrsTrackObjectTagState::Successfull,
                            1 => tr.tags_status = DzrsTrackObjectTagState::Matched,
                            _ => (),
                        }
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

// Fetch possibile tracks matching the query (source) from deezer and apply them into the inner DzrsTrackObject
#[tauri::command]
async fn tracks_fetch_sources(
    path: String,
    tracks: State<'_, Mutex<DzrsTrackObjectWrapper>>,
    tagger: State<'_, DeezerTagger>,
    config: State<'_, Mutex<DzrsConfiguration>>,
) -> Result<(), String> {
    let t = tracks.lock().unwrap().clone();
    let conf = config.lock().unwrap().parsed();

    match t.get_track_obj(&path) {
        Some(tr) => {
            // Fetch sources from deezer using the track metadata
            let mut tr = tr.to_owned();
            // Stripping '&' messes up the url
            let _title = tr.tags.title.to_owned().replace("&", "");
            let _album = tr.tags.album.to_owned().replace("&", "");
            let _artist = tr.tags.artist.to_owned().replace("&", "");
            let tr_meta = (_title.deref(), _album.deref(), _artist.deref());
            let query = match (
                tr_meta.0.is_empty() && tr_meta.1.is_empty() && tr_meta.2.is_empty(),
                conf.tag_fetch_with_filename,
            ) {
                (true, true) => tr.file_name.clone().strip_suffix(".flac").unwrap().into(),
                _ => format!(r#"track:"{}" album:"{}" artist:"{}""#, tr_meta.0, tr_meta.1, tr_meta.2),
            };
            let sources = tagger.fetch_sources(&query).await;
            // Update the DzrsTrackObject with the fetched sources
            match sources {
                Ok(sources) => {
                    tr.tags_sources = sources;
                    tracks.lock().unwrap().replace_track_obj(tr).unwrap();
                }
                Err(err) => {
                    return Err(err);
                }
            };
        }
        _ => (),
    };

    Ok(())
}

// Fetch possibile tracks matching the manual query (source) from deezer and apply them into the inner DzrsTrackObject
#[tauri::command]
async fn tracks_fetch_sources_manual(
    path: String,
    query: String,
    tracks: State<'_, Mutex<DzrsTrackObjectWrapper>>,
    tagger: State<'_, DeezerTagger>,
) -> Result<(), String> {
    let t = tracks.lock().unwrap().clone();
    let query = query.replace("&", "");

    match t.get_track_obj(&path) {
        Some(tr) => {
            // Fetch sources from deezer using the track metadata
            let mut tr = tr.to_owned();
            let sources = tagger.fetch_sources(&query).await;
            // Update the DzrsTrackObject with the fetched sources
            match sources {
                Ok(sources) => {
                    tr.tags_sources = sources;
                    tracks.lock().unwrap().replace_track_obj(tr).unwrap();
                }
                Err(err) => {
                    return Err(err);
                }
            };
        }
        _ => (),
    };

    Ok(())
}

// Fetch tags for a specific track_id and apply them into the inner DzrsTrackObjects for the given path
// This command is used when applying a different deezer source over the file
#[tauri::command]
async fn tracks_source(
    path: String,
    id: u64,
    tracks: State<'_, Mutex<DzrsTrackObjectWrapper>>,
    tagger: State<'_, DeezerTagger>,
    config: State<'_, Mutex<DzrsConfiguration>>,
) -> Result<(), String> {
    let mut t = tracks.lock().unwrap().clone();
    let conf = config.lock().unwrap().parsed();

    // Call deezer, get the track and update its tags_deezer and tags_to_save with the newly received values
    match t.get_track_obj_mut(&path) {
        Some(tr) => {
            let mut tr = tr.to_owned();
            let payload = tagger.fetch_by_id(id).await;
            tr.tags_deezer.apply_deezer(payload.clone(), &conf);
            tr.tags_to_save.apply_deezer(payload, &conf);
            tr.tags_status = DzrsTrackObjectTagState::Matched;
            tracks.lock().unwrap().replace_track_obj(tr)?;
        }
        _ => (),
    }

    Ok(())
}

// Reload tags from file while keeping all other DzrsTrackObject properties unchanged
#[tauri::command]
async fn tracks_reload(
    path: String,
    tracks: State<'_, Mutex<DzrsTrackObjectWrapper>>,
    config: State<'_, Mutex<DzrsConfiguration>>,
) -> Result<(), String> {
    let mut t = tracks.lock().unwrap();
    let conf = config.lock().unwrap().parsed();

    match t.get_track_obj_mut(&path) {
        Some(tr) => {
            tr.load_tags(&conf)?;
            tr.tags_status = DzrsTrackObjectTagState::Finalized;
        }
        _ => (),
    }

    Ok(())
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
    let mut errors: Vec<String> = Vec::new();
    // Save tags into the flac
    files::save_tags(&path, &tags, &conf)?;
    // Update the loaded track to match the saved file
    let mut track_ = DzrsTrackObject::new(&path)?;
    track_.load_tags(&conf)?;
    if conf.directory_move_on_save && !conf.directory_output.is_empty() {
        let tr_file_name = Path::new(&track_.file_path).file_name().unwrap_or_default();
        let mut new_path = PathBuf::from(conf.directory_output);
        new_path.push(tr_file_name);
        if let Err(e) = std::fs::rename(&track_.file_path, new_path) {
            errors.push(e.to_string())
        };
    }
    let mut t = tracks.lock().unwrap();
    let tr = match t.get_track_obj_mut(&path) {
        Some(tr) => tr,
        None => return Err(format!("Cannot find inner DzrsTrackObject for {}", path)),
    };
    tr.tags = track_.tags;
    tr.tags_pictures = track_.tags_pictures;
    tr.tags_to_save = track_.tags_to_save;
    tr.tags_status = DzrsTrackObjectTagState::Finalized;
    if !errors.is_empty() {
        Err(errors.join("\n"))
    } else {
        Ok(())
    }
}

// Deletes files from given paths
#[tauri::command]
async fn delete_files(paths: Vec<String>, tracks: State<'_, Mutex<DzrsTrackObjectWrapper>>) -> Result<(), String> {
    let mut t = tracks.lock().unwrap();
    let mut errors: Vec<String> = Vec::new();
    for p in paths {
        if let Err(e) = t.remove_track(&p) {
            errors.push(e)
        };
        if let Err(e) = std::fs::remove_file(p) {
            errors.push(e.to_string())
        };
    }
    if !errors.is_empty() {
        Err(errors.join("\n"))
    } else {
        Ok(())
    }
}

// Reinitializes a new file watcher using a given directory
// To avoid unnecessary triggers when manipulating the inner DzrsTrackObjects from the frontend
// the event kind of Modify will be ignored, the downside is that if the user manually renames
// a file or modifies it in any way other than using dzrs, the changes will not be
// immediately reflected in the frontend
#[tauri::command]
async fn watch_dir(
    dir: String,
    window: Window,
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

    if let Err(err) = w.watch(Path::new(&dir), RecursiveMode::NonRecursive) {
        return Err(format!("Error watching {dir}, {err}"));
    };

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
    if !app_dir.exists() {
        let _ = std::fs::create_dir_all(app_dir);
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
            tracks_fetch_sources,
            tracks_fetch_sources_manual,
            tracks_source,
            tracks_reload,
            save_tags,
            delete_files,
            watch_dir,
            browse_cmd,
        ])
        .run(tauri::generate_context!())
        .expect("error while running dzrs");
}
