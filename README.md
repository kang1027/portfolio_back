# Portfolio Backend

A Rust-based backend server for a portfolio website. It streams currently playing music information from Apple Music in real-time and processes visitor contact information.

> **Frontend Repository**: [https://github.com/kang1027/portfolio_front/](https://github.com/kang1027/portfolio_front/)

## Key Features

### 1. Live Music Streaming

- Polls currently playing music information from Apple Music API every 10 seconds
- Real-time streaming to clients via WebSocket
- Broadcasts on initial connection/track changes for efficient data transmission
- Automatic playback time tracking and calculation

### 2. Contact Form Processing

- Saves visitor contact information to files
- Real-time notifications via Telegram Bot

## Tech Stack

- **Framework**: Rocket (Rust Web Framework)
- **WebSocket**: rocket_ws
- **HTTP Client**: reqwest
- **Telegram Bot**: teloxide
- **Serialization**: serde, serde_json
- **Auth**: jsonwebtoken
- **CORS**: rocket_cors

## Environment Variables Setup

Create a `.env` file in the project root and set the following environment variables.

### Apple Music API Setup

1. Log in to [Apple Developer Account](https://developer.apple.com/account)
2. Create a Media ID in Identifiers
3. Create and download a MusicKit Key in Keys
4. Check your Team ID

```bash
APPLE_MUSIC_PRIVATE_KEY="-----BEGIN PRIVATE KEY-----\n...\n-----END PRIVATE KEY-----"
APPLE_MUSIC_KEY_ID="YOUR_KEY_ID"
APPLE_MUSIC_TEAM_KEY="YOUR_TEAM_ID"
```

### Telegram Bot Setup

1. Create a bot with [@BotFather](https://t.me/botfather)
2. Get the bot token
3. Check your Chat ID (use [@userinfobot](https://t.me/userinfobot))

```bash
TELOXIDE_TOKEN="YOUR_BOT_TOKEN"
TELEGRAM_CHAT_ID="YOUR_CHAT_ID"
```

Refer to the `.env.example` file for detailed setup instructions.

## Running the Application

### Development Environment

```bash
# Install dependencies and build
cargo build

# Run development server (port: 1049)
cargo run
```

### Production Environment

```bash
# Release build
cargo build --release

# Run (port: 1047)
cargo run --release
```

### Docker

```bash
# Build and run container
docker-compose up --build -d
```

## API Documentation

### REST API

#### 1. Get Apple Music Developer Token

```http
GET /api/admin/get-developer-token
```

**Response:**

```json
"eyJabGciOiJFUzI1NiIsInR5cCI6IkpAVCJ9..."
```

#### 2. Save User Token

```http
POST /api/admin/save-token
Content-Type: application/json

{
  "userToken": "string"
}
```

**Response:**

```json
"Ok"
```

#### 3. Submit Contact

```http
POST /api/contact
Content-Type: application/json

{
  "name": "John Doe",
  "email": "example@email.com",
  "message": "Your message here"
}
```

**Response:**

```json
"Ok"
```

### WebSocket API

#### Real-time Now Playing Information

```
WS /ws/now-playing
```

**Message Format:**

```json
{
  "isPlaying": true,
  "track": {
    "id": "1234567890",
    "title": "Sunflower",
    "artist": "Post Malone, Swae Lee",
    "album": "Spider-Man: Into the Spider-Verse",
    "artwork": "https://is1-ssl.mzstatic.com/image/thumb/...",
    "duration": 158,
    "currentTime": 45,
    "genreNames": ["Soundtrack", "Hip-Hop"],
    "trackNumber": 1,
    "releaseDate": "2018-12-14",
    "isrc": "USUM71814890",
    "url": "https://music.apple.com/...",
    "hasLyrics": true,
    "previewUrl": "https://audio-ssl.itunes.apple.com/..."
  },
  "timestamp": 1703472123000
}
```

**How it works:**

1. Immediately sends currently cached playback information upon connection
2. Broadcasts new information whenever the track changes
3. Playback time is calculated by the server

## Project Structure

```
portfolio_back/
├── src/
│   ├── apis/
│   │   └── api.rs           # REST API handlers
│   ├── models/
│   │   └── model.rs         # Data models
│   ├── services/
│   │   └── contact.rs       # Contact processing logic
│   ├── websocket/
│   │   └── ws_handler.rs    # WebSocket handlers
│   ├── scheduler.rs         # Apple Music polling scheduler
│   ├── lib.rs
│   └── main.rs
├── contacts/                # Contact storage directory
├── Cargo.toml
├── Rocket.toml              # Rocket configuration
├── Dockerfile
└── docker-compose.yml
```

## CORS Configuration

By default, requests from the following domains are allowed:

- `http://localhost:5173` (development environment)
- `http://kang1027.com`
- `https://kang1027.com`

If changes are needed, modify the CORS settings in `src/main.rs`.

## Port Configuration

- **Default**: 1048
- **Debug**: 1049
- **Release**: 1047

Ports can be changed in `Rocket.toml`.

## License

MIT
