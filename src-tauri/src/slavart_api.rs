use std::ops::{Deref, DerefMut};

use crate::models::slavart::Search;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct SlavartDownloadItems {
    pub items: Vec<SlavartDownloadItem>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct SlavartDownloadItem {
    pub thumbnail: String,
    pub large: String,
    pub performer_name: String,
    pub album_title: String,
    pub duration: i64,
    pub title: String,
    pub id: i64,
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

impl From<Search> for SlavartDownloadItems {
    fn from(value: Search) -> Self {
        let items: Vec<SlavartDownloadItem> = value
            .tracks
            .items
            .into_iter()
            .map(|item| SlavartDownloadItem {
                thumbnail: item.album.image.thumbnail.unwrap_or_default(),
                large: item.album.image.large,
                performer_name: item.performer.name,
                album_title: item.album.title,
                duration: item.duration,
                title: item.title,
                id: item.id,
            })
            .collect();
        Self { items }
    }
}
