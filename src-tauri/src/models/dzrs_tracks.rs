use crate::config::DzrsConfigurationParsed;
use crate::helpers::make_indices_unique;
use base64::{engine::general_purpose, Engine as _};
use deezerapi_rs::models::{api as deezer_api, gw as deezer_gw};
use deezerapi_rs::Deezer;
use lofty::ogg::{OggPictureStorage, VorbisComments};
use lofty::{flac::FlacFile, AudioFile, ParseOptions};
use lofty::{Accessor, Picture, PictureInformation};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;

fn read_flac(path: String) -> Result<FlacFile, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let flac = FlacFile::read_from(&mut reader, ParseOptions::default())?;
    Ok(flac)
}

fn update_vorbis_with_tags<'a>(vorbis: &'a mut VorbisComments, tags: TrackTags) {
    vorbis.set_title(tags.title);
    vorbis.set_artist(tags.artist);
    vorbis.set_album(tags.album);
    vorbis.set_genre(tags.genre);
    vorbis.set_comment(tags.comment);
    vorbis.set_track(tags.track_number.parse().unwrap_or_default()); // needs handling
    vorbis.set_track_total(tags.track_total.parse().unwrap_or_default()); // needs handling
    vorbis.set_disk(tags.disk_number.parse().unwrap_or_default());
    vorbis.set_disk_total(tags.disk_total.parse().unwrap_or_default()); // needs handling
    vorbis.insert("ALBUMARTIST".to_string(), tags.album_artist);
    vorbis.insert("COMPOSER".to_string(), tags.composer);
    vorbis.insert("PERFORMER".to_string(), tags.performer);
    vorbis.insert("PRODUCER".to_string(), tags.producer);
    vorbis.insert("LYRICS".to_string(), tags.lyrics);
    vorbis.insert("COPYRIGHT".to_string(), tags.copyright);
    vorbis.insert("DESCRIPTION".to_string(), tags.description);
    vorbis.insert("DATE".to_string(), tags.date);
    vorbis.insert("YEAR".to_string(), tags.year);
    vorbis.insert("ORIGINALDATE".to_string(), tags.original_date);
    vorbis.insert("LABEL".to_string(), tags.label);
    vorbis.insert("BARCODE".to_string(), tags.barcode);
    vorbis.insert("ISRC".to_string(), tags.isrc);
    vorbis.insert("BPM".to_string(), tags.bpm);
    vorbis.insert("REPLAYGAIN_ALBUM_GAIN".to_string(), tags.replaygain_album_gain);
    vorbis.insert("REPLAYGAIN_ALBUM_PEAK".to_string(), tags.replaygain_album_peak);
    vorbis.insert("REPLAYGAIN_TRACK_GAIN".to_string(), tags.replaygain_track_gain);
    vorbis.insert("REPLAYGAIN_TRACK_PEAK".to_string(), tags.replaygain_track_peak);
    vorbis.insert("SOURCEID".to_string(), tags.source_id);
    vorbis.insert("ENCODER".to_string(), tags.encoder);
    for item in tags.extra_tags {
        if let Some(tag_value) = vorbis.get(&item.0) {
            if tag_value != &item.1 {
                vorbis.insert(item.0, item.1);
            };
        };
    }
}

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
    pub tags: TrackTags,
    pub pictures: Vec<DzrsTrackPicture>,
    pub tags_deezer: TrackTags,
    pub tag_candidates: Vec<DzrsTrackTagCandidate>,
    pub tags_to_save: TrackTags,
    pub matched: bool,
    pub fetched: bool,
    pub candidates: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct TrackTags {
    title: String,
    artist: String,
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
    disk_number: String,
    disk_total: String,
    date: String,
    year: String,
    original_date: String,
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
    extra_tags: Vec<(String, String)>,
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
pub struct DzrsTrackTagCandidate {
    id: u64,
    title: String,
    link: String,
    duration: u64,
    artist: String,
    album: String,
    cover: String,
}

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

    pub async fn update(&mut self, paths: Vec<String>, config: &DzrsConfigurationParsed, fetch_deezer_tags: bool) {
        for path in paths {
            match self.get_track(path.clone()) {
                Some(_) => {
                    let updated_track = if fetch_deezer_tags {
                        DzrsTrack::from_with_deezer(path.clone(), &config).await
                    } else {
                        DzrsTrack::from_with_config(path.clone(), &config)
                    };
                    *self.get_track_mut(path.clone()).unwrap() = updated_track;
                }
                None => {
                    let new_track = if fetch_deezer_tags {
                        DzrsTrack::from_with_deezer(path, &config).await
                    } else {
                        DzrsTrack::from_with_config(path, &config)
                    };
                    self.add(new_track);
                }
            }
        }
    }

    pub fn save_tags(self, path: String, tags: TrackTags) -> Result<(), String> {
        if let None = self.get_track(path.clone()) {
            return Err(format!("Track currently not loaded for path {}", path.clone()));
        }
        let mut flac = match read_flac(path.clone()) {
            Ok(f) => f,
            Err(err) => return Err(err.to_string()),
        };
        let vorbis = match flac.vorbis_comments_mut() {
            Some(v) => v,
            None => return Err(format!("No Vorbis Comments Found for {}", path.clone())),
        };
        update_vorbis_with_tags(vorbis, tags);
        if let Err(err) = flac.save_to_path(path) {
            return Err(err.to_string());
        };
        Ok(())
    }
}

