use std::{error::Error, fs, time::Duration};

use portfolio_back::initialize_apple_music_api_jwt::get_apple_music_bearer_token;
use reqwest::Client;
use serde_json::Value;
use tokio::time::{Instant, interval_at};

use crate::models::model::{AppState, NowPlaying, Track};
pub struct Scheduler;

impl Scheduler {
    pub async fn fetch_apple_music_scheduler(app_state: AppState) {
        println!("Start fetch apple music scheduler..");
        let mut interval = interval_at(Instant::now(), Duration::from_secs(10));

        loop {
            interval.tick().await;

            let res = match Self::fetch_apple_music(&app_state).await {
                Ok(res) => {
                    println!("track: {}", res.track.clone().unwrap().title);

                    res
                }
                Err(e) => {
                    eprintln!("Failed fetch apple music scheduler: {e}");
                    NowPlaying {
                        is_playing: false,
                        track: None,
                        timestamp: 0,
                        last_poll_time: 0,
                    }
                }
            };

            {
                let mut guard = app_state.cached_now_playing.write().unwrap();
                let track_changed = if let Some(old_track) = &guard.track {
                    if let Some(new_track) = &res.track {
                        old_track.id != new_track.id
                    } else {
                        true
                    }
                } else {
                    res.track.is_some()
                };

                *guard = res.clone();

                // Broadcast only when track changes
                if track_changed {
                    let _ = app_state.broadcast_tx.send(res);
                }
            }
        }
    }

    async fn fetch_apple_music(app_state: &AppState) -> Result<NowPlaying, Box<dyn Error>> {
        let developer_token = get_apple_music_bearer_token();
        let mut user_token = { app_state.user_token.read().unwrap().clone() };
        if user_token.is_empty() {
            user_token = str::from_utf8(&fs::read("user_token.txt").unwrap())
                .unwrap()
                .to_string();
        }

        let client = Client::new();
        let res = client
            .get("https://api.music.apple.com/v1/me/recent/played/tracks")
            .query(&[("limit", "1"), ("l", "en-US")])
            .bearer_auth(developer_token)
            .header("Music-User-Token", user_token)
            .send()
            .await?;

        if !res.status().is_success() {
            // user_token을 user_token.txt에서 보관중임. 안될 시 admin으로 로그인해 apple
            // music로그인하면 캐싱 및 파일저장 다시 됨.
            return Err(format!("API 오류: {}", res.status()).into());
        }

        let json: Value = res.json().await?;

        // 데이터 파싱
        let data = &json["data"];

        // 재생 중인 곡 없음 = NowPlaying false
        if data.as_array().map_or(true, |arr| arr.is_empty()) {
            let cached_track = {
                let mut guard = app_state.cached_now_playing.write().unwrap();
                *guard = NowPlaying {
                    is_playing: false,
                    track: None,
                    timestamp: Self::current_timestamp(),
                    last_poll_time: Self::current_timestamp(),
                };
                drop(guard);
                app_state.cached_now_playing.read().unwrap().clone()
            };

            return Ok(cached_track);
        }

        let item = &data[0];
        let attributes = &item["attributes"];
        let new_track_id = item["id"].as_str().unwrap_or("Unknown");

        // 캐시된 곡과 비교
        let (current_time, _is_same_track) = {
            let cached = app_state.cached_now_playing.read().unwrap();
            let now = Self::current_timestamp();

            if let Some(cached_track) = &cached.track {
                if cached_track.id == new_track_id && cached.is_playing {
                    // 같은 곡이 계속 재생 중 - 경과 시간 계산
                    let elapsed_seconds = (now - cached.last_poll_time) / 1000;
                    let new_current_time = cached_track.current_time + elapsed_seconds;
                    // new_current_time이 곡 길이를 넘어섰을 때??
                    // -> 그냥 current time을 곡ㄱ 끝 길이로 설정.
                    (new_current_time, true)
                } else {
                    // 다른 곡으로 변경됨
                    (0, false)
                }
            } else {
                // 처음 재생 시작
                (0, false)
            }
        };

        let track = Track {
            id: new_track_id.to_string(),
            title: attributes["name"].as_str().unwrap_or("Unknown").to_string(),
            artist: attributes["artistName"]
                .as_str()
                .unwrap_or("Unknown")
                .to_string(),
            album: attributes["albumName"]
                .as_str()
                .unwrap_or("Unknown")
                .to_string(),
            artwork: Self::get_artwork_url(&attributes["artwork"]),
            duration: attributes["durationInMillis"].as_u64().unwrap_or(0) / 1000,
            current_time,
            genre_names: attributes["genreNames"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect()
                })
                .unwrap_or_default(),
            track_number: attributes["trackNumber"].as_u64().unwrap_or(0),
            release_date: attributes["releaseDate"]
                .as_str()
                .unwrap_or("Unknown")
                .to_string(),
            isrc: attributes["isrc"].as_str().unwrap_or("Unknown").to_string(),
            url: attributes["url"].as_str().unwrap_or("Unknown").to_string(),
            has_lyrics: attributes["hasLyrics"].as_bool().unwrap_or(false),
            preview_url: Self::get_preview_url(&attributes["previews"]),
        };

        Ok(NowPlaying {
            is_playing: true,
            track: Some(track),
            timestamp: Self::current_timestamp(),
            last_poll_time: Self::current_timestamp(),
        })
    }

    fn get_artwork_url(artwork: &Value) -> String {
        let url_template = artwork["url"].as_str().unwrap_or("");
        url_template.replace("{w}", "400").replace("{h}", "400")
    }

    fn get_preview_url(previews: &Value) -> Option<String> {
        previews
            .as_array()
            .and_then(|arr| arr.first())
            .and_then(|preview| preview["url"].as_str())
            .map(|s| s.to_string())
    }

    fn current_timestamp() -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
    }
}
