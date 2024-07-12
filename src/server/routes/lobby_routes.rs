use axum::{extract::Path, extract::State, extract::Json};
use axum::http::StatusCode;
use serde::{Deserialize};
use crate::lobby::Lobby;
use crate::player::Player;
use crate::server::AppState;
use crate::server::errors::{error_msg_to_server_error};
use crate::server::redis_service::{
    delete_struct_from_redis,
    get_struct_from_redis,
    get_vector_from_redis,
    set_struct_to_redis
};

#[derive(Deserialize)]
pub struct NewLobbyData {
    public: bool
}

pub async fn route_get_lobby_by_id(
    State(state): State<AppState>,
    Path(id): Path<String>
) -> Result<Json<Lobby>, (StatusCode, String)>
{
    let lobby = get_struct_from_redis::<Lobby>(&state.redis_pool, id.as_str())
        .await.map_err(error_msg_to_server_error)?;

    Ok(Json(lobby))
}

pub async fn route_create_lobby(
    State(state): State<AppState>,
    payload: Option<Json<NewLobbyData>>
) -> Result<Json<Lobby>, (StatusCode, String)>
{
    if payload.is_none() {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "Body is empty".to_string()));
    }

    let payload = payload.unwrap();

    let lobby = Lobby::new(payload.public);

    set_struct_to_redis::<Lobby>(&state.redis_pool, lobby.get_id(), lobby.clone())
        .await.map_err(error_msg_to_server_error)?;
    Ok(Json(lobby))
}

pub async fn route_get_lobbies(
    State(state): State<AppState>,
) -> Result<Json<Vec<Lobby>>, (StatusCode, String)>
{
    let lobbies = get_vector_from_redis(&state.redis_pool, "LOBBY*")
        .await.map_err(error_msg_to_server_error)?;

    Ok(Json(lobbies))
}

pub async fn route_delete_lobby(
    State(state): State<AppState>,
    Path(id): Path<String>
) -> Result<(), (StatusCode, String)>
{
    delete_struct_from_redis::<Lobby>(&state.redis_pool, id.as_str())
        .await.map_err(error_msg_to_server_error)?;

    Ok(())
}

pub async fn route_add_player_to_lobby(
    State(state): State<AppState>,
    Path(lobby_id): Path<String>,
    Path(player_id): Path<String>
) -> Result<(), (StatusCode, String)>
{
    let mut lobby = get_struct_from_redis::<Lobby>(&state.redis_pool, lobby_id.as_str())
        .await.map_err(error_msg_to_server_error)?;

    // check if player exists
    get_struct_from_redis::<Player>(&state.redis_pool, player_id.as_str())
        .await.map_err(error_msg_to_server_error)?;

    lobby.player_add(&player_id);

    set_struct_to_redis::<Lobby>(&state.redis_pool, lobby_id.as_str(), lobby)
        .await.map_err(error_msg_to_server_error)?;

    Ok(())
}

pub async fn route_delete_player_from_lobby(
    State(state): State<AppState>,
    Path(lobby_id): Path<String>,
    Path(player_id): Path<String>
) -> Result<(), (StatusCode, String)>
{
    let mut lobby = get_struct_from_redis::<Lobby>(&state.redis_pool, lobby_id.as_str())
        .await.map_err(error_msg_to_server_error)?;

    // check if player exists
    get_struct_from_redis::<Player>(&state.redis_pool, player_id.as_str())
        .await.map_err(error_msg_to_server_error)?;

    lobby.player_remove(&player_id);

    set_struct_to_redis::<Lobby>(&state.redis_pool, lobby_id.as_str(), lobby)
        .await.map_err(error_msg_to_server_error)?;

    Ok(())
}
