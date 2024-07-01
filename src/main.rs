mod lobby;
mod utils;
mod player;
mod game;

use std::cell::RefCell;
use crate::lobby::Lobby;
use crate::player::Player;

fn main() {
    let mut lobby = Lobby::new();
    let players = vec![
        Player::new("name1"),
        Player::new("name2"),
        Player::new("name3")
    ];

    players.iter().for_each(|item| {
       lobby.player_add(item.clone());
    });

    lobby.init_game();
    println!("Lobby: {:#?}", lobby);
}
