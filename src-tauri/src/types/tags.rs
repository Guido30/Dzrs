use crate::config::DzrsConfigurationParsed;

use base64::{engine::general_purpose, Engine as _};
use deezerapi_rs::models::{api as deezer_api, gw as deezer_gw};
use deezerapi_rs::Deezer;
use lofty::ogg::VorbisComments;
use lofty::Accessor;
use lofty::{Picture, PictureInformation};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn set_vorbis_tags(tags: &DzrsTrackObjectTags, vorbis: &mut VorbisComments) {
    let tags = tags.clone();
    vorbis.set_title(tags.title);
    vorbis.set_artist(tags.artist);
    vorbis.set_album(tags.album);
    vorbis.set_genre(tags.genre);
    vorbis.set_comment(tags.comment);
    vorbis.set_track(tags.track_number.parse().unwrap_or_default());
    vorbis.set_track_total(tags.track_total.parse().unwrap_or_default());
    vorbis.set_disk(tags.disk_number.parse().unwrap_or_default());
    vorbis.set_disk_total(tags.disk_total.parse().unwrap_or_default());
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

pub fn unique_indices(vec: &mut Vec<(usize, String)>) {
    let mut index_map: HashMap<usize, usize> = HashMap::new();

    for i in 0..vec.len() {
        let (current_index, _) = &mut vec[i];
        if let Some(new_index) = index_map.get_mut(current_index) {
            *new_index += 1;
            *current_index = *new_index;
        } else {
            index_map.insert(*current_index, *current_index);
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct DeezerTagger {
    client: Deezer,
}

#[derive(Deserialize, Clone, Debug, Default)]
pub struct DeezerStructuredPayload {
    pub track: Option<deezer_api::MainTrack>,
    pub gw_track: Option<deezer_gw::Song>,
    pub album: Option<deezer_api::MainAlbum>,
    pub gw_album: Option<deezer_gw::Album>,
    pub lyrics: Option<deezer_gw::Lyrics>,
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct DzrsTrackObjectTags {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub album_artist: String,
    pub composer: String,
    pub performer: String,
    pub producer: String,
    pub genre: String,
    pub lyrics: String,
    pub copyright: String,
    pub description: String,
    pub track_number: String,
    pub track_total: String,
    pub disk_number: String,
    pub disk_total: String,
    pub date: String,
    pub year: String,
    pub original_date: String,
    pub comment: String,
    pub label: String,
    pub barcode: String,
    pub isrc: String,
    pub bpm: String,
    pub replaygain_album_gain: String,
    pub replaygain_album_peak: String,
    pub replaygain_track_gain: String,
    pub replaygain_track_peak: String,
    pub source_id: String,
    pub encoder: String,
    pub extra_tags: Vec<(String, String)>,
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct DzrsTrackObjectPicture {
    pub b64: String,
    pub pic_type: String,
    pub description: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct DzrsTrackObjectTagSource {
    id: u64,
    title: String,
    link: String,
    duration: u64,
    artist: String,
    album: String,
    cover: String,
}

impl DeezerTagger {
    pub fn new() -> Self {
        let client = Deezer::new();
        Self { client }
    }

    // Call deezer and get a structured payload back based on the given track metadata, errors
    // on request fail or no tracks are found
    pub async fn fetch_by_query(
        &self,
        title_q: &str,
        album_q: &str,
        artist_q: &str,
    ) -> Result<(DeezerStructuredPayload, Vec<DzrsTrackObjectTagSource>), String> {
        // Build the query from the provided metadata and send the request
        let query = format!(r#"track:"{}" album:"{}" artist:"{}""#, title_q, album_q, artist_q);
        let res = match self.client.search(&query, true).await {
            Ok(p) => p,
            Err(err) => return Err(format!("{:?}", err)),
        };

        // Build the possible sources payload
        let sources = res.iter().map(|tr| DzrsTrackObjectTagSource::new(tr)).collect();

        // Build the main track payload if present
        let t_id = res.first().map(|i| i.id);
        let payload = match t_id {
            Some(t_id) => {
                let mut album_id: Option<u64> = None;
                let track: Option<deezer_api::MainTrack> = match self.client.track(t_id).await {
                    Ok(t) => {
                        album_id = Some(t.album.id);
                        Some(t)
                    }
                    Err(_) => None,
                };
                let gw_track: Option<deezer_gw::Song> = match self.client.gw_song(t_id).await {
                    Ok(s) => Some(s),
                    Err(_) => None,
                };
                let lyrics: Option<deezer_gw::Lyrics> = match self.client.gw_lyrics(t_id).await {
                    Ok(l) => Some(l),
                    Err(_) => None,
                };
                let mut album: Option<deezer_api::MainAlbum> = None;
                let mut gw_album: Option<deezer_gw::Album> = None;
                match album_id {
                    Some(album_id) => {
                        if let Ok(a) = self.client.album(album_id).await {
                            album = Some(a);
                        };
                        if let Ok(a) = self.client.gw_album(album_id).await {
                            gw_album = Some(a);
                        };
                    }
                    None => {
                        album = None;
                        gw_album = None;
                    }
                };
                DeezerStructuredPayload {
                    track,
                    gw_track,
                    album,
                    gw_album,
                    lyrics,
                }
            }
            None => return Err(format!("No tracks found on deezer for query {}", query)),
        };

        Ok((payload, sources))
    }

    pub async fn fetch_by_id(&self, track_id: u64) -> DeezerStructuredPayload {
        let mut album_id: Option<u64> = None;
        let track: Option<deezer_api::MainTrack> = match self.client.track(track_id).await {
            Ok(t) => {
                album_id = Some(t.album.id);
                Some(t)
            }
            Err(_) => None,
        };
        let gw_track: Option<deezer_gw::Song> = match self.client.gw_song(track_id).await {
            Ok(s) => Some(s),
            Err(_) => None,
        };
        let lyrics: Option<deezer_gw::Lyrics> = match self.client.gw_lyrics(track_id).await {
            Ok(l) => Some(l),
            Err(_) => None,
        };
        let mut album: Option<deezer_api::MainAlbum> = None;
        let mut gw_album: Option<deezer_gw::Album> = None;
        match album_id {
            Some(album_id) => {
                if let Ok(a) = self.client.album(album_id).await {
                    album = Some(a);
                };
                if let Ok(a) = self.client.gw_album(album_id).await {
                    gw_album = Some(a);
                };
            }
            None => {
                album = None;
                gw_album = None;
            }
        };
        DeezerStructuredPayload {
            track,
            gw_track,
            album,
            gw_album,
            lyrics,
        }
    }
}

impl DzrsTrackObjectTags {
    pub fn new(vorbis: &VorbisComments, config: &DzrsConfigurationParsed) -> Self {
        let sep = &config.tag_separator;
        let mut t = Self::default();
        let mut artists: Vec<String> = Vec::new();
        let mut composers: Vec<String> = Vec::new();
        let mut performers: Vec<String> = Vec::new();
        let mut producers: Vec<String> = Vec::new();
        let mut labels: Vec<String> = Vec::new();
        let mut extra_tags: Vec<(String, String)> = Vec::new();
        let v = vorbis.items();
        for tag in v {
            match tag.0 {
                "TITLE" => t.title = tag.1.to_string(),
                "ARTIST" => artists.push(tag.1.to_string()),
                "BARCODE" => t.barcode = tag.1.to_string(),
                "ALBUM" => t.album = tag.1.to_string(),
                "ALBUMARTIST" => t.album_artist = tag.1.to_string(),
                "COMMENT" => t.comment = tag.1.to_string(),
                "COMPOSER" => composers.push(tag.1.to_string()),
                "PERFORMER" => performers.push(tag.1.to_string()),
                "PRODUCER" => producers.push(tag.1.to_string()),
                "DESCRIPTION" => t.description = tag.1.to_string(),
                "GENRE" => t.genre = tag.1.to_string(),
                "LYRICS" => t.lyrics = tag.1.to_string(),
                "COPYRIGHT" => t.copyright = tag.1.to_string(),
                "TRACKNUMBER" => t.track_number = tag.1.to_string(),
                "TRACKTOTAL" => t.track_total = tag.1.to_string(),
                "TOTALTRACKS" => t.track_total = tag.1.to_string(),
                "DISCNUMBER" => t.disk_number = tag.1.to_string(),
                "DISCTOTAL" => t.disk_total = tag.1.to_string(),
                "TOTALDISCS" => t.disk_total = tag.1.to_string(),
                "DATE" => t.date = tag.1.to_string(),
                "ORIGINALDATE" => t.original_date = tag.1.to_string(),
                "LABEL" => labels.push(tag.1.to_string()),
                "YEAR" => t.year = tag.1.to_string(),
                "ISRC" => t.isrc = tag.1.to_string(),
                "BPM" => t.bpm = tag.1.to_string(),
                "REPLAYGAIN_ALBUM_GAIN" => t.replaygain_album_gain = tag.1.to_string(),
                "REPLAYGAIN_ALBUM_PEAK" => t.replaygain_album_peak = tag.1.to_string(),
                "REPLAYGAIN_TRACK_GAIN" => t.replaygain_track_gain = tag.1.to_string(),
                "REPLAYGAIN_TRACK_PEAK" => t.replaygain_track_peak = tag.1.to_string(),
                "SOURCEID" => t.source_id = tag.1.to_string(),
                "ENCODER" => t.encoder = tag.1.to_string(),
                _ => extra_tags.push((tag.0.to_string(), tag.1.to_string())),
            };
        }
        t.artist = artists.join(sep);
        t.composer = composers.join(sep);
        t.performer = performers.join(sep);
        t.producer = producers.join(sep);
        t.label = labels.join(sep);
        t.extra_tags = extra_tags;
        t
    }

    // Applies values retrieved from deezer, only updating fields based on config
    pub fn apply_deezer(&mut self, payload: DeezerStructuredPayload, conf: &DzrsConfigurationParsed) {
        let mut artists: Vec<String> = vec![];

        if let Some(t) = payload.track {
            if conf.tag_dz_title {
                self.title = t.title;
            };
            if conf.tag_dz_album {
                self.album = t.album.title;
            };
            if conf.tag_dz_album_artist {
                self.album_artist = t.artist.name.clone();
            };
            artists.push(t.artist.name);
            if conf.tag_dz_track_number {
                if conf.tag_pad_track {
                    self.track_number = format!("{:0>2}", t.track_position);
                } else {
                    self.track_number = t.track_position.to_string()
                };
            };
            if conf.tag_dz_disk_number {
                if conf.tag_pad_disk {
                    self.disk_number = format!("{:0>2}", t.disk_number);
                } else {
                    self.disk_number = t.disk_number.to_string()
                };
            };
            if conf.tag_dz_date {
                if conf.tag_date_as_year {
                    self.date = t.release_date[..=3].to_string();
                } else {
                    self.date = t.release_date.clone();
                };
            };
            if conf.tag_dz_year {
                self.year = t.release_date[..=3].to_string();
            }
            if conf.tag_dz_isrc {
                self.isrc = t.isrc;
            };
            if conf.tag_dz_bpm {
                if t.bpm != 0.0 {
                    self.bpm = t.bpm.to_string();
                };
            };
            if conf.tag_dz_replaygain_track_gain {
                if t.gain != 0.0 {
                    self.replaygain_track_gain = t.gain.to_string() + " dB";
                };
            };
            if conf.tag_dz_source_id {
                self.source_id = t.id.to_string();
            };
        };

        if let Some(t) = payload.gw_track {
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
            if conf.tag_dz_composer {
                if let Some(composers) = contributors.get("composer") {
                    self.composer = composers.join(&conf.tag_separator)
                };
            };
            if conf.tag_dz_performer {
                if let Some(performer) = contributors.get("performer") {
                    self.performer = performer.join(&conf.tag_separator)
                };
            };
            if conf.tag_dz_producer {
                if let Some(producer) = contributors.get("producer") {
                    self.producer = producer.join(&conf.tag_separator)
                };
            };
        };

        if let Some(a) = payload.album {
            if conf.tag_dz_genre {
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

                unique_indices(&mut split_genres);

                for (i, g) in split_genres {
                    if !genres.contains(&g) {
                        genres.insert(i, g);
                    };
                }

                self.genre = genres.join(&conf.tag_separator);
            };
            if conf.tag_dz_label {
                self.label = a.label;
            }
            if conf.tag_dz_barcode {
                self.barcode = a.upc;
            }
        };

        if let Some(a) = payload.gw_album {
            if conf.tag_dz_copyright {
                self.copyright = a.copyright;
            };
            if conf.tag_dz_track_total {
                if conf.tag_pad_track_total {
                    self.track_total = format!("{:0>2}", a.number_track);
                } else {
                    self.track_total = a.number_track;
                };
            };
            if conf.tag_dz_disk_total {
                if conf.tag_pad_disk_total {
                    self.disk_total = format!("{:0>2}", a.number_disk);
                } else {
                    self.disk_total = a.number_disk;
                };
            };
            if conf.tag_dz_original_date {
                if let Some(d) = a.original_release_date {
                    if conf.tag_originaldate_as_year {
                        self.original_date = d[..=3].to_string();
                    } else {
                        self.original_date = d;
                    };
                };
            };
        };

        if let Some(l) = payload.lyrics {
            if conf.tag_dz_lyrics {
                if conf.tag_prefer_sync_lyrics && l.lyrics_sync_json.is_some() {
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

        if conf.tag_dz_artist {
            self.artist = artists.join(conf.tag_separator.as_str());
        };
    }
}

impl DzrsTrackObjectPicture {
    pub fn new(value: &(Picture, PictureInformation)) -> Self {
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

impl DzrsTrackObjectTagSource {
    pub fn new(value: &deezer_api::Track) -> Self {
        let value = value.to_owned();
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
