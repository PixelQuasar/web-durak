use serde::{Deserialize, Serialize};
use crate::player::Player;
use crate::utils::generate_id;

pub enum GameLoopState {
    START, PAUSE, FINISH, TURN,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Game {
    id: u64,
    participants: Vec<Player>,
    target_player: Option<Player>,
    turn_queue: Vec<Player>
}

impl Game {
    pub fn new(players: Vec<Player>) -> Game {
        Game {
            id: generate_id(),
            participants: players,
            target_player: None,
            turn_queue: vec![]
        }
    }
}