impl DzrsTrack {
    pub async fn from_with_deezer(path: String, config: &DzrsConfigurationParsed) -> Self {
        let mut dzrs_track = Self::from_with_config(path, config);

        let deezer = Deezer::new();
        let query = format!(
            r#"track:"{}" album:"{}" artist:"{}""#,
            dzrs_track.tags.title, dzrs_track.tags.album, dzrs_track.tags.artist
        );
        let track_id = match deezer.search(&query, true).await {
            Ok(t) => {
                dzrs_track.fetched = true;
                match t.len() {
                    0 => None,
                    _ => {
                        dzrs_track.matched = true;
                        dzrs_track.candidates = true;
                        dzrs_track.tag_candidates = t.iter().map(|tr| DzrsTrackTagCandidate::from(tr)).collect();
                        Some((t[0].id, t[0].album.id))
                    }
                }
            }
            Err(_) => {
                dzrs_track.fetched = true;
                None
            }
        };

        if let Some(ids) = track_id {
            let track = match deezer.track(ids.0).await {
                Ok(s) => Some(s),
                Err(_) => None,
            };

            let gw_track = match deezer.gw_song(ids.0).await {
                Ok(s) => Some(s),
                Err(_) => None,
            };

            let album = match deezer.album(ids.1).await {
                Ok(a) => Some(a),
                Err(_) => None,
            };

            let gw_album = match deezer.gw_album(ids.1).await {
                Ok(a) => Some(a),
                Err(_) => None,
            };

            let lyrics = match deezer.gw_lyrics(ids.0).await {
                Ok(l) => Some(l),
                Err(_) => None,
            };

            let mut tags_deezer = dzrs_track.tags.clone();
            tags_deezer.update_with_deezer(track, gw_track, album, gw_album, lyrics, config);
            dzrs_track.tags_deezer = tags_deezer.clone();
            dzrs_track.tags_to_save = tags_deezer;
        };

        dzrs_track
    }
}

