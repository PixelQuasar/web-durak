use std::sync::Arc;
use serde_json::to_string;
use tokio::sync::broadcast;
use crate::lobby::{Lobby, PopulatedLobby};
use crate::server::AppState;
use crate::server::controllers::lobby_controller::get_populated_lobby;
use crate::server::websocket::{WSBody, WSRequestType};
use crate::server::websocket::client_request::{ClientRequest, ClientRequestType};
use crate::server::websocket::websocket_service::{ws_create_lobby, ws_join_lobby, ws_leave_lobby};

pub async fn handle_player_join(
    app_state: &Arc<AppState>,
    request: WSBody
) -> Result<(PopulatedLobby, broadcast::Sender<String>), String> {
    let (lobby, sender) = match request.req_type {
        WSRequestType::LobbyJoin => {
            ws_join_lobby(&app_state, request.clone()).await?
        },
        WSRequestType::LobbyCreate => {
            ws_create_lobby(&app_state, request.clone()).await?
        },
        _ => {
            return Err("Lobby connection error: wrong request type".to_string())
        }
    };

    let lobby = get_populated_lobby(&app_state.redis_pool, lobby.get_id()).await?;

    Ok((lobby, sender))
}

pub async fn handle_message(
    request: WSBody
) -> Result<String, String> {
    Ok(match request.req_type {
        WSRequestType::GameTurn => {
            "hi!".to_string()
        },
        _ => {
            "idk what is that".to_string()
        }
    })
}

pub async fn disconnect_message(
    app_state: &Arc<AppState>,
    current_lobby_id: Option<String>,
    current_player_id: Option<String>
) -> Result<String, String> {
    if current_lobby_id.is_some() && current_player_id.is_some() {
        let lobby = ws_leave_lobby(&app_state, &current_lobby_id.unwrap(), &current_player_id.unwrap()).await?;
        let lobby = get_populated_lobby(&app_state.redis_pool, lobby.get_id()).await?;
        Ok(ClientRequest::new(
            ClientRequestType::LobbyUpdate, to_string::<PopulatedLobby>(&lobby).unwrap()
        ).to_string())
    } else {
        println!("Websocket connection closing error:");
        Err("Websocket connection closing error: invalid player or lobby".to_string())
    }
}
