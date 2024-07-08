use serde::{Deserialize, Serialize};
use crate::utils::{gen_special_id};

pub enum GameLoopState {
    START, PAUSE, FINISH, TURN,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Game {
    id: String,
    participant_ids: Vec<String>,
    target_player_id: Option<String>,
    turn_queue: Vec<String>
}

impl Game {
    pub fn new(players: Vec<String>) -> Game {
        Game {
            id: gen_special_id("GAME"),
            participant_ids: players,
            target_player_id: None,
            turn_queue: vec![]
        }
    }
}
