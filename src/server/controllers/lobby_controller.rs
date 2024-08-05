use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use futures_util::future::join_all;
use serde::{Deserialize};
use crate::lobby::{Lobby, PopulatedLobby};
use crate::player::Player;
use crate::server::redis_service::{
    delete_struct_from_redis, get_struct_from_redis,
    get_vector_from_redis, set_struct_to_redis
};

#[derive(Deserialize)]
pub struct NewLobbyData {
    public: bool
}

pub async fn get_lobby_by_id(
    redis_pool: &Pool<RedisConnectionManager>, id: &str
) -> Result<Lobby, String>
{
    Ok(get_struct_from_redis::<Lobby>(redis_pool, id).await?)
}

pub async fn create_lobby(
    redis_pool: &Pool<RedisConnectionManager>, payload: NewLobbyData
) -> Result<Lobby, String>
{
    let lobby = Lobby::new(payload.public);

    set_struct_to_redis::<Lobby>(redis_pool, lobby.get_id(), lobby.clone()).await?;

    Ok(lobby)
}

pub async fn save_lobby(
    redis_pool: &Pool<RedisConnectionManager>, lobby: Lobby
) -> Result<(), String>
{
    set_struct_to_redis::<Lobby>(redis_pool, lobby.get_id(), lobby.clone()).await?;

    Ok(())
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
) -> Result<Lobby, String>
{
    let mut lobby = get_struct_from_redis::<Lobby>(redis_pool, lobby_id).await?;

    let player = get_struct_from_redis::<Player>(redis_pool, player_id).await?;

    // check if player exists
    get_struct_from_redis::<Player>(redis_pool, player_id).await?;

    lobby.player_add(&player_id);

    set_struct_to_redis::<Lobby>(redis_pool, lobby_id, lobby.clone()).await?;

    Ok(lobby)
}

pub async fn delete_player_from_lobby(
    redis_pool: &Pool<RedisConnectionManager>, lobby_id: &str, player_id: &str
) -> Result<(), String>
{
    let mut lobby = get_struct_from_redis::<Lobby>(redis_pool, lobby_id).await?;

    // check if player exists
    get_struct_from_redis::<Player>(redis_pool, player_id).await?;

    lobby.player_remove(&player_id);

    Ok(set_struct_to_redis::<Lobby>(redis_pool, lobby_id, lobby).await?)
}

pub async fn get_populated_lobby(
    redis_pool: &Pool<RedisConnectionManager>, lobby_id: &str
) -> Result<PopulatedLobby, String> {
    let lobby = get_struct_from_redis::<Lobby>(redis_pool, lobby_id).await?;

    let players: Vec<Option<Player>> = join_all(lobby.player_list().into_iter().map(|item: &String| async move {
        match get_struct_from_redis::<Player>(&redis_pool, &item).await {
            Ok(result) => Some(result),
            Err(e) => None
        }
    })).await;

    Ok(PopulatedLobby::from_lobby(lobby, players))
}

pub async fn add_player_to_populated_lobby(
    redis_pool: &Pool<RedisConnectionManager>, lobby_id: &str, player_id: &str
) -> Result<PopulatedLobby, String>
{
    let mut lobby = get_struct_from_redis::<Lobby>(redis_pool, lobby_id).await?;

    let lobby = add_player_to_lobby(&redis_pool, lobby_id.clone(), player_id).await?;

    set_struct_to_redis::<Lobby>(redis_pool, lobby_id, lobby.clone()).await?;

    let populated_lobby = get_populated_lobby(&redis_pool, lobby_id).await?;

    Ok(populated_lobby)
}
