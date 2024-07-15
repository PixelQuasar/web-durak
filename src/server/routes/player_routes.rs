use std::sync::Arc;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;
use crate::player::Player;
use crate::server::AppState;
use crate::server::errors::error_msg_to_server_error;
use crate::server::redis_service::{get_struct_from_redis, set_struct_to_redis};

#[derive(Deserialize)]
pub struct NewPlayerData {
    name: String
}

pub async fn route_create_player(
    State(state): State<Arc<AppState>>,
    payload: Option<Json<NewPlayerData>>
) -> Result<Json<Player>, (StatusCode, String)>
{
    if payload.is_none() {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "Body is empty".to_string()));
    }

    let payload = payload.unwrap();

    let player = Player::new(payload.name.clone());

    set_struct_to_redis::<Player>(&state.redis_pool, player.get_id(), player.clone())
        .await.map_err(error_msg_to_server_error)?;
    Ok(Json(player))
}

pub async fn route_get_player_by_id(
    State(state): State<Arc<AppState>>,
    id: String
) -> Result<Json<Player>, (StatusCode, String)>
{
    let player = get_struct_from_redis::<Player>(&state.redis_pool, id.as_str())
        .await.map_err(error_msg_to_server_error)?;

    Ok(Json(player))
}
