pub mod deck_manager;
use std::cmp::PartialEq;
use serde::{Deserialize, Serialize};
use crate::game::deck_manager::{Card, DeckManager};
use crate::player::Player;
use crate::utils::{gen_special_id};


#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum GameLoopState {
    Start, Pause, Finish, BeforeTurn, Turn
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Game {
    id: String,
    status: GameLoopState,
    participant_ids: Vec<String>,
    target_player_id: Option<String>,
    next_player_id: Option<String>,
    turn_queue: Vec<String>,
    pub deck_manager: DeckManager
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PopulatedGame {
    id: String,
    status: GameLoopState,
    participant_ids: Vec<Option<Player>>,
    target_player_id: Option<Player>,
    next_player_id: Option<Player>,
    turn_queue: Vec<Option<Player>>,
    pub deck_manager: DeckManager
}

impl Game {
    pub fn new(players: Vec<String>) -> Game {
        Game {
            id: gen_special_id("GAME"),
            status: GameLoopState::Start,
            participant_ids: players,
            target_player_id: None,
            next_player_id: None,
            turn_queue: vec![],
            deck_manager: DeckManager::new()
        }
    }

    pub fn init_table(&mut self, card: Card) -> Result<(), ()> {
        Ok(())
    }

    pub fn beat(&mut self, beating: Card, beatable: Card) -> Result<(), ()> {
        Ok(())
    }

    pub fn toss(&mut self, attacker_id: &str, card: Card) -> Result<(), ()> {
        Ok(())
    }

    fn can_init_table(&self, player_id: &str) -> bool {
        if self.status != GameLoopState::BeforeTurn {
            return false;
        }
        if let Some(next_player) = &self.next_player_id {
            player_id == next_player
        } else {
            false
        }
    }

    fn can_toss(&self, player_id: &str) -> bool {
        if self.status != GameLoopState::Turn {
            return false;
        }
        if let Some(target) = &self.target_player_id {
            player_id != target
        } else {
            false
        }
    }

    fn can_beat(&self, player_id: &str) -> bool {
        if self.status != GameLoopState::Turn {
            return false;
        }
        if let Some(target) = &self.target_player_id {
            player_id == target
        } else {
            false
        }
    }

    fn can_take_table(&self, player_id: &str) -> bool {
        if self.status != GameLoopState::Turn {
            return false;
        }
        if let Some(target) = &self.target_player_id {
            player_id == target
        } else {
            false
        }
    }

    fn can_discard_table(&self, player_id: &str) -> bool {
        if self.status != GameLoopState::Turn {
            return false;
        }
        if let Some(target) = &self.target_player_id {
            player_id == target && self.deck_manager.is_table_beaten()
        } else {
            false
        }
    }
}
