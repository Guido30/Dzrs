use serde::Deserialize;

#[derive(Deserialize, Clone, Debug, Default)]
pub struct SlavartDownloadItem {
    pub thumbnail: String,
    pub performer_name: String,
    pub album_title: String,
    pub duration: u32,
    pub title: String,
    pub id: u64,
}
