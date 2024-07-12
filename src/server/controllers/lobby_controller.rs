use axum::{extract::Path, extract::State, extract::Json};
use axum::http::StatusCode;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use serde::{Deserialize};
use crate::lobby::Lobby;
use crate::player::Player;
use crate::server::AppState;
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

pub async fn get_lobby_by_id(id: &str, redis_pool: &Pool<RedisConnectionManager>) -> Result<Lobby, String>
{
    let lobby = get_struct_from_redis::<Lobby>(redis_pool, id.as_str()).await?;

    Ok(Json(lobby))
}

pub async fn create_lobby(redis_pool: &Pool<RedisConnectionManager>, payload: NewLobbyData) -> Result<(), String>
{
    let lobby = Lobby::new(payload.public);

    set_struct_to_redis::<Lobby>(redis_pool, lobby.get_id(), lobby.clone()).await?;
    Ok(())
}

pub async fn get_lobbies(redis_pool: &Pool<RedisConnectionManager>) -> Result<Vec<Lobby>, String>
{
    let lobbies = get_vector_from_redis(redis_pool, "LOBBY*")
        .await?;

    Ok(lobbies)
}

pub async fn delete_lobby(redis_pool: &Pool<RedisConnectionManager>, id: String
) -> Result<(), String>
{
    delete_struct_from_redis::<Lobby>(redis_pool, id.as_str()).await?;

    Ok(())
}

pub async fn add_player_to_lobby(
    redis_pool: &Pool<RedisConnectionManager>,
    lobby_id: String,
    player_id: String
) -> Result<(), String>
{
    let mut lobby = get_struct_from_redis::<Lobby>(redis_pool, lobby_id.as_str()).await?;

    // check if player exists
    get_struct_from_redis::<Player>(redis_pool, player_id.as_str()).await?;

    lobby.player_add(&player_id);

    set_struct_to_redis::<Lobby>(redis_pool, lobby_id.as_str(), lobby).await?;

    Ok(())
}

pub async fn delete_player_from_lobby(
    redis_pool: &Pool<RedisConnectionManager>,
    lobby_id: String,
    player_id: String
) -> Result<(), String>
{
    let mut lobby = get_struct_from_redis::<Lobby>(redis_pool, lobby_id.as_str()).await?;

    // check if player exists
    get_struct_from_redis::<Player>(redis_pool, player_id.as_str()).await?;

    lobby.player_remove(&player_id);

    set_struct_to_redis::<Lobby>(redis_pool, lobby_id.as_str(), lobby).await?;

    Ok(())
}
