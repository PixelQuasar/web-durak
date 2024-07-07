mod lobby;
mod utils;
mod player;
mod game;
mod server;
mod routes;

use std::collections::HashMap;
use crate::lobby::Lobby;
use crate::player::Player;
use crate::server::create_app;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use bb8_redis::{bb8, RedisConnectionManager};

#[tokio::main]
async fn main() {
    // load dotenv
    dotenv::dotenv().ok();


    // for debug
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_tokio_redis=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    tracing::debug!("connecting to redis");

    let manager = RedisConnectionManager::new("redis://localhost").unwrap();
    let redis_pool = bb8::Pool::builder().build(manager).await.unwrap();

    create_app(redis_pool).await;
}
