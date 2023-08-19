use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct SlavartDownloadItem {
    pub thumbnail: String,
    pub performer_name: String,
    pub album_title: String,
    pub duration: u32,
    pub title: String,
    pub id: u64,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub performer: Performer,
    pub album: Album,
    pub title: String,
    pub duration: u32,
    pub id: u64,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Performer {
    pub name: String,
    pub id: u64,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Album {
    pub title: String,
    pub image: AlbumImage,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct AlbumImage {
    pub thumbnail: String,
}
