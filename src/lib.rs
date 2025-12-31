pub mod initialize_apple_music_api_jwt {
    use std::{
        env,
        time::{SystemTime, UNIX_EPOCH},
    };

    use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone)]
    struct AppleMusicConfig {
        private_key: String,
        key_id: String,
        team_key: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct AppleMusicApiClaims {
        iss: String, // team key
        iat: u64,    // Issued at
        exp: u64,    // Expiration
    }

    pub fn get_apple_music_bearer_token() -> String {
        generate_apple_music_jwt(&initalize_apple_music_config())
    }

    fn generate_apple_music_jwt(config: &AppleMusicConfig) -> String {
        // set jwt header
        let mut header = Header::new(Algorithm::ES256);
        header.kid = Some(config.key_id.clone());

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // set payload
        let claims = AppleMusicApiClaims {
            iss: config.team_key.clone(),
            iat: now,
            exp: now + 20 * 60, // 20 minutes expiration
        };

        let encoding_key = EncodingKey::from_ec_pem(config.private_key.as_bytes()).unwrap();

        encode(&header, &claims, &encoding_key).unwrap()
    }

    fn initalize_apple_music_config() -> AppleMusicConfig {
        let apple_private_key = env::var("APPLE_MUSIC_PRIVATE_KEY").unwrap();
        let apple_key_id = env::var("APPLE_MUSIC_KEY_ID").unwrap();
        let apple_team_key = env::var("APPLE_MUSIC_TEAM_KEY").unwrap();

        AppleMusicConfig {
            private_key: apple_private_key.to_string(),
            key_id: apple_key_id.to_string(),
            team_key: apple_team_key.to_string(),
        }
    }
}
