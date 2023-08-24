use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::{Path, PathBuf};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DzrsConfiguration {
    #[serde(skip_serializing, skip_deserializing)]
    _path: PathBuf,
    pub download_path: String,
    pub file_template: String,
    pub overwrite_downloads: String,
}

impl DzrsConfiguration {
    pub fn from_file(
        path: PathBuf,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path.clone())?;
        let reader = BufReader::new(file);
        let mut result: DzrsConfiguration =
            match serde_json::from_reader(reader) {
                Ok(conf) => conf,
                Err(_) => {
                    let default_conf = Self::default();
                    let _conf_str =
                        serde_json::to_string_pretty(&Self::default())?;
                    match std::fs::write(path.clone(), _conf_str) {
                        Ok(_) => default_conf,
                        Err(err) => {
                            return Err(Box::new(Error(err.to_string())))
                        }
                    }
                }
            };
        result._path = path.clone();
        Ok(result)
    }

    pub fn get(&self, field: String) -> Result<String, Error> {
        match field.as_str() {
            "download_path" => Ok(self.download_path.clone()),
            _ => Err(Error("Field not found in config!".into())),
        }
    }

    pub fn update(&mut self, field: String, value: String) {
        match field.as_str() {
            "download_path" => self.download_path = value,
            "file_template" => self.file_template = value,
            "overwrite_downloads" => self.overwrite_downloads = value,
            _ => (),
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let json_string = serde_json::to_string_pretty(&self)?;
        let mut file = File::create(&self._path)?;
        file.write_all(json_string.as_bytes())?;
        Ok(())
    }
}

impl Default for DzrsConfiguration {
    fn default() -> Self {
        Self {
            _path: PathBuf::new(),
            download_path: "".into(),
            file_template: "%title% - %album%".into(),
            overwrite_downloads: "true".into(),
        }
    }
}

#[derive(Debug)]
pub struct Error(String);

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for Error {}
