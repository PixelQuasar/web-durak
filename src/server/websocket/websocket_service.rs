use std::sync::Arc;
use serde_json::{from_str, to_string};
use tokio::sync::broadcast;
use crate::server::{AppState, LobbyConnection};
use crate::server::controllers::lobby_controller::{
    add_player_to_lobby, create_lobby, delete_lobby,
    delete_player_from_lobby, get_lobby_by_id, NewLobbyData};
use crate::server::errors::error_message;
use crate::server::websocket::{WSLobbyBody};

pub async fn ws_create_lobby (
    app_state: &Arc<AppState>,
    req_body: WSLobbyBody
) -> Result<(String, broadcast::Sender<String>), String> {
    let payload = match req_body.content {
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

pub async fn ws_join_lobby (
    app_state: &Arc<AppState>,
    req_body: WSLobbyBody
) -> Result<(String, broadcast::Sender<String>), String> {
    let lobby_id = match req_body.lobby_id {
        Some(res) => res,
        None => return Err("Body is empty".to_string())
    };
    let player_id = req_body.sender_id;

    let lobby = get_lobby_by_id(&app_state.redis_pool, &lobby_id).await?;

    let tx = match app_state.lobby_connections.lock().await.get(&lobby_id) {
        Some(result) => result.tx.clone(),
        None => return Err("Invalid lobby id".to_string())
    };

    add_player_to_lobby(&app_state.redis_pool, &lobby_id, &player_id).await?;

    Ok((to_string(&lobby).map_err(error_message)?, tx))
}

pub async fn ws_leave_lobby (
    app_state: &Arc<AppState>,
    lobby_id: &str,
    player_id: &str
) -> Result<(), String> {
    let lobby = get_lobby_by_id(&app_state.redis_pool, &lobby_id).await?;

   if lobby.players_num() == 1 {
       delete_lobby(&app_state.redis_pool, &lobby_id).await?;
       app_state.lobby_connections.lock().await.remove(&lobby_id);
   } else {
       delete_player_from_lobby(&app_state.redis_pool, &lobby_id, &player_id).await?;
   };

    Ok(())
}
