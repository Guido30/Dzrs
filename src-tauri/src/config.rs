use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::PathBuf;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DzrsConfiguration {
    #[serde(skip_serializing, skip_deserializing)]
    _path: PathBuf,
    pub download_path: String,
    pub file_template: String,
    pub overwrite_downloads: String,
    pub filter_column_genre: String,
    pub filter_column_date: String,
    pub filter_column_composer: String,
    pub filter_column_isrc: String,
    pub filter_column_copyright: String,
    pub filter_column_bitdepth: String,
    pub filter_column_samplingrate: String,
}

impl DzrsConfiguration {
    pub fn from_file(path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path.clone())?;
        let reader = BufReader::new(file);
        let mut result: DzrsConfiguration = match serde_json::from_reader(reader) {
            Ok(conf) => conf,
            Err(_) => {
                let default_conf = Self::default();
                let _conf_str = serde_json::to_string_pretty(&Self::default())?;
                match std::fs::write(path.clone(), _conf_str) {
                    Ok(_) => default_conf,
                    Err(err) => return Err(Box::new(Error(err.to_string()))),
                }
            }
        };
        result._path = path.clone();
        Ok(result)
    }

    pub fn get(&self, field: String) -> Result<String, Error> {
        match field.as_str() {
            "download_path" => Ok(self.download_path.clone()),
            "file_template" => Ok(self.file_template.clone()),
            "overwrite_downloads" => Ok(self.overwrite_downloads.clone()),
            "filter_column_genre" => Ok(self.filter_column_genre.clone()),
            "filter_column_date" => Ok(self.filter_column_date.clone()),
            "filter_column_composer" => Ok(self.filter_column_composer.clone()),
            "filter_column_isrc" => Ok(self.filter_column_isrc.clone()),
            "filter_column_copyright" => Ok(self.filter_column_copyright.clone()),
            "filter_column_bitdepth" => Ok(self.filter_column_bitdepth.clone()),
            "filter_column_samplingrate" => Ok(self.filter_column_samplingrate.clone()),
            _ => Err(Error("Field not found in config!".into())),
        }
    }

    pub fn update(&mut self, field: String, value: String) {
        match field.as_str() {
            "download_path" => self.download_path = value,
            "file_template" => self.file_template = value,
            "overwrite_downloads" => self.overwrite_downloads = value,
            "filter_column_genre" => self.filter_column_genre = value,
            "filter_column_date" => self.filter_column_date = value,
            "filter_column_composer" => self.filter_column_composer = value,
            "filter_column_isrc" => self.filter_column_isrc = value,
            "filter_column_copyright" => self.filter_column_copyright = value,
            "filter_column_bitdepth" => self.filter_column_bitdepth = value,
            "filter_column_samplingrate" => self.filter_column_samplingrate = value,
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
            filter_column_genre: "false".into(),
            filter_column_date: "false".into(),
            filter_column_composer: "false".into(),
            filter_column_isrc: "false".into(),
            filter_column_copyright: "false".into(),
            filter_column_bitdepth: "false".into(),
            filter_column_samplingrate: "false".into(),
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
