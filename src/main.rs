use std::sync::{Arc, RwLock};

use dotenv::dotenv;
use rocket::{launch, routes};
use rocket_cors::{AllowedOrigins, CorsOptions};
use tokio::sync::broadcast;

use crate::{
    apis::api::{contact, get_developer_token, save_token},
    models::model::{AppState, NowPlaying},
    scheduler::Scheduler,
    websocket::ws_handler::now_playing_ws,
};

mod apis;
mod models;
mod scheduler;
mod services;
mod websocket;

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    let (broadcast_tx, _) = broadcast::channel::<NowPlaying>(100);

    let app_state = AppState {
        user_token: Arc::new(RwLock::new(String::default())),
        cached_now_playing: Arc::new(RwLock::new(NowPlaying {
            is_playing: false,
            track: None,
            timestamp: 0,
            last_poll_time: 0,
        })),
        broadcast_tx,
    };
    // start apple music fetch scheduler
    tokio::spawn(Scheduler::fetch_apple_music_scheduler(app_state.clone()));

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::some_exact(&[
            "http://localhost:5173",
            "http://kang1027.com",
            "https://kang1027.com",
            "http://192.0.0.2:5173",
        ]))
        .allow_credentials(true)
        .max_age(Some(3600))
        .to_cors()
        .expect("Failed to create CORS");

    rocket::build()
        .attach(cors)
        .manage(app_state)
        .mount("/api", routes![get_developer_token, save_token, contact])
        .mount("/", routes![now_playing_ws])
}
