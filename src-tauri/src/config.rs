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
    pub directory_view_path: String,
    pub filter_download_genre: String,
    pub filter_download_date: String,
    pub filter_download_composer: String,
    pub filter_download_isrc: String,
    pub filter_download_copyright: String,
    pub filter_download_bitdepth: String,
    pub filter_download_samplingrate: String,
    pub filter_dirview_extension: String,
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
            "directory_view_path" => Ok(self.directory_view_path.clone()),
            "filter_download_genre" => Ok(self.filter_download_genre.clone()),
            "filter_download_date" => Ok(self.filter_download_date.clone()),
            "filter_download_composer" => Ok(self.filter_download_composer.clone()),
            "filter_download_isrc" => Ok(self.filter_download_isrc.clone()),
            "filter_download_copyright" => Ok(self.filter_download_copyright.clone()),
            "filter_download_bitdepth" => Ok(self.filter_download_bitdepth.clone()),
            "filter_download_samplingrate" => Ok(self.filter_download_samplingrate.clone()),
            "filter_dirview_extension" => Ok(self.filter_dirview_extension.clone()),
            _ => Err(Error("Field not found in config!".into())),
        }
    }

    pub fn update(&mut self, field: String, value: String) {
        match field.as_str() {
            "download_path" => self.download_path = value,
            "file_template" => self.file_template = value,
            "overwrite_downloads" => self.overwrite_downloads = value,
            "directory_view_path" => self.directory_view_path = value,
            "filter_download_genre" => self.filter_download_genre = value,
            "filter_download_date" => self.filter_download_date = value,
            "filter_download_composer" => self.filter_download_composer = value,
            "filter_download_isrc" => self.filter_download_isrc = value,
            "filter_download_copyright" => self.filter_download_copyright = value,
            "filter_download_bitdepth" => self.filter_download_bitdepth = value,
            "filter_download_samplingrate" => self.filter_download_samplingrate = value,
            "filter_dirview_extension" => self.filter_dirview_extension = value,
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
            directory_view_path: "".into(),
            filter_download_genre: "false".into(),
            filter_download_date: "false".into(),
            filter_download_composer: "false".into(),
            filter_download_isrc: "false".into(),
            filter_download_copyright: "false".into(),
            filter_download_bitdepth: "false".into(),
            filter_download_samplingrate: "false".into(),
            filter_dirview_extension: "false".into(),
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
