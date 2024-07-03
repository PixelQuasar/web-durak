use std::sync::Arc;
use axum::{extract, Json};
use axum::http::StatusCode;
use tokio::sync::RwLock;
use crate::lobby::Lobby;
use crate::server::AppState;

pub struct NewLobbyData {
    public: bool
}

pub async fn get_lobby_by_id(
    extract::Extension(state): extract::Extension<Arc<RwLock<AppState>>>,
    extract::Path(id): extract::Path<u64>
) -> Result<Json<Lobby>, StatusCode>
{
    let awaited_state = &state.read().await;
    let lobby = awaited_state.game_data.lobby_pool.get(&id);

    match lobby {
        None => Err(StatusCode::INTERNAL_SERVER_ERROR),
        Some(result) => Ok(Json(result.to_owned()))
    }
}

pub async fn create_lobby(
    extract::Extension(mut state): extract::Extension<Arc<RwLock<AppState>>>,
    payload: Option<Json<NewLobbyData>>
) -> Result<Json<Lobby>, StatusCode>
{
    if let Some(payload) = payload {
        return Err(StatusCode::BAD_REQUEST)
    }

    let payload = payload.unwrap();
    let lobby = Lobby::new(payload.public);

    state.write().await.game_data.lobby_pool.insert(lobby.get_id(), lobby.clone());

    Ok(Json(lobby))
}

pub async fn get_lobby(
    extract::Extension(state): extract::Extension<Arc<RwLock<AppState>>>,
) -> Result<Json<Vec<Lobby>>, StatusCode>
{
    let lobbies = &state.read().await.game_data.lobby_pool;
    let lobbies: Vec<Lobby> = lobbies.values().cloned().collect();

    Ok(Json(lobbies))
}

pub async fn delete_lobby(
    extract::Extension(mut state): extract::Extension<Arc<RwLock<AppState>>>,
    extract::Path(mut id): extract::Path<u64>
) -> Result<Json<Lobby>, StatusCode>
{
    let lobby = state.write().await.game_data.lobby_pool.remove(&id);

    match lobby {
        None => Err(StatusCode::INTERNAL_SERVER_ERROR),
        Some(result) =>  Ok(Json(result.clone()))
    }
}
