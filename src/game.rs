use serde::{Deserialize, Serialize};
use crate::utils::generate_id;

pub enum GameLoopState {
    START, PAUSE, FINISH, TURN,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Game {
    id: u64,
    participant_ids: Vec<u64>,
    target_player_id: Option<u64>,
    turn_queue: Vec<u64>
}

impl Game {
    pub fn new(players: Vec<u64>) -> Game {
        Game {
            id: generate_id(),
            participant_ids: players,
            target_player_id: None,
            turn_queue: vec![]
        }
    }
}