impl TrackTags {
    fn update_with_deezer(
        &mut self,
        track: Option<deezer_api::MainTrack>,
        gw_track: Option<deezer_gw::Song>,
        album: Option<deezer_api::MainAlbum>,
        gw_album: Option<deezer_gw::Album>,
        lyrics: Option<deezer_gw::Lyrics>,
        config: &DzrsConfigurationParsed,
    ) {
        let mut artists: Vec<String> = vec![];

        if let Some(t) = track {
            if config.tag_dz_title {
                self.title = t.title;
            };
            if config.tag_dz_album {
                self.album = t.album.title;
            };
            if config.tag_dz_album_artist {
                self.album_artist = t.artist.name.clone();
            };
            artists.push(t.artist.name);
            if config.tag_dz_track_number {
                if config.tag_pad_track {
                    self.track_number = format!("{:0>2}", t.track_position);
                } else {
                    self.track_number = t.track_position.to_string()
                };
            };
            if config.tag_dz_disk_number {
                if config.tag_pad_disk {
                    self.disk_number = format!("{:0>2}", t.disk_number);
                } else {
                    self.disk_number = t.disk_number.to_string()
                };
            };
            if config.tag_dz_date {
                if config.tag_date_as_year {
                    self.date = t.release_date[..=3].to_string();
                } else {
                    self.date = t.release_date.clone();
                };
            };
            if config.tag_dz_year {
                self.year = t.release_date[..=3].to_string();
            }
            if config.tag_dz_isrc {
                self.isrc = t.isrc;
            };
            if config.tag_dz_bpm {
                if t.bpm != 0.0 {
                    self.bpm = t.bpm.to_string();
                };
            };
            if config.tag_dz_replaygain_track_gain {
                if t.gain != 0.0 {
                    self.replaygain_track_gain = t.gain.to_string() + " dB";
                };
            };
            if config.tag_dz_source_id {
                self.source_id = t.id.to_string();
            };
        };

        if let Some(t) = gw_track {
            let mut _artists = t.artists.clone();
            _artists.sort_by(|a, b| {
                a.artists_songs_order
                    .parse::<u16>()
                    .unwrap_or_default()
                    .cmp(&b.artists_songs_order.parse::<u16>().unwrap_or_default())
            });
            _artists.iter().for_each(|a| {
                if !artists.contains(&a.art_name) {
                    artists.push(a.art_name.clone())
                };
            });
            let contributors = match t.sng_contributors {
                deezer_gw::SngContributors::SngContributors(contributors) => contributors,
                deezer_gw::SngContributors::Empty(_) => HashMap::new(),
            };
            if config.tag_dz_composer {
                if let Some(composers) = contributors.get("composer") {
                    self.composer = composers.join(&config.tag_separator)
                };
            };
            if config.tag_dz_performer {
                if let Some(performer) = contributors.get("performer") {
                    self.performer = performer.join(&config.tag_separator)
                };
            };
            if config.tag_dz_producer {
                if let Some(producer) = contributors.get("producer") {
                    self.producer = producer.join(&config.tag_separator)
                };
            };
        };

        if let Some(a) = album {
            if config.tag_dz_genre {
                let mut split_genres: Vec<(usize, String)> = Vec::new();
                let mut genres: Vec<String> = a
                    .genres
                    .data
                    .clone()
                    .into_iter()
                    .filter(|g| !g.name.contains("/"))
                    .map(|g| g.name)
                    .collect();

                a.genres
                    .data
                    .into_iter()
                    .enumerate()
                    .filter(|(_, g)| g.name.contains("/"))
                    .for_each(|(i, g)| {
                        let genre_names: Vec<&str> = g.name.split('/').collect();
                        for genre_name in genre_names {
                            split_genres.push((i, genre_name.trim().to_string()));
                        }
                    });

                make_indices_unique(&mut split_genres);

                for (i, g) in split_genres {
                    if !genres.contains(&g) {
                        genres.insert(i, g);
                    };
                }

                self.genre = genres.join(&config.tag_separator);
            };
            if config.tag_dz_label {
                self.label = a.label;
            }
            if config.tag_dz_barcode {
                self.barcode = a.upc;
            }
        };

        if let Some(a) = gw_album {
            if config.tag_dz_copyright {
                self.copyright = a.copyright;
            };
            if config.tag_dz_track_total {
                if config.tag_pad_track_total {
                    self.track_total = format!("{:0>2}", a.number_track);
                } else {
                    self.track_total = a.number_track;
                };
            };
            if config.tag_dz_disk_total {
                if config.tag_pad_disk_total {
                    self.disk_total = format!("{:0>2}", a.number_disk);
                } else {
                    self.disk_total = a.number_disk;
                };
            };
            if config.tag_dz_original_date {
                if let Some(d) = a.original_release_date {
                    if config.tag_originaldate_as_year {
                        self.original_date = d[..=3].to_string();
                    } else {
                        self.original_date = d;
                    };
                };
            };
        };

        if let Some(l) = lyrics {
            if config.tag_dz_lyrics {
                if config.tag_prefer_sync_lyrics && l.lyrics_sync_json.is_some() {
                    let mut lines = Vec::new();
                    for l_line in l.lyrics_sync_json.unwrap().into_iter() {
                        let mut line = String::new();
                        if let Some(mut t) = l_line.lrc_timestamp {
                            t.push(' ');
                            line.push_str(t.as_str());
                        };
                        line.push_str(l_line.line.as_str());
                        lines.push(line);
                    }
                    self.lyrics = lines.join("\r\n");
                } else {
                    self.lyrics = l.lyrics_text
                };
            };
        };

        if config.tag_dz_artist {
            self.artist = artists.join(config.tag_separator.as_str());
        };
    }
}

pub trait FromWithConfig<T>
where
    Self: Sized,
{
    fn from_with_config(value: T, config: &DzrsConfigurationParsed) -> Self;
}

