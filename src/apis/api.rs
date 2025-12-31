use portfolio_back::initialize_apple_music_api_jwt::get_apple_music_bearer_token;
use rocket::{State, get, post, serde::json::Json};

use crate::{
    models::model::{AppState, Contact, UserToken},
    services::contact::ContactService,
};

#[get("/admin/get-developer-token")]
pub fn get_developer_token() -> Json<String> {
    Json(get_apple_music_bearer_token())
}

#[post("/admin/save-token", data = "<token>")]
pub fn save_token(app_state: &State<AppState>, token: Json<UserToken>) -> Json<String> {
    {
        let mut guard = app_state.user_token.write().unwrap();
        *guard = token.user_token.clone();
    }

    Json("Ok".to_string())
}

#[post("/contact", data = "<data>")]
pub async fn contact(data: Json<Contact>) -> Json<String> {
    ContactService::contact(data.into_inner()).await.unwrap();
    Json("Ok".to_string())
}
