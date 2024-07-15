use std::sync::Arc;
use axum::extract::ws::{Message, WebSocket};
use futures_util::SinkExt;
use futures_util::stream::SplitSink;
use crate::server::{AppState};
use crate::server::websocket::{WSGameTurnBody, WSGameTurnRequestType};

pub async fn message_body_handler (
    req_body: &WSGameTurnBody,
    ws_sender: &mut SplitSink<WebSocket, Message>,
    app_state: Arc<AppState>
) {
    let resp = match req_body.req_type {
        WSGameTurnRequestType::GameCreate => {
           "empty".to_string()
       }
        WSGameTurnRequestType::GameTurn => {
           "empty".to_string()
       }
    };

    let _ = ws_sender.send(Message::Text(resp)).await;
}
