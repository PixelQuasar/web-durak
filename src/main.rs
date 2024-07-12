mod lobby;
mod utils;
mod player;
mod game;
mod server;

use crate::server::create_app;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use bb8_redis::{bb8, RedisConnectionManager};

#[tokio::main]
async fn main() {
    // load dotenv
    dotenv::dotenv().ok();
    let manager = RedisConnectionManager::new(
        dotenv::var("REDIS_CONNECTION_URL").expect("REDIS_CONNECTION_URL environment variable is empty")
    ).unwrap();
    let redis_pool = bb8::Pool::builder().build(manager).await.unwrap();

    create_app(redis_pool).await;
}
