use crate::player::Player;
use crate::server::redis_service::{get_struct_from_redis, set_struct_to_redis};
use bb8::Pool;
use bb8_redis::RedisConnectionManager;

pub async fn get_player_by_id(
    redis_pool: &Pool<RedisConnectionManager>,
    id: String,
) -> Result<Player, String> {
    let player = get_struct_from_redis::<Player>(&redis_pool, id.as_str()).await?;

    Ok(player)
}

pub async fn add_player_score(
    redis_pool: &Pool<RedisConnectionManager>,
    id: String,
    value: usize,
) -> Result<(), String> {
    let mut player = get_struct_from_redis::<Player>(&redis_pool, id.as_str()).await?;

    player.add_score(value);

    set_struct_to_redis::<Player>(&redis_pool, player.get_id(), player.clone()).await?;
    Ok(())
}
