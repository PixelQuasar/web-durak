pub mod client_request;
pub mod handle_socket;
pub mod process_message;
pub mod websocket_service;

use crate::game::deck_manager::Card;
use crate::lobby::PopulatedLobby;
use crate::server::websocket::handle_socket::handle_socket;
use crate::server::AppState;
use axum::extract::connect_info::ConnectInfo;
use axum::{extract, extract::ws::WebSocketUpgrade, response::IntoResponse};
use axum_extra::TypedHeader;
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use std::net::SocketAddr;
use std::sync::Arc;

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub enum WSRequestType {
    LobbyCreate,
    LobbyJoin,
    GameCreate,
    GameTurnInitTable,
    GameTurnConfirmBeat,
    GameTurnToss,
    GameTurnBeat,
    GameTurnTransfer,
    GameTurnTake,
    GameTurnDiscard,
    GameFinish,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub enum WSErrorType {
    LobbyError,
    GameError,
    Warning,
    ConnectionError,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub enum GameEntityType {
    Nobody,
    Player,
    Deck,
    Table,
    Discard,
}

#[derive(Deserialize, Clone, Debug)]
pub struct WSBody {
    req_type: WSRequestType,
    sender_id: String,
    lobby_id: Option<String>,
    content: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct WSBodyCardContent {
    card: Option<Card>,
    beating: Option<Card>,
    beatable: Option<Card>,
    player_id: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct WSError {
    err_type: WSErrorType,
    message: String,
}

impl WSError {
    // pub fn new(message: String, err_type: WSErrorType) -> WSError {
    //    WSError { message, err_type }
    //}

    pub fn conn_error(msg: String) -> WSError {
        WSError {
            message: msg,
            err_type: WSErrorType::ConnectionError,
        }
    }

    pub fn game_error(msg: String) -> WSError {
        WSError {
            message: msg,
            err_type: WSErrorType::GameError,
        }
    }

    pub fn stringify(&self) -> String {
        to_string::<WSError>(&self).unwrap()
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GameUpdateState {
    receiver_type: GameEntityType,
    sender_type: GameEntityType,
    receiver_id: String,
    sender_id: String,
    cards: Vec<Card>,
}

impl GameUpdateState {
    pub fn new(
        receiver_type: GameEntityType,
        sender_type: GameEntityType,
        receiver_id: String,
        sender_id: String,
        cards: Vec<Card>,
    ) -> GameUpdateState {
        GameUpdateState {
            receiver_type,
            sender_type,
            receiver_id,
            sender_id,
            cards,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WSGameUpdateResponseType {
    lobby: PopulatedLobby,
    game_update_states: Vec<GameUpdateState>,
}

impl WSGameUpdateResponseType {
    pub fn new(
        lobby: PopulatedLobby,
        game_update_states: Vec<GameUpdateState>,
    ) -> WSGameUpdateResponseType {
        WSGameUpdateResponseType {
            lobby,
            game_update_states,
        }
    }
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
    ws.on_upgrade(move |socket| handle_socket(socket, addr, state))
}
