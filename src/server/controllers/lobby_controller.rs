use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use serde::{Deserialize};
use crate::lobby::Lobby;
use crate::player::Player;
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

pub async fn get_lobby_by_id(
    redis_pool: &Pool<RedisConnectionManager>, id: &str
) -> Result<Lobby, String>
{
    Ok( get_struct_from_redis::<Lobby>(redis_pool, id).await?)
}

pub async fn create_lobby(
    redis_pool: &Pool<RedisConnectionManager>, payload: NewLobbyData
) -> Result<Lobby, String>
{
    let lobby = Lobby::new(payload.public);

    set_struct_to_redis::<Lobby>(redis_pool, lobby.get_id(), lobby.clone()).await?;

    Ok(lobby)
}

pub async fn get_lobbies(
    redis_pool: &Pool<RedisConnectionManager>
) -> Result<Vec<Lobby>, String>
{
    Ok(get_vector_from_redis(redis_pool, "LOBBY*").await?)
}

pub async fn delete_lobby(
    redis_pool: &Pool<RedisConnectionManager>, id: &str
) -> Result<(), String>
{
    Ok(delete_struct_from_redis::<Lobby>(redis_pool, id).await?)
}

pub async fn add_player_to_lobby(
    redis_pool: &Pool<RedisConnectionManager>, lobby_id: &str, player_id: &str
) -> Result<(), String>
{
    let mut lobby = get_struct_from_redis::<Lobby>(redis_pool, lobby_id.as_str()).await?;

    // check if player exists
    get_struct_from_redis::<Player>(redis_pool, player_id.as_str()).await?;

    lobby.player_add(&player_id);

    Ok(set_struct_to_redis::<Lobby>(redis_pool, lobby_id.as_str(), lobby).await?)
}

pub async fn delete_player_from_lobby(
    redis_pool: &Pool<RedisConnectionManager>, lobby_id: &str, player_id: &str
) -> Result<(), String>
{
    let mut lobby = get_struct_from_redis::<Lobby>(redis_pool, lobby_id.as_str()).await?;

    // check if player exists
    get_struct_from_redis::<Player>(redis_pool, player_id.as_str()).await?;

    lobby.player_remove(&player_id);

    Ok(set_struct_to_redis::<Lobby>(redis_pool, lobby_id.as_str(), lobby).await?)
}
