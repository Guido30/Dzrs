use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::PathBuf;

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DzrsConfiguration {
    #[serde(rename = "_path")]
    _path: PathBuf,
    #[serde(rename = "_loaded")]
    _loaded: bool,
    #[serde(rename = "_created")]
    _created: bool,
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
    pub tag_pad_track: String,
    pub tag_pad_track_total: String,
    pub tag_pad_track_char: String,
    pub tag_pad_disc: String,
    pub tag_pad_disc_total: String,
    pub tag_pad_disc_char: String,
    pub tag_separator: String,
    pub tag_dz_title: String,
    pub tag_dz_artist: String,
    pub tag_dz_album: String,
    pub tag_dz_album_artist: String,
    pub tag_dz_composer: String,
    pub tag_dz_performer: String,
    pub tag_dz_producer: String,
    pub tag_dz_genre: String,
    pub tag_dz_lyrics: String,
    pub tag_dz_itunesadvisory: String,
    pub tag_dz_length: String,
    pub tag_dz_copyright: String,
    pub tag_dz_track_number: String,
    pub tag_dz_track_total: String,
    pub tag_dz_disc_number: String,
    pub tag_dz_disc_total: String,
    pub tag_dz_date: String,
    pub tag_dz_year: String,
    pub tag_dz_label: String,
    pub tag_dz_barcode: String,
    pub tag_dz_isrc: String,
    pub tag_dz_bpm: String,
    pub tag_dz_replaygain_track_gain: String,
    pub tag_dz_source_id: String,
    pub tag_prefer_sync_lyrics: String,
}

impl DzrsConfiguration {
    pub fn from_file(path: PathBuf) -> Self {
        let mut result = DzrsConfiguration::default();
        let mut loaded = false;
        let mut created = false;
        let file = File::open(path.clone());
        if let Ok(file) = file {
            let reader = BufReader::new(file);
            let serialized_file: Result<DzrsConfiguration, _> = serde_json::from_reader(reader);
            match serialized_file {
                Ok(res) => {
                    loaded = true;
                    result = res;
                }
                Err(_) => {
                    let _conf_str = serde_json::to_string_pretty(&Self::default()).unwrap();
                    let _ = std::fs::write(path.clone(), _conf_str);
                    created = true;
                }
            };
        };
        result._path = path.clone();
        result._loaded = loaded;
        result._created = created;
        result
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
            "tag_pad_track" => self.tag_pad_track = value,
            "tag_pad_track_total" => self.tag_pad_track_total = value,
            "tag_pad_track_char" => self.tag_pad_track_char = value,
            "tag_pad_disc" => self.tag_pad_disc = value,
            "tag_pad_disc_total" => self.tag_pad_disc_total = value,
            "tag_pad_disc_char" => self.tag_pad_disc_char = value,
            "tag_separator" => self.tag_separator = value,
            "tag_dz_title" => self.tag_dz_title = value,
            "tag_dz_artist" => self.tag_dz_artist = value,
            "tag_dz_album" => self.tag_dz_album = value,
            "tag_dz_album_artist" => self.tag_dz_album_artist = value,
            "tag_dz_composer" => self.tag_dz_composer = value,
            "tag_dz_performer" => self.tag_dz_performer = value,
            "tag_dz_producer" => self.tag_dz_producer = value,
            "tag_dz_genre" => self.tag_dz_genre = value,
            "tag_dz_lyrics" => self.tag_dz_lyrics = value,
            "tag_dz_itunesadvisory" => self.tag_dz_itunesadvisory = value,
            "tag_dz_length" => self.tag_dz_length = value,
            "tag_dz_copyright" => self.tag_dz_copyright = value,
            "tag_dz_track_number" => self.tag_dz_track_number = value,
            "tag_dz_track_total" => self.tag_dz_track_total = value,
            "tag_dz_disc_number" => self.tag_dz_disc_number = value,
            "tag_dz_disc_total" => self.tag_dz_disc_total = value,
            "tag_dz_date" => self.tag_dz_date = value,
            "tag_dz_year" => self.tag_dz_date = value,
            "tag_dz_label" => self.tag_dz_label = value,
            "tag_dz_barcode" => self.tag_dz_barcode = value,
            "tag_dz_isrc" => self.tag_dz_isrc = value,
            "tag_dz_bpm" => self.tag_dz_bpm = value,
            "tag_dz_replaygain_track_gain" => self.tag_dz_replaygain_track_gain = value,
            "tag_dz_source_id" => self.tag_dz_source_id = value,
            "tag_prefer_sync_lyrics" => self.tag_prefer_sync_lyrics = value,
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
            _loaded: false,
            _created: false,
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
            tag_pad_track: "false".into(),
            tag_pad_track_total: "false".into(),
            tag_pad_track_char: "0".into(),
            tag_pad_disc: "false".into(),
            tag_pad_disc_total: "false".into(),
            tag_pad_disc_char: "0".into(),
            tag_separator: "; ".into(),
            tag_dz_title: "true".into(),
            tag_dz_artist: "true".into(),
            tag_dz_album: "true".into(),
            tag_dz_album_artist: "true".into(),
            tag_dz_composer: "true".into(),
            tag_dz_performer: "true".into(),
            tag_dz_producer: "true".into(),
            tag_dz_genre: "true".into(),
            tag_dz_lyrics: "true".into(),
            tag_dz_itunesadvisory: "true".into(),
            tag_dz_length: "true".into(),
            tag_dz_copyright: "true".into(),
            tag_dz_track_number: "true".into(),
            tag_dz_track_total: "true".into(),
            tag_dz_disc_number: "true".into(),
            tag_dz_disc_total: "true".into(),
            tag_dz_date: "true".into(),
            tag_dz_year: "true".into(),
            tag_dz_label: "true".into(),
            tag_dz_barcode: "true".into(),
            tag_dz_isrc: "true".into(),
            tag_dz_bpm: "true".into(),
            tag_dz_replaygain_track_gain: "true".into(),
            tag_dz_source_id: "true".into(),
            tag_prefer_sync_lyrics: "true".into(),
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