impl FromWithConfig<Vec<String>> for DzrsTracks {
    fn from_with_config(paths: Vec<String>, config: &DzrsConfigurationParsed) -> Self {
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
    fn from_with_config(path: String, config: &DzrsConfigurationParsed) -> Self {
        let flac = match read_flac(path.clone()) {
            Ok(f) => f,
            Err(_) => return Self::default(),
        };
        let pictures: Vec<DzrsTrackPicture> = flac.pictures().into_iter().map(|p| DzrsTrackPicture::from(p)).collect();
        let vorbis = flac.vorbis_comments().unwrap().clone();
        let tags = TrackTags::from_with_config(vorbis, &config);
        Self {
            path: path,
            tags: tags.clone(),
            pictures,
            tags_deezer: TrackTags::default(),
            tag_candidates: Vec::new(),
            tags_to_save: tags,
            matched: false,
            fetched: false,
            candidates: false,
        }
    }
}

impl FromWithConfig<VorbisComments> for TrackTags {
    fn from_with_config(vorbis: VorbisComments, config: &DzrsConfigurationParsed) -> Self {
        let sep = &config.tag_separator;
        let mut track_tags = Self::default();
        let mut artists: Vec<String> = Vec::new();
        let mut composers: Vec<String> = Vec::new();
        let mut performers: Vec<String> = Vec::new();
        let mut producers: Vec<String> = Vec::new();
        let mut labels: Vec<String> = Vec::new();
        let mut extra_tags: Vec<(String, String)> = Vec::new();
        let vorbis_tags = vorbis.items();
        for item in vorbis_tags {
            match item.0 {
                "TITLE" => track_tags.title = item.1.to_string(),
                "ARTIST" => artists.push(item.1.to_string()),
                "BARCODE" => track_tags.barcode = item.1.to_string(),
                "ALBUM" => track_tags.album = item.1.to_string(),
                "ALBUMARTIST" => track_tags.album_artist = item.1.to_string(),
                "COMMENT" => track_tags.comment = item.1.to_string(),
                "COMPOSER" => composers.push(item.1.to_string()),
                "PERFORMER" => performers.push(item.1.to_string()),
                "PRODUCER" => producers.push(item.1.to_string()),
                "DESCRIPTION" => track_tags.description = item.1.to_string(),
                "GENRE" => track_tags.genre = item.1.to_string(),
                "LYRICS" => track_tags.lyrics = item.1.to_string(),
                "COPYRIGHT" => track_tags.copyright = item.1.to_string(),
                "TRACKNUMBER" => track_tags.track_number = item.1.to_string(),
                "TRACKTOTAL" => track_tags.track_total = item.1.to_string(),
                "TOTALTRACKS" => track_tags.track_total = item.1.to_string(),
                "DISCNUMBER" => track_tags.disk_number = item.1.to_string(),
                "DISCTOTAL" => track_tags.disk_total = item.1.to_string(),
                "TOTALDISCS" => track_tags.disk_total = item.1.to_string(),
                "DATE" => track_tags.date = item.1.to_string(),
                "ORIGINALDATE" => track_tags.original_date = item.1.to_string(),
                "LABEL" => labels.push(item.1.to_string()),
                "YEAR" => track_tags.year = item.1.to_string(),
                "ISRC" => track_tags.isrc = item.1.to_string(),
                "BPM" => track_tags.bpm = item.1.to_string(),
                "REPLAYGAIN_ALBUM_GAIN" => track_tags.replaygain_album_gain = item.1.to_string(),
                "REPLAYGAIN_ALBUM_PEAK" => track_tags.replaygain_album_peak = item.1.to_string(),
                "REPLAYGAIN_TRACK_GAIN" => track_tags.replaygain_track_gain = item.1.to_string(),
                "REPLAYGAIN_TRACK_PEAK" => track_tags.replaygain_track_peak = item.1.to_string(),
                "SOURCEID" => track_tags.source_id = item.1.to_string(),
                "ENCODER" => track_tags.encoder = item.1.to_string(),
                _ => extra_tags.push((item.0.to_string(), item.1.to_string())),
            };
        }
        track_tags.artist = artists.join(sep);
        track_tags.composer = composers.join(sep);
        track_tags.performer = performers.join(sep);
        track_tags.producer = producers.join(sep);
        track_tags.extra_tags = extra_tags;
        track_tags
    }
}

impl From<&(Picture, PictureInformation)> for DzrsTrackPicture {
    fn from(value: &(Picture, PictureInformation)) -> Self {
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

impl From<&deezer_api::Track> for DzrsTrackTagCandidate {
    fn from(value: &deezer_api::Track) -> Self {
        let value = value.clone();
        Self {
            id: value.id,
            title: value.title,
            link: value.link.unwrap_or_default(),
            duration: value.duration,
            artist: value.artist.name,
            album: value.album.title,
            cover: value.album.cover,
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
