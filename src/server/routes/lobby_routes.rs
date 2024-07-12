use axum::{extract::Path, extract::State, extract::Json};
use axum::http::StatusCode;
use serde::{Deserialize};
use crate::lobby::Lobby;
use crate::player::Player;
use crate::server::AppState;
use crate::server::controllers::lobby_controller::{
    add_player_to_lobby,
    create_lobby,
    delete_player_from_lobby,
    get_lobbies,
    get_lobby_by_id,
    NewLobbyData
};
use crate::server::errors::{error_msg_to_server_error};
use crate::server::redis_service::{
    delete_struct_from_redis,
    get_struct_from_redis,
    get_vector_from_redis,
    set_struct_to_redis
};

pub async fn route_get_lobby_by_id(
    State(state): State<AppState>, Path(id): Path<String>
) -> Result<Json<Lobby>, (StatusCode, String)>
{
    Ok(Json(get_lobby_by_id(&state.redis_pool, &id).await.map_err(error_msg_to_server_error)?))
}

pub async fn route_create_lobby(
    State(state): State<AppState>, payload: Option<Json<NewLobbyData>>
) -> Result<Json<Lobby>, (StatusCode, String)>
{
    if payload.is_none() {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "Body is empty".to_string()));
    }

    Ok(Json(create_lobby(&state.redis_pool, payload.unwrap().0).await.map_err(error_msg_to_server_error)?))
}

pub async fn route_get_lobbies(
    State(state): State<AppState>,
) -> Result<Json<Vec<Lobby>>, (StatusCode, String)>
{
    Ok(Json(get_lobbies(&state.redis_pool).await.map_err(error_msg_to_server_error)?))
}

pub async fn route_delete_lobby(
    State(state): State<AppState>,
    Path(id): Path<String>
) -> Result<(), (StatusCode, String)>
{
    Ok(delete_struct_from_redis(&state.redis_pool, &id).await.map_err(error_msg_to_server_error)?)
}

pub async fn route_add_player_to_lobby(
    State(state): State<AppState>,
    Path(lobby_id): Path<String>,
    Path(player_id): Path<String>
) -> Result<(), (StatusCode, String)>
{
    Ok(add_player_to_lobby(&state.redis_pool, &lobby_id, &player_id).await.map_err(error_msg_to_server_error)?)
}

pub async fn route_delete_player_from_lobby(
    State(state): State<AppState>,
    Path(lobby_id): Path<String>,
    Path(player_id): Path<String>
) -> Result<(), (StatusCode, String)>
{
    Ok(delete_player_from_lobby(&state.redis_pool, &lobby_id, &player_id).await.map_err(error_msg_to_server_error)?)
}
