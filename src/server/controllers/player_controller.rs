use std::sync::Arc;
use axum::{extract, Json};
use axum::http::StatusCode;
use tokio::sync::RwLock;
use crate::player::Player;
use crate::server::AppState;

pub struct NewPlayerData {
    name: String
}

pub async fn create_player(
    extract::Extension(state): extract::Extension<Arc<RwLock<AppState>>>,
    payload: Option<Json<NewPlayerData>>
) -> Result<Json<Player>, StatusCode>
{
    if let Some(payload) = payload {
        return Err(StatusCode::BAD_REQUEST)
    }

    let payload = payload.unwrap();
    let new_player = Player::new(payload.name.to_string());

    state.write().await.game_data.player_pool.insert(new_player.get_id(), new_player.clone());

    Ok(Json(new_player))
}

pub async fn get_player_by_id(
    extract::Extension(state): extract::Extension<Arc<RwLock<AppState>>>,
    extract::Path(id): extract::Path<u64>
) -> Result<Json<Player>, StatusCode>
{
    let awaited_state = state.read().await;
    let player = awaited_state.game_data.player_pool.get(&id);

    match player {
        None => Err(StatusCode::INTERNAL_SERVER_ERROR),
        Some(result) => Ok(Json(result.to_owned()))
    }
}

pub async fn delete_player(
    extract::Extension(state): extract::Extension<Arc<RwLock<AppState>>>,
    extract::Path(id): extract::Path<u64>
) -> Result<Json<Player>, StatusCode>
{
    let player = state.write().await.game_data.player_pool.remove(&id);

    match player {
        None => Err(StatusCode::INTERNAL_SERVER_ERROR),
        Some(result) =>  Ok(Json(result.clone()))
    }
}
