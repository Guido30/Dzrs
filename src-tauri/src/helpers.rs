use std::env;
use std::env::consts::OS;
use std::path::PathBuf;
use std::process::Command;

pub fn app_data_dir() -> PathBuf {
    let mut path = PathBuf::new();
    let os = OS;
    match os {
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

// pub fn build_file_dialog() -> FileDialogBuilder {
//     let dialog_path: PathBuf;
//     match tauri::api::path::audio_dir() {
//         Some(audio_path) => dialog_path = audio_path,
//         None => dialog_path = tauri::api::path::home_dir().unwrap(),
//     }

//     let file_dialog = FileDialogBuilder::new()
//         .add_filter("", &["mp3", "flac"])
//         .set_directory(dialog_path);

//     file_dialog
// }

pub fn open_explorer(path: String) -> Result<(), String> {
    let path: PathBuf = path.into();
    let program: String = match OS {
        "linux" => "xdg-open".into(),
        "macos" => "open".into(),
        "windows" => "explorer".into(),
        _ => "".into(),
    };
    match Command::new(program).arg(path).spawn() {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}
