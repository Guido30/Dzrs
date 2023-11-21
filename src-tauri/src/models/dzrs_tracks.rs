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
    vorbis.set_track(tags.track_number.parse().unwrap()); // needs handling
    vorbis.set_track_total(tags.track_total.parse().unwrap()); // needs handling
    vorbis.set_disk(tags.disk_number.parse().unwrap()); // needs handling
    vorbis.set_disk_total(tags.disk_total.parse().unwrap()); // needs handling
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
    vorbis.insert(
        "REPLAYGAIN_ALBUM_GAIN".to_string(),
        tags.replaygain_album_gain,
    );
    vorbis.insert(
        "REPLAYGAIN_ALBUM_PEAK".to_string(),
        tags.replaygain_album_peak,
    );
    vorbis.insert(
        "REPLAYGAIN_TRACK_GAIN".to_string(),
        tags.replaygain_track_gain,
    );
    vorbis.insert(
        "REPLAYGAIN_TRACK_PEAK".to_string(),
        tags.replaygain_track_peak,
    );
    vorbis.insert("SOURCEID".to_string(), tags.source_id);
    vorbis.insert("ENCODER".to_string(), tags.encoder);
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
pub struct DzrsTrackTagCandidate {}

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

    pub async fn update(
        &mut self,
        paths: Vec<String>,
        config: &DzrsConfigurationParsed,
        fetch_deezer_tags: bool,
    ) {
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
            return Err(format!(
                "Track currently not loaded for path {}",
                path.clone()
            ));
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
        let track_ = match deezer
            .search_track(
                &dzrs_track.tags.title,
                &dzrs_track.tags.artist,
                &dzrs_track.tags.album,
                false,
            )
            .await
        {
            Ok(t) => {
                dzrs_track.matched = true;
                dzrs_track.fetched = true;
                Some(t)
            }
            Err(_) => {
                dzrs_track.fetched = true;
                None
            }
        };

        let track = match &track_ {
            Some(t) => match deezer.track(t.id).await {
                Ok(s) => Some(s),
                Err(_) => None,
            },
            None => None,
        };

        let gw_track = match &track_ {
            Some(t) => match deezer.gw_song(t.id).await {
                Ok(s) => Some(s),
                Err(_) => None,
            },
            None => None,
        };

        let album = match &track_ {
            Some(t) => match deezer.album(t.album.id).await {
                Ok(a) => Some(a),
                Err(_) => None,
            },
            None => None,
        };

        let gw_album = match &track_ {
            Some(t) => match deezer.gw_album(t.album.id).await {
                Ok(a) => Some(a),
                Err(_) => None,
            },
            None => None,
        };

        let lyrics = match &track_ {
            Some(t) => match deezer.gw_lyrics(t.id).await {
                Ok(l) => Some(l),
                Err(_) => None,
            },
            None => None,
        };

        let mut tags_deezer = dzrs_track.tags.clone();
        tags_deezer.update_with_deezer(track, gw_track, album, gw_album, lyrics, config);
        dzrs_track.tags_deezer = tags_deezer.clone();
        dzrs_track.tags_to_save = tags_deezer;

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
                self.track_number = t.track_position.to_string()
            };
            if config.tag_dz_disk_number {
                self.disk_number = t.disk_number.to_string()
            };
            if config.tag_dz_date {
                self.date = t.release_date.clone();
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
                self.track_total = a.number_track;
            };
            if config.tag_dz_disk_total {
                self.disk_total = a.number_disk;
            };
            if config.tag_dz_date {
                if let Some(d) = a.original_release_date {
                    self.original_date = d;
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
        let pictures: Vec<DzrsTrackPicture> = flac
            .pictures()
            .into_iter()
            .map(|p| DzrsTrackPicture::from_with_config(p, config))
            .collect();
        let vorbis = flac.vorbis_comments().unwrap().clone();
        let tags = TrackTags::from_with_config(vorbis, &config);
        Self {
            path: path,
            tags: tags.clone(),
            pictures,
            tags_deezer: TrackTags::default(),
            tag_candidates: vec![DzrsTrackTagCandidate::default()],
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
        Self {
            title: vorbis.title().unwrap_or_default().to_string(),
            artist: vorbis.get_all("ARTIST").collect::<Vec<&str>>().join(sep),
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
            disk_number: vorbis.disk().unwrap_or_default().to_string(),
            disk_total: vorbis.disk_total().unwrap_or_default().to_string(),
            date: vorbis.get("DATE").unwrap_or_default().to_string(),
            original_date: vorbis.get("ORIGINALDATE").unwrap_or_default().to_string(),
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
    fn from_with_config(
        value: &(Picture, PictureInformation),
        config: &DzrsConfigurationParsed,
    ) -> Self {
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
