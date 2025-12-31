use std::sync::{Arc, RwLock};

use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;

// static
#[derive(Clone)]
pub struct AppState {
    pub user_token: Arc<RwLock<String>>,
    pub cached_now_playing: Arc<RwLock<NowPlaying>>,
    pub broadcast_tx: broadcast::Sender<NowPlaying>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserToken {
    pub user_token: String,
}

/*
 *
  "isPlaying": true,
  "track": {
    "title": "Sunflower",
    "artist": "Post Malone, Swae Lee",
    "album": "Spider-Man: Into the Spider-Verse",
    "artwork": "https://is1-ssl.mzstatic.com/image/thumb/Music125/v4/...",
    "duration": 158,
    "currentTime": 45
  },
  "timestamp": 1703472123000
*/
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NowPlaying {
    pub is_playing: bool,
    pub track: Option<Track>,
    pub timestamp: u64,
    #[serde(skip_serializing)]
    pub last_poll_time: u64,
}

// follow https://developer.apple.com/documentation/applemusicapi/get-recently-played-resources
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Track {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub artwork: String,
    pub duration: u64,
    pub current_time: u64,
    pub genre_names: Vec<String>,
    pub track_number: u64,
    pub release_date: String,
    pub isrc: String,
    pub url: String,
    pub has_lyrics: bool,
    pub preview_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contact {
    pub name: String,
    pub email: String,
    pub message: String,
}
