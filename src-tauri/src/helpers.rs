use std::collections::HashMap;
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

pub fn make_indices_unique(vec: &mut Vec<(usize, String)>) {
    let mut index_map: HashMap<usize, usize> = HashMap::new();

    for i in 0..vec.len() {
        let (current_index, genre) = &mut vec[i];
        if let Some(new_index) = index_map.get_mut(current_index) {
            *new_index += 1;
            *current_index = *new_index;
        } else {
            index_map.insert(*current_index, *current_index);
        }
    }
}
