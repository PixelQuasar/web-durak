use std::sync::Arc;
use axum::{extract::Path, extract::State, extract::Json};
use axum::http::StatusCode;
use crate::lobby::Lobby;
use crate::server::AppState;
use crate::server::controllers::lobby_controller::{
    add_player_to_lobby,
    create_lobby,
    delete_lobby,
    delete_player_from_lobby,
    get_lobbies,
    get_lobby_by_id,
    NewLobbyData
};
use crate::server::errors::{error_msg_to_server_error};

pub async fn route_get_lobby_by_id(
    State(state): State<Arc<AppState>>, Path(id): Path<String>
) -> Result<Json<Lobby>, (StatusCode, String)>
{
    Ok(Json(get_lobby_by_id(&state.redis_pool, &id).await.map_err(error_msg_to_server_error)?))
}

pub async fn route_create_lobby(
    State(state): State<Arc<AppState>>, payload: Option<Json<NewLobbyData>>
) -> Result<Json<Lobby>, (StatusCode, String)>
{
    if payload.is_none() {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "Body is empty".to_string()));
    }

    Ok(Json(create_lobby(&state.redis_pool, payload.unwrap().0).await.map_err(error_msg_to_server_error)?))
}

pub async fn route_get_lobbies(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Lobby>>, (StatusCode, String)>
{
    Ok(Json(get_lobbies(&state.redis_pool).await.map_err(error_msg_to_server_error)?))
}

pub async fn route_delete_lobby(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>
) -> Result<(), (StatusCode, String)>
{
    Ok(delete_lobby(&state.redis_pool, &id).await.map_err(error_msg_to_server_error)?)
}

pub async fn route_add_player_to_lobby(
    State(state): State<Arc<AppState>>,
    Path(lobby_id): Path<String>,
    Path(player_id): Path<String>
) -> Result<(), (StatusCode, String)>
{
    Ok(add_player_to_lobby(&state.redis_pool, &lobby_id, &player_id).await.map_err(error_msg_to_server_error)?)
}

pub async fn route_delete_player_from_lobby(
    State(state): State<Arc<AppState>>,
    Path(lobby_id): Path<String>,
    Path(player_id): Path<String>
) -> Result<(), (StatusCode, String)>
{
    Ok(delete_player_from_lobby(&state.redis_pool, &lobby_id, &player_id).await.map_err(error_msg_to_server_error)?)
}