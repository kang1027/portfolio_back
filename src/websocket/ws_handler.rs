use rocket::futures::SinkExt;
use rocket::{State, get};
use rocket_ws::{Channel, Message, WebSocket};
use tokio_stream::{StreamExt, wrappers::BroadcastStream};

use crate::models::model::AppState;

#[get("/ws/now-playing")]
pub fn now_playing_ws(ws: WebSocket, app_state: &State<AppState>) -> Channel<'static> {
    let rx = app_state.broadcast_tx.subscribe();
    let cached = app_state.cached_now_playing.clone();

    ws.channel(move |mut ws_stream| {
        Box::pin(async move {
            // 1. 연결 시 현재 캐시된 데이터 전송
            let initial_json = {
                let current = cached.read().unwrap();
                serde_json::to_string(&*current).ok()
            };

            if let Some(json) = initial_json {
                if let Err(_) = ws_stream.send(Message::Text(json)).await {
                    println!("초기 데이터 전송 실패");
                    return Ok(());
                }
            }

            // 2. Broadcast 스트림으로 실시간 업데이트 수신
            let mut broadcast_stream = BroadcastStream::new(rx);

            loop {
                tokio::select! {
                    // Broadcast에서 새로운 데이터 수신
                    Some(Ok(now_playing)) = StreamExt::next(&mut broadcast_stream) => {
                        if let Ok(json) = serde_json::to_string(&now_playing) {
                            if ws_stream.send(Message::Text(json)).await.is_err() {
                                break;
                            }
                        }
                    }
                    // 클라이언트로부터 메시지 수신 (keep-alive)
                    Some(msg) = rocket::futures::StreamExt::next(&mut ws_stream) => {
                        match msg {
                            Ok(Message::Close(_)) => break,
                            Err(_) => break,
                            _ => {} // ping/pong이나 다른 메시지는 무시
                        }
                    }
                    else => break,
                }
            }

            Ok(())
        })
    })
}
