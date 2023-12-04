use crate::models::slavart_api::Search;

use chrono::prelude::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};
use std::time::{Duration, UNIX_EPOCH};

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct SlavartDownloadItems {
    pub items: Vec<SlavartDownloadItem>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct SlavartDownloadItem {
    pub thumbnail: String,
    pub large: String,
    pub artist: String,
    pub genre: String,
    pub album_title: String,
    pub duration: i64,
    pub title: String,
    pub id: i64,
    pub bit_depth: i64,
    pub sampling_rate: f64,
    pub isrc: String,
    pub composer: String,
    pub copyright: String,
    pub date: String,
}

impl From<Search> for SlavartDownloadItems {
    fn from(value: Search) -> Self {
        let items: Vec<SlavartDownloadItem> = value
            .tracks
            .items
            .into_iter()
            .map(|item| {
                let title = match item.version {
                    Some(ver) => item.title + " (" + ver.as_str() + ")",
                    None => item.title,
                };
                let album_date =
                    DateTime::<Utc>::from(UNIX_EPOCH + Duration::from_secs(item.album.released_at.abs() as u64))
                        .format("%Y-%m-%d")
                        .to_string();
                SlavartDownloadItem {
                    thumbnail: item.album.image.thumbnail.unwrap_or_default(),
                    large: item.album.image.large,
                    artist: item.album.artist.name,
                    genre: item.album.genre.name,
                    album_title: item.album.title,
                    duration: item.duration,
                    title: title,
                    id: item.id,
                    bit_depth: item.maximum_bit_depth,
                    sampling_rate: item.maximum_sampling_rate,
                    isrc: item.isrc,
                    composer: item.composer.unwrap_or_default().name,
                    copyright: item.copyright.unwrap_or_default(),
                    date: album_date,
                }
            })
            .collect();
        Self { items }
    }
}

impl Deref for SlavartDownloadItems {
    type Target = Vec<SlavartDownloadItem>;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl DerefMut for SlavartDownloadItems {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
    }
}
