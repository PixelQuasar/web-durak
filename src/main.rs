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


#[tokio::main]
async fn main() {
    let mut lobby = Lobby::new();
    // lobby.player_add(Player::new("name1"));
    // lobby.player_add(Player::new("name2"));
    // lobby.player_add(Player::new("name3"));

    let mut lobby_pool = HashMap::new();
    lobby_pool.insert(lobby.get_id(), lobby);

    create_app(lobby_pool).await;
}
