use crate::config::DzrsConfiguration;
use base64::{engine::general_purpose, Engine as _};
use deezerapi_rs::Deezer;
use lofty::ogg::{OggPictureStorage, VorbisComments};
use lofty::{flac::FlacFile, AudioFile, ParseOptions};
use lofty::{Accessor, Picture, PictureInformation};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct DzrsTracks {
    pub items: Vec<DzrsTrack>,
}

pub struct DzrsTracksIterator<'a> {
    inner: std::slice::Iter<'a, DzrsTrack>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct DzrsTrack {
    pub path: String,
    pub tags: DzrsTrackTags,
    pub pictures: Vec<DzrsTrackPicture>,
    pub tags_deezer: DzrsTrackTags,
    pub tag_candidates: DzrsTrackTagCandidates,
    pub matched: bool,
    pub fetched: bool,
    pub candidates: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct DzrsTrackTags {
    title: String,
    artist: String,
    artists: String,
    album: String,
    album_artist: String,
    composer: String,
    performer: String,
    producer: String,
    genre: String,
    lyrics: String,
    copyright: String,
    description: String,
    track_number: String,
    track_total: String,
    disc_number: String,
    disc_total: String,
    date: String,
    year: String,
    original_date: String,
    original_year: String,
    comment: String,
    label: String,
    barcode: String,
    isrc: String,
    bpm: String,
    replaygain_album_gain: String,
    replaygain_album_peak: String,
    replaygain_track_gain: String,
    replaygain_track_peak: String,
    source_id: String,
    encoder: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct DzrsTrackPicture {
    pub b64: String,
    pub pic_type: String,
    pub description: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct DzrsTrackTagCandidates {}

impl DzrsTracks {
    pub fn add(&mut self, track: DzrsTrack) {
        self.items.push(track);
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn get_track(&self, path: String) -> Option<&DzrsTrack> {
        self.iter().find(|&track| track.path == path)
    }

    pub fn get_track_mut(&mut self, path: String) -> Option<&mut DzrsTrack> {
        self.iter_mut().find(|track| track.path == path)
    }
}

pub trait FromWithConfig<T>
where
    Self: Sized + Default,
{
    fn from_with_config(value: T, config: &DzrsConfiguration) -> Self;
    fn from_with_deezer(_value: T, _config: &DzrsConfiguration) -> Self {
        Self::default()
    }
}

impl FromWithConfig<Vec<String>> for DzrsTracks {
    fn from_with_config(paths: Vec<String>, config: &DzrsConfiguration) -> Self {
        // Filter the files, only flacs are allowed to be parsed
        let flacs_only: Vec<String> = paths
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
            let track = DzrsTrack::from_with_config(path, config);
            items.push(track);
        }
        Self { items }
    }
}

impl FromWithConfig<String> for DzrsTrack {
    fn from_with_config(path: String, config: &DzrsConfiguration) -> Self {
        // Read file from given path and its currently stored tags
        let file = match File::open(path.clone()) {
            Ok(f) => f,
            Err(_) => return Self::default(),
        };
        let mut reader = BufReader::new(file);
        let flac = match FlacFile::read_from(&mut reader, ParseOptions::default()) {
            Ok(f) => f,
            Err(_) => return Self::default(),
        };
        let pictures: Vec<DzrsTrackPicture> = flac
            .pictures()
            .into_iter()
            .map(|p| DzrsTrackPicture::from_with_config(p, config))
            .collect();
        let vorbis = flac.vorbis_comments().unwrap().clone();
        let tags = DzrsTrackTags::from_with_config(vorbis, &config);
        Self {
            path: path,
            tags,
            pictures,
            tags_deezer: DzrsTrackTags::default(),
            tag_candidates: DzrsTrackTagCandidates::default(),
            matched: false,
            fetched: false,
            candidates: false,
        }
    }

    fn from_with_deezer(path: String, config: &DzrsConfiguration) -> Self {
        // Read file from given path and its currently stored tags, and try to get tags from deezer, setting the matched to true if a high accuracy match is found
        let dzrs_track = Self::from_with_config(path, config);

        // let deezer = Deezer::new();
        // let api_tracks = deezer.search_track(
        //     &dzrs_track.tags.title,
        //     &dzrs_track.tags.artist,
        //     &dzrs_track.tags.album,
        //     false,
        // );

        // println!("{:?}", api_tracks);
        dzrs_track
    }
}

impl FromWithConfig<VorbisComments> for DzrsTrackTags {
    fn from_with_config(vorbis: VorbisComments, config: &DzrsConfiguration) -> Self {
        let sep = &config.tag_separator;
        Self {
            title: vorbis.title().unwrap_or_default().to_string(),
            artist: vorbis.get_all("ARTIST").collect::<Vec<&str>>().join(sep),
            artists: vorbis.get_all("ARTISTS").collect::<Vec<&str>>().join(sep),
            barcode: vorbis.get("BARCODE").unwrap_or_default().to_string(),
            album: vorbis.album().unwrap_or_default().to_string(),
            album_artist: vorbis.get("ALBUMARTIST").unwrap_or_default().to_string(),
            comment: vorbis.comment().unwrap_or_default().to_string(),
            composer: vorbis.get_all("COMPOSER").collect::<Vec<&str>>().join(sep),
            performer: vorbis.get_all("PERFORMER").collect::<Vec<&str>>().join(sep),
            producer: vorbis.get_all("PRODUCER").collect::<Vec<&str>>().join(sep),
            description: vorbis.get("DESCRIPTION").unwrap_or_default().to_string(),
            genre: vorbis.genre().unwrap_or_default().to_string(),
            lyrics: vorbis.get("LYRICS").unwrap_or_default().to_string(),
            copyright: vorbis.get("COPYRIGHT").unwrap_or_default().to_string(),
            track_number: vorbis.track().unwrap_or_default().to_string(),
            track_total: vorbis.track_total().unwrap_or_default().to_string(),
            disc_number: vorbis.disk().unwrap_or_default().to_string(),
            disc_total: vorbis.disk_total().unwrap_or_default().to_string(),
            date: vorbis.get("DATE").unwrap_or_default().to_string(),
            original_date: vorbis.get("ORIGINALDATE").unwrap_or_default().to_string(),
            original_year: vorbis.get("ORIGINALYEAR").unwrap_or_default().to_string(),
            label: vorbis.get_all("LABEL").collect::<Vec<&str>>().join(sep),
            year: vorbis.year().unwrap_or_default().to_string(),
            isrc: vorbis.get("ISRC").unwrap_or_default().to_string(),
            bpm: vorbis.get("BPM").unwrap_or_default().to_string(),
            replaygain_album_gain: vorbis
                .get("REPLAYGAIN_ALBUM_GAIN")
                .unwrap_or_default()
                .to_string(),
            replaygain_album_peak: vorbis
                .get("REPLAYGAIN_ALBUM_PEAK")
                .unwrap_or_default()
                .to_string(),
            replaygain_track_gain: vorbis
                .get("REPLAYGAIN_TRACK_GAIN")
                .unwrap_or_default()
                .to_string(),
            replaygain_track_peak: vorbis
                .get("REPLAYGAIN_TRACK_PEAK")
                .unwrap_or_default()
                .to_string(),
            source_id: vorbis.get("SOURCEID").unwrap_or_default().to_string(),
            encoder: vorbis.get("ENCODER").unwrap_or_default().to_string(),
        }
    }
}

impl FromWithConfig<&(Picture, PictureInformation)> for DzrsTrackPicture {
    fn from_with_config(value: &(Picture, PictureInformation), config: &DzrsConfiguration) -> Self {
        let data = value.0.data();
        let b64 = general_purpose::STANDARD.encode(data);
        let pic_type = format!("{:?}", value.0.pic_type());
        let description = value.0.description().unwrap_or_default().to_string();
        Self {
            b64,
            pic_type,
            description,
            width: value.1.width,
            height: value.1.height,
        }
    }
}

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

impl<'a> DzrsTracksIterator<'a> {
    pub fn new(tracks: &'a DzrsTracks) -> Self {
        DzrsTracksIterator {
            inner: tracks.items.iter(),
        }
    }
}

impl<'a> Iterator for DzrsTracksIterator<'a> {
    type Item = &'a DzrsTrack;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<'a> IntoIterator for &'a DzrsTracks {
    type Item = &'a DzrsTrack;
    type IntoIter = DzrsTracksIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        DzrsTracksIterator::new(self)
    }
}
