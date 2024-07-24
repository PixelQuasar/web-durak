pub mod deck_manager;

use serde::{Deserialize, Serialize};
use crate::game::deck_manager::DeckManager;
use crate::player::Player;
use crate::utils::{gen_special_id};

pub enum GameLoopState {
    START, PAUSE, FINISH, TURN,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Game {
    id: String,
    participant_ids: Vec<String>,
    target_player_id: Option<String>,
    turn_queue: Vec<String>,
    pub deck_manager: DeckManager
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PopulatedGame {
    id: String,
    participant_ids: Vec<Option<Player>>,
    target_player_id: Option<Player>,
    turn_queue: Vec<Option<Player>>,
    pub deck_manager: DeckManager
}

impl Game {
    pub fn new(players: Vec<String>) -> Game {
        Game {
            id: gen_special_id("GAME"),
            participant_ids: players,
            target_player_id: None,
            turn_queue: vec![],
            deck_manager: DeckManager::new()
        }
    }

    //pub fn init_turn
}
