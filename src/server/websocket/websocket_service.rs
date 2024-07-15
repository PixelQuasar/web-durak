use std::sync::Arc;
use serde_json::{from_str, to_string};
use tokio::sync::broadcast;
use crate::server::{AppState, LobbyConnection};
use crate::server::controllers::lobby_controller::{create_lobby, NewLobbyData};
use crate::server::errors::error_message;
use crate::server::websocket::{WSLobbyBody};

pub async fn ws_create_lobby (
    req_body: &WSLobbyBody,
    app_state: &Arc<AppState>
) -> Result<(String, broadcast::Sender<String>), String> {
    let payload = match req_body.content.clone() {
        Some(res) => res,
        None => return Err("Body is empty".to_string())
    };
    let lobby_data: NewLobbyData = from_str(&payload).map_err(error_message)?;

    let lobby = create_lobby(&app_state.redis_pool, lobby_data).await?;

    let lobby_connection = LobbyConnection::new();

    app_state.lobby_connections.lock().await.insert(
        lobby.get_id().to_string(),
        lobby_connection.clone()
    );

    Ok((to_string(&lobby).map_err(error_message)?, lobby_connection.tx.clone()))
}
