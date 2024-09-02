use crate::server::errors::error_message;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use redis::AsyncCommands;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub async fn get_struct_from_redis<T: DeserializeOwned>(
    pool: &Pool<RedisConnectionManager>,
    id: &str,
) -> Result<T, String> {
    let mut conn = pool.get().await.map_err(error_message)?;

    let stored_string = conn
        .get::<String, String>(id.to_string())
        .await
        .map_err(error_message)?;

    let parsed_struct = serde_json::from_str::<T>(&stored_string).map_err(error_message)?;

    Ok(parsed_struct)
}

pub async fn set_struct_to_redis<T: Serialize>(
    pool: &Pool<RedisConnectionManager>,
    id: &str,
    payload: T,
) -> Result<(), String> {
    let mut conn = pool.get().await.map_err(error_message)?;

    let str_value = serde_json::to_string(&payload).map_err(error_message)?;

    conn.set::<String, String, String>(id.to_string(), str_value)
        .await
        .map_err(error_message)?;

    Ok(())
}

pub async fn get_vector_from_redis<T: DeserializeOwned>(
    pool: &Pool<RedisConnectionManager>,
    prefix: &str,
) -> Result<Vec<T>, String> {
    let mut conn = pool.get().await.map_err(error_message)?;

    // Some bad code here. But sadly I didn't figure out how to make it better.
    //TODO: rewrite later.
    let mut raw_values = conn
        .scan_match::<&str, String>(prefix)
        .await
        .map_err(error_message)?;
    let mut result: Vec<T> = vec![];

    while let Some(element) = raw_values.next_item().await {
        result.push(get_struct_from_redis(&pool, &element).await?)
    }

    Ok(result)
}

pub async fn delete_struct_from_redis<T: DeserializeOwned + Serialize>(
    pool: &Pool<RedisConnectionManager>,
    id: &str,
) -> Result<(), String> {
    let mut conn = pool.get().await.map_err(error_message)?;

    let result = conn
        .del::<String, i32>(id.to_string())
        .await
        .map_err(error_message)?;

    match result {
        1 => Ok(()),
        _ => Err("Value does not exist.".to_string()),
    }
}
