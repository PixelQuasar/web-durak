use crate::player::Player;
use crate::server::redis_service::{get_struct_from_redis, set_struct_to_redis};
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewPlayerData {
    name: String,
}

pub async fn create_player(
    redis_pool: &Pool<RedisConnectionManager>,
    payload: NewPlayerData,
) -> Result<(), String> {
    let player = Player::new(payload.name.clone());

    set_struct_to_redis::<Player>(&redis_pool, player.get_id(), player.clone()).await?;
    Ok(())
}

pub async fn get_player_by_id(
    redis_pool: &Pool<RedisConnectionManager>,
    id: String,
) -> Result<Player, String> {
    let player = get_struct_from_redis::<Player>(&redis_pool, id.as_str()).await?;

    Ok(player)
}
