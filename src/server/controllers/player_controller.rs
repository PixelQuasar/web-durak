use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;
use crate::player::Player;
use crate::server::AppState;
use crate::server::errors::error_msg_to_server_error;
use crate::server::redis_service::set_struct_to_redis;

#[derive(Deserialize)]
pub struct NewPlayerData {
    name: String
}

pub async fn create_player(
    State(state): State<AppState>,
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
