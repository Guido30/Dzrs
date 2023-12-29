use crate::config::DzrsConfigurationParsed;
use crate::types::files;
use crate::types::tags::{set_vorbis_tags, DzrsTrackObjectPicture, DzrsTrackObjectTagSource, DzrsTrackObjectTags};

use lofty::ogg::OggPictureStorage;
use lofty::{flac::FlacFile, AudioFile, ParseOptions};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::ops::{Deref, DerefMut};
use std::path::Path;

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct DzrsTrackObjectWrapper {
    pub items: Vec<DzrsTrackObject>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct DzrsTrackObject {
    pub file_path: String,
    pub file_name: String,
    pub file_size: u64,
    pub file_extension: String,
    pub tags_status: DzrsTrackObjectTagState,
    pub tags: DzrsTrackObjectTags,
    pub tags_deezer: DzrsTrackObjectTags,
    pub tags_to_save: DzrsTrackObjectTags,
    pub tags_sources: Vec<DzrsTrackObjectTagSource>,
    pub tags_pictures: Vec<DzrsTrackObjectPicture>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub enum DzrsTrackObjectTagState {
    #[default]
    NotFetched,
    Unsuccessfull,
    Successfull, // When multiple tracks are returned from the query, user has to ensure the matched track (first one in response) actually matches the file
    Matched,
    Finalized,
}

// Reads a flac file
pub fn read_flac<P: AsRef<Path>>(path: P) -> Result<FlacFile, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let flac = FlacFile::read_from(&mut reader, ParseOptions::default())?;
    Ok(flac)
}

// Save given DzrsTrackObjectTags into a flac file, by manipulating the file stored vorbis tags
pub fn save_tags<P: AsRef<Path>>(path: P, tags: &DzrsTrackObjectTags) -> Result<(), String> {
    let path = path.as_ref().to_str().unwrap();
    let mut flac = match read_flac(path) {
        Ok(f) => f,
        Err(err) => return Err(err.to_string()),
    };
    let vorbis = match flac.vorbis_comments_mut() {
        Some(v) => v,
        None => return Err(format!("Vorbis Comments not found for {}", path)),
    };
    set_vorbis_tags(tags, vorbis);
    if let Err(err) = flac.save_to_path(path) {
        return Err(err.to_string());
    };
    Ok(())
}

impl DzrsTrackObjectWrapper {
    // Create a new DzrsTrackObjectWrapper which contains all files within a given directory
    pub fn new<P: AsRef<Path>>(dir: P) -> Result<Self, String> {
        let dir: &Path = dir.as_ref();
        if dir.exists() && dir.is_dir() {
            let mut items: Vec<DzrsTrackObject> = vec![];
            for entry in std::fs::read_dir(dir).unwrap() {
                if let Ok(item) = entry {
                    match DzrsTrackObject::new(&item.path()) {
                        Ok(tr) => items.push(tr),
                        Err(_) => (),
                    };
                }
            }
            Ok(Self { items })
        } else {
            Err(format!("Invalid path {}", dir.to_str().unwrap()))
        }
    }

    // Clears all inner DzrsTrackObjects
    pub fn clear(&mut self) {
        self.items.clear();
    }

    // Creates a new DzrsTrackObject from a given path and adds it
    pub fn add_track<P: AsRef<Path>>(&mut self, path: P) -> Result<(), String> {
        let path = path.as_ref().to_str().unwrap();
        match self.get_track_obj(path) {
            Some(_) => Err("Cannot add duplicate file".to_string()),
            None => {
                let tr = DzrsTrackObject::new(path);
                match tr {
                    Ok(tr) => {
                        self.items.push(tr);
                        Ok(())
                    }
                    Err(err) => Err(err),
                }
            }
        }
    }

    // Replace an existing inner DzrsTrackObject based on its path by reloading the file
    pub fn replace_track<P: AsRef<Path>>(&mut self, path: P) -> Result<(), String> {
        let path = path.as_ref().to_str().unwrap();
        match self.get_track_obj_mut(path) {
            Some(tr1) => {
                let tr2 = DzrsTrackObject::new(path);
                match tr2 {
                    Ok(tr2) => {
                        *tr1 = tr2;
                        Ok(())
                    }
                    Err(err) => Err(err),
                }
            }
            None => Err(format!("Cannot replace for {}", path)),
        }
    }

    // Replace an existing inner DzrsTrackObject based on its path by swapping it with a given one
    pub fn replace_track_obj(&mut self, tr: DzrsTrackObject) -> Result<(), String> {
        match self.iter().position(|i| i.file_path == tr.file_path) {
            Some(i) => {
                self[i] = tr;
                Ok(())
            }
            None => Err(format!("Cannot insert for {}", &tr.file_path)),
        }
    }

    // Replace an existing inner DzrsTrackObject based on its path OR add a new one by reloading the file
    pub fn insert_track<P: AsRef<Path>>(&mut self, path: P) -> Result<(), String> {
        let path = path.as_ref().to_str().unwrap();
        let tr1: DzrsTrackObject = match DzrsTrackObject::new(path) {
            Ok(tr1) => tr1,
            Err(err) => return Err(err),
        };
        match self.get_track_obj_mut(path) {
            Some(tr2) => {
                *tr2 = tr1;
            }
            None => {
                self.items.push(tr1);
            }
        }
        Ok(())
    }

    // Remove an existing inner DzrsTrackObject based on its path
    pub fn remove_track<P: AsRef<Path>>(&mut self, path: P) -> Result<(), String> {
        let path = path.as_ref().to_str().unwrap();
        let i = self.iter().position(|i| i.file_path == path);
        match i {
            Some(i) => {
                self.items.remove(i);
                Ok(())
            }
            None => Err(format!("Cannot remove for {}", path)),
        }
    }

    pub fn get_track_obj(&self, path: &str) -> Option<&DzrsTrackObject> {
        self.iter().find(|&i| &i.file_path == path)
    }

    pub fn get_track_obj_mut(&mut self, path: &str) -> Option<&mut DzrsTrackObject> {
        self.iter_mut().find(|i| &i.file_path == path)
    }
}

impl DzrsTrackObject {
    // Load a file and its information into a DzrsTrackObject from a given path
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let path: &Path = path.as_ref();
        if !path.is_file() {
            return Err(format!("Not a file {}", path.to_str().unwrap()));
        }
        // Initialize an empty DzrsTrackObject
        let mut track_obj = Self::default();
        // Load basic file information
        track_obj.file_path = path.to_str().unwrap_or_default().to_string();
        if let Ok(metadata) = std::fs::metadata(path) {
            track_obj.file_name = path
                .file_name()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default()
                .to_string();
            track_obj.file_extension = path
                .extension()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default()
                .to_string();
            track_obj.file_size = metadata.len();
        }
        Ok(track_obj)
    }

    // Load flac metadata for this DzrsTrackObject using its file_path, replacing in-place
    pub fn load_tags(&mut self, config: &DzrsConfigurationParsed) -> Result<(), String> {
        let flac = match files::read_flac(&self.file_path) {
            Ok(f) => f,
            Err(err) => return Err(err.to_string()),
        };
        let pictures: Vec<DzrsTrackObjectPicture> = flac
            .pictures()
            .into_iter()
            .map(|p| DzrsTrackObjectPicture::new(p))
            .collect();
        let vorbis = flac.vorbis_comments().unwrap();
        let mut tags = DzrsTrackObjectTags::new(vorbis, config);

        // Set the length read from the flac properties, readonly tag!
        let properties = flac.properties();
        tags.length = properties.duration().as_secs().to_string();

        self.tags = tags.clone();
        self.tags_pictures = pictures;
        self.tags_to_save = tags;
        Ok(())
    }
}

impl Deref for DzrsTrackObjectWrapper {
    type Target = Vec<DzrsTrackObject>;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl DerefMut for DzrsTrackObjectWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
    }
}

pub struct DzrsTracksObjectWrapperIterator<'a> {
    inner: std::slice::Iter<'a, DzrsTrackObject>,
}

impl<'a> DzrsTracksObjectWrapperIterator<'a> {
    pub fn new(object: &'a DzrsTrackObjectWrapper) -> Self {
        DzrsTracksObjectWrapperIterator {
            inner: object.items.iter(),
        }
    }
}

impl<'a> Iterator for DzrsTracksObjectWrapperIterator<'a> {
    type Item = &'a DzrsTrackObject;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<'a> IntoIterator for &'a DzrsTrackObjectWrapper {
    type Item = &'a DzrsTrackObject;
    type IntoIter = DzrsTracksObjectWrapperIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        DzrsTracksObjectWrapperIterator::new(self)
    }
}
