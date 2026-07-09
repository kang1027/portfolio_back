use portfolio_back::{
    apple_music_user_token::write_user_token,
    initialize_apple_music_api_jwt::get_apple_music_bearer_token,
};
use rocket::{State, get, http::Status, post, response::status::Custom, serde::json::Json};

use crate::{
    models::model::{AppState, Contact, UserToken},
    services::contact::ContactService,
};

#[get("/admin/get-developer-token")]
pub fn get_developer_token() -> Json<String> {
    Json(get_apple_music_bearer_token())
}

#[post("/admin/save-token", data = "<token>")]
pub fn save_token(
    app_state: &State<AppState>,
    token: Json<UserToken>,
) -> Result<Json<String>, Custom<String>> {
    write_user_token(&token.user_token).map_err(|error| {
        Custom(
            Status::InternalServerError,
            format!("Failed to persist Apple Music user token: {error}"),
        )
    })?;

    {
        let mut guard = app_state.user_token.write().unwrap();
        *guard = token.user_token.trim().to_string();
    }

    Ok(Json("Ok".to_string()))
}

#[post("/contact", data = "<data>")]
pub async fn contact(data: Json<Contact>) -> Json<String> {
    ContactService::contact(data.into_inner()).await.unwrap();
    Json("Ok".to_string())
}
