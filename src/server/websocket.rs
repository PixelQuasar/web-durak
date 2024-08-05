pub mod handle_socket;
pub mod process_message;
pub mod websocket_service;
pub mod client_request;

use axum::{extract, extract::ws::{WebSocketUpgrade}, response::IntoResponse};
use axum_extra::TypedHeader;
use std::net::SocketAddr;
use std::sync::Arc;
use axum::extract::connect_info::ConnectInfo;
use axum::extract::ws::{Message, WebSocket};
use futures_util::SinkExt;
use futures_util::stream::SplitSink;
use serde::Deserialize;
use crate::game::deck_manager::Card;
use crate::server::AppState;
use crate::server::websocket::handle_socket::handle_socket;

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub enum  WSRequestType {
    LobbyCreate, LobbyJoin, GameCreate, GameTurnInitTable, GameTurnConfirmBeat,
    GameTurnToss, GameTurnBeat, GameTurnTake, GameTurnDiscard
}

#[derive(Deserialize, Clone, Debug)]
pub struct WSBody {
    req_type: WSRequestType,
    sender_id: String,
    lobby_id: Option<String>,
    content: Option<String>
}

#[derive(Deserialize, Clone, Debug)]
pub struct WSBodyCardContent {
    card: Option<Card>,
    beating: Option<Card>,
    beatable: Option<Card>,
    player_id: Option<String>
}

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    extract::State(state): extract::State<Arc<AppState>>,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    println!("Websocket connection established.");
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        "Unknown browser".to_string()
    };
    println!("`{user_agent}` at {addr} connected.");
    ws.on_upgrade(move |socket| {
        handle_socket(socket, addr, state)
    })
}

pub async fn handle_error(ws_sender: &mut SplitSink<WebSocket, Message>, msg: &str) {
    println!("Websocket error: {}", msg);
    let _ = ws_sender.send(Message::Text("Websocket error".to_string())).await;
}
