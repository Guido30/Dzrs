use serde::{Deserialize, Serialize};
use std::fs::DirEntry;
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct DzrsFiles {
    pub items: Vec<DzrsFile>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct DzrsFile {
    path: String,
    filename: String,
    size: u64,
    extension: String,
}

impl Deref for DzrsFiles {
    type Target = Vec<DzrsFile>;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl DerefMut for DzrsFiles {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
    }
}

impl From<String> for DzrsFiles {
    fn from(path: String) -> Self {
        let path: PathBuf = PathBuf::from(path);
        if path.is_dir() && path.exists() {
            let mut files: Vec<DzrsFile> = vec![];
            for entry in std::fs::read_dir(path).unwrap() {
                if let Ok(item) = entry {
                    let file = DzrsFile::from(item);
                    files.push(file);
                }
            }
            Self { items: files }
        } else {
            Self::default()
        }
    }
}

impl From<String> for DzrsFile {
    fn from(path: String) -> Self {
        let path: PathBuf = PathBuf::from(path);
        match std::fs::metadata(path.clone()) {
            Ok(metadata) => {
                if metadata.is_file() {
                    let _path = path.to_str().unwrap_or_default().to_string();
                    let filename = path
                        .file_name()
                        .unwrap_or_default()
                        .to_str()
                        .unwrap_or_default()
                        .to_string();
                    let extension = path
                        .extension()
                        .unwrap_or_default()
                        .to_str()
                        .unwrap_or_default()
                        .to_string();
                    Self {
                        path: _path,
                        filename: filename,
                        size: metadata.len(),
                        extension: extension,
                    }
                } else {
                    return Self::default();
                }
            }
            Err(_) => {
                return Self::default();
            }
        }
    }
}

impl From<DirEntry> for DzrsFile {
    fn from(value: DirEntry) -> Self {
        let metadata = value.metadata();
        let path = value.path().to_str().unwrap_or_default().to_string();
        let filename = value.file_name().to_str().unwrap_or_default().to_string();
        let size: u64 = match metadata {
            Ok(metadata) => metadata.len(),
            Err(_) => u64::default(),
        };
        let extension = value
            .path()
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
            .to_string();
        Self {
            path: path,
            filename: filename,
            size: size,
            extension: extension,
        }
    }
}
