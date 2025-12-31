use std::{env, error::Error, fs};

use chrono::{FixedOffset, Utc};
use rocket::serde::json::to_pretty_string;
use teloxide::{Bot, prelude::Requester};

use crate::models::model::Contact;

pub struct ContactService;

impl ContactService {
    /// save contact info with txt, and send notification using telegram bot.
    pub async fn contact(data: Contact) -> Result<(), Box<dyn Error>> {
        let now = Utc::now().with_timezone(&FixedOffset::east_opt(9 * 3600).unwrap());
        let dir_path = format!("contacts/{}", now.format("%Y-%m-%d").to_string());
        let file_path = format!(
            "{dir_path}/{}_{}.txt",
            data.name,
            now.timestamp().to_string()
        );
        let data = to_pretty_string(&data).unwrap();

        // create dir
        if let Err(e) = fs::DirBuilder::new().recursive(true).create(dir_path) {
            println!("Error dir create: {}", e.to_string());
        }

        // create file
        if let Err(e) = fs::write(file_path, &data) {
            println!("Error file create: {}", e.to_string());
        }

        // send notification
        Self::send_telegram(&data).await;

        Ok(())
    }

    async fn send_telegram(data: &str) {
        let bot = Bot::from_env();
        let chat_id = env::var("TELEGRAM_CHAT_ID").expect("should set chat_id");
        let message = format!("📬 New Contact Received\n\n{}", data);

        if let Err(e) = bot
            .send_message(
                teloxide::types::ChatId(chat_id.parse::<i64>().unwrap()),
                message,
            )
            .await
        {
            println!("Failed to send telegram notification: {}", e);
        }
    }
}
