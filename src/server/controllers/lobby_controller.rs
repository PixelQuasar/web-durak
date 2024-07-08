use axum::{extract::Path, extract::State, extract::Json};
use axum::http::StatusCode;
use serde::{Deserialize};
use crate::lobby::Lobby;
use crate::server::AppState;
use crate::server::errors::{error_msg_to_server_error};
use crate::server::redis_service::{delete_struct_from_redis, get_struct_from_redis, get_vector_from_redis, set_struct_to_redis};

#[derive(Deserialize)]
pub struct NewLobbyData {
    public: bool
}

pub async fn get_lobby_by_id(
    State(state): State<AppState>,
    Path(id): Path<String>
) -> Result<Json<Lobby>, (StatusCode, String)>
{
    let lobby = get_struct_from_redis::<Lobby>(&state.redis_pool, id.as_str())
       .await.map_err(error_msg_to_server_error)?;

    Ok(Json(lobby))
}

pub async fn create_lobby(
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

pub async fn get_lobbies(
    State(state): State<AppState>,
) -> Result<Json<Vec<Lobby>>, (StsatusCode, String)>
{
    let lobbies = get_vector_from_redis(&state.redis_pool, "LOBBY*")
        .await.map_err(error_msg_to_server_error)?;

    Ok(Json(lobbies))
}

pub async fn delete_lobby(
    State(state): State<AppState>,
    Path(id): Path<String>
) -> Result<(), (StatusCode, String)>
{
    delete_struct_from_redis::<Lobby>(&state.redis_pool, id.as_str())
        .await.map_err(error_msg_to_server_error)?;

    Ok(())
}
