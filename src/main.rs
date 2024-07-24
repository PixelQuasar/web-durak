mod lobby;
mod utils;
mod player;
mod game;
mod server;

use crate::server::create_app;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use bb8_redis::{bb8, RedisConnectionManager};
use crate::game::deck_manager::{Card, DeckManager};
use crate::player::Player;

#[tokio::main]
async fn main() {
    // testing game deck state machine

    let players = vec![
        Player::new_with_id("1".to_string(), "Player One".to_string()),
        Player::new_with_id("2".to_string(), "Player Two".to_string()),
        Player::new_with_id("3".to_string(), "Player Three".to_string()),
        Player::new_with_id("4".to_string(), "Player Four".to_string())
    ];

    let player_ids: Vec<String> = players.iter().map(|x| {x.get_id().to_string()}).collect();

    let mut deck_manager = DeckManager::new();
    deck_manager.deal_six(player_ids.clone());

    println!("{}", player_ids[0]);

    deck_manager.init_table(&player_ids[0], Card::new(2, 3)).unwrap();

    deck_manager.beat(&player_ids[3], Card::new(4, 3), Card::new(2, 3)).unwrap();

    deck_manager.toss(&player_ids[2], Card::new(4, 1)).unwrap();

    deck_manager.beat(&player_ids[3], Card::new(11, 3), Card::new(4, 1)).unwrap();

    deck_manager.toss(&player_ids[1], Card::new(11, 1)).unwrap();

    deck_manager.take_table(&player_ids[3]).unwrap();

    println!("{:#?}", deck_manager);



    // load dotenv
    dotenv::dotenv().ok();
    let manager = RedisConnectionManager::new(
        dotenv::var("REDIS_CONNECTION_URL").expect("REDIS_CONNECTION_URL environment variable is empty")
    ).unwrap();
    let redis_pool = bb8::Pool::builder().build(manager).await.unwrap();

    create_app(redis_pool).await;
}
