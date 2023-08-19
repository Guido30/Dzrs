use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Search {
    pub query: String,
    pub albums: Albums,
    pub tracks: Tracks,
    pub artists: ArtistsRoot,
    pub playlists: Playlists,
    pub focus: Focus,
    pub articles: Articles,
    pub stories: Stories,
    pub most_popular: MostPopular,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Albums {
    pub limit: i64,
    pub offset: i64,
    pub analytics: Analytics,
    pub total: i64,
    pub items: Vec<AlbumItem>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Analytics {
    pub search_external_id: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct AlbumItem {
    pub maximum_bit_depth: i64,
    pub image: Image,
    pub media_count: i64,
    pub artist: Artist,
    pub artists: Vec<Artists>,
    pub upc: String,
    pub released_at: i64,
    pub label: Label,
    pub title: String,
    pub qobuz_id: i64,
    pub version: Option<String>,
    pub url: String,
    pub duration: i64,
    pub parental_warning: bool,
    pub popularity: i64,
    pub tracks_count: i64,
    pub genre: Genre,
    pub maximum_channel_count: i64,
    pub id: String,
    pub maximum_sampling_rate: f64,
    pub articles: Vec<Value>,
    pub release_date_original: String,
    pub release_date_download: String,
    pub release_date_stream: String,
    pub purchasable: bool,
    pub streamable: bool,
    pub previewable: bool,
    pub sampleable: bool,
    pub downloadable: bool,
    pub displayable: bool,
    pub purchasable_at: i64,
    pub streamable_at: i64,
    pub hires: bool,
    pub hires_streamable: bool,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    pub small: String,
    pub thumbnail: Option<String>,
    pub large: String,
    pub back: Option<Value>,
    pub medium: Option<String>,
    pub extralarge: Option<String>,
    pub mega: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Artist {
    pub image: Value,
    pub name: String,
    pub id: i64,
    pub albums_count: i64,
    pub slug: String,
    pub picture: Value,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Artists {
    pub id: i64,
    pub name: String,
    pub roles: Vec<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Label {
    pub name: String,
    pub id: i64,
    pub albums_count: i64,
    pub supplier_id: i64,
    pub slug: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Genre {
    pub path: Vec<i64>,
    pub color: String,
    pub name: String,
    pub id: i64,
    pub slug: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Tracks {
    pub limit: i64,
    pub offset: i64,
    pub analytics: Analytics,
    pub total: i64,
    pub items: Vec<TrackItem>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct TrackItem {
    pub maximum_bit_depth: i64,
    pub copyright: String,
    pub performers: String,
    pub audio_info: AudioInfo,
    pub performer: Performer,
    pub album: Album,
    pub work: Value,
    pub composer: Option<Composer>,
    pub isrc: String,
    pub title: String,
    pub version: Option<String>,
    pub duration: i64,
    pub parental_warning: bool,
    pub track_number: i64,
    pub maximum_channel_count: i64,
    pub id: i64,
    pub media_number: i64,
    pub maximum_sampling_rate: f64,
    pub articles: Option<Vec<TrackArticle>>,
    pub release_date_original: Value,
    pub release_date_download: Value,
    pub release_date_stream: Value,
    pub release_date_purchase: Value,
    pub purchasable: bool,
    pub streamable: bool,
    pub previewable: bool,
    pub sampleable: bool,
    pub downloadable: bool,
    pub displayable: bool,
    pub purchasable_at: Option<i64>,
    pub streamable_at: i64,
    pub hires: bool,
    pub hires_streamable: bool,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct AudioInfo {
    pub replaygain_track_peak: f64,
    pub replaygain_track_gain: f64,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Performer {
    pub name: String,
    pub id: i64,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Album {
    pub image: Image,
    pub maximum_bit_depth: i64,
    pub media_count: i64,
    pub artist: Artist,
    pub upc: String,
    pub released_at: i64,
    pub label: Label,
    pub title: String,
    pub qobuz_id: i64,
    pub version: Option<String>,
    pub duration: i64,
    pub parental_warning: bool,
    pub tracks_count: i64,
    pub popularity: i64,
    pub genre: Genre,
    pub maximum_channel_count: i64,
    pub id: String,
    pub maximum_sampling_rate: f64,
    pub previewable: bool,
    pub sampleable: bool,
    pub displayable: bool,
    pub streamable: bool,
    pub streamable_at: i64,
    pub downloadable: bool,
    pub purchasable_at: Value,
    pub purchasable: bool,
    pub release_date_original: String,
    pub release_date_download: String,
    pub release_date_stream: String,
    pub release_date_purchase: String,
    pub hires: bool,
    pub hires_streamable: bool,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Composer {
    pub name: String,
    pub id: i64,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ArtistsRoot {
    pub limit: i64,
    pub offset: i64,
    pub analytics: Analytics,
    pub total: i64,
    pub items: Vec<Value>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Playlists {
    pub limit: i64,
    pub offset: i64,
    pub analytics: Analytics,
    pub total: i64,
    pub items: Vec<Value>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Focus {
    pub limit: i64,
    pub offset: i64,
    pub analytics: Analytics,
    pub total: i64,
    pub items: Vec<Value>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Articles {
    pub limit: i64,
    pub offset: i64,
    pub analytics: Analytics,
    pub total: i64,
    pub items: Vec<Article>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    pub image: String,
    pub thumbnail: String,
    pub root_category: i64,
    pub author: String,
    #[serde(rename = "abstract")]
    pub abstract_field: String,
    pub source: String,
    pub title: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub url: String,
    pub image_original: String,
    pub category_id: i64,
    pub source_image: String,
    pub id: i64,
    pub published_at: i64,
    pub category: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Stories {
    pub limit: i64,
    pub offset: i64,
    pub analytics: Analytics,
    pub total: i64,
    pub items: Vec<Value>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct MostPopular {
    pub limit: i64,
    pub offset: i64,
    pub analytics: Analytics,
    pub total: i64,
    pub items: Vec<MostPopularItem>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct MostPopularItem {
    #[serde(rename = "type")]
    pub type_field: String,
    pub content: Value,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct TrackArticle {
    pub id: i64,
    pub url: String,
    pub price: f64,
    pub currency: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub label: String,
    pub description: String,
}
