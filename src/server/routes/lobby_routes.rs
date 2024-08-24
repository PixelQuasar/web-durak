use crate::lobby::{Lobby, PopulatedLobby};
use crate::player::Player;
use crate::server::controllers::lobby_controller::{
    create_lobby, delete_lobby, get_lobbies, get_lobby_score_board, get_populated_lobby,
    NewLobbyData,
};
use crate::server::errors::error_msg_to_server_error;
use crate::server::AppState;
use axum::http::StatusCode;
use axum::{extract::Json, extract::Path, extract::State};
use std::sync::Arc;

pub async fn route_get_lobby_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<PopulatedLobby>, (StatusCode, String)> {
    Ok(Json(
        get_populated_lobby(&state.redis_pool, &id)
            .await
            .map_err(error_msg_to_server_error)?,
    ))
}

pub async fn route_create_lobby(
    State(state): State<Arc<AppState>>,
    payload: Option<Json<NewLobbyData>>,
) -> Result<Json<Lobby>, (StatusCode, String)> {
    if payload.is_none() {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Body is empty".to_string(),
        ));
    }

    Ok(Json(
        create_lobby(&state.redis_pool, payload.unwrap().0)
            .await
            .map_err(error_msg_to_server_error)?,
    ))
}

pub async fn route_get_lobbies(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<PopulatedLobby>>, (StatusCode, String)> {
    let lobbies = get_lobbies(&state.redis_pool)
        .await
        .map_err(error_msg_to_server_error)?;

    let mut populated_lobbies = vec![];

    for lobby in lobbies {
        match get_populated_lobby(&state.redis_pool, lobby.get_id()).await {
            Ok(result) => populated_lobbies.push(result),
            Err(_) => ()
        }
    }

    Ok(Json(
        populated_lobbies
    ))
}

pub async fn route_delete_lobby(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<(), (StatusCode, String)> {
    Ok(delete_lobby(&state.redis_pool, &id)
        .await
        .map_err(error_msg_to_server_error)?)
}

pub async fn route_get_lobby_score_board(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<Vec<(Player, usize)>>, (StatusCode, String)> {
    Ok(Json(
        get_lobby_score_board(&state.redis_pool, &id)
            .await
            .map_err(error_msg_to_server_error)?,
    ))
}
