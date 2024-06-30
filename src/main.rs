mod lobby;
mod utils;
mod player;
mod game;

use crate::lobby::Lobby;

fn main() {
    let lobby = Lobby::new();

    println!("Lobby: {:?}", lobby);
}
