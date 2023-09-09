use deezerapi_rs::Deezer;
use lofty::ogg::OggPictureStorage;
use lofty::{flac::FlacFile, AudioFile, ParseOptions};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct DzrsTracks {
    pub items: Vec<DzrsTrack>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct DzrsTrack {
    pub path: String,
    pub tags: DzrsTrackTags,
    pub matched: bool,
    pub tag_candidates: DzrsTrackTagCandidates,
    pub success: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct DzrsTrackTags {}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct DzrsTrackTagCandidates {}

impl Deref for DzrsTracks {
    type Target = Vec<DzrsTrack>;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl DerefMut for DzrsTracks {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
    }
}

impl From<String> for DzrsTrack {
    fn from(value: String) -> Self {
        // Read file from given path and its currently stored tags, and try to get tags from deezer, setting the matched to true if a high accuracy match is found
        let file = match File::open(value) {
            Ok(f) => f,
            Err(_) => return Self::default(),
        };
        let mut reader = BufReader::new(file);
        let flac = match FlacFile::read_from(&mut reader, ParseOptions::default()) {
            Ok(f) => f,
            Err(_) => return Self::default(),
        };
        let flac_tags = flac.vorbis_comments().unwrap().clone();
        println!("TAGS {:?}", flac_tags);
        println!("{:?}", flac.pictures());
        // Self {
        //     path: value,
        //     tags:
        // }
        Self::default()
    }
}

impl From<Vec<String>> for DzrsTracks {
    fn from(value: Vec<String>) -> Self {
        // Filter the files, only flacs are allowed to be parsed
        let flacs_only: Vec<String> = value
            .into_iter()
            .filter(|path| {
                let p = PathBuf::from(path);
                if p.extension().unwrap_or_default() == "flac" {
                    true
                } else {
                    false
                }
            })
            .collect();
        // Load flacs and their tags in memory
        let mut items = Vec::new();
        for path in flacs_only {
            let track = DzrsTrack::from(path);
            items.push(track);
        }
        Self { items }
    }
}
