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
    attacker_player_id: Option<String>,
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
            status: GameLoopState::BeforeTurn,
            participant_ids: players,
            attacker_player_id: None,
            target_player_id: None,
            next_player_id: None,
            turn_queue: vec![],
            deck_manager: DeckManager::new()
        }
    }

    pub fn init_table(&mut self, player_id: &str, card: Card) -> Result<(), ()> {
        if self.can_init_table(player_id) {
            self.status = GameLoopState::Turn;

            self.deck_manager.init_table(player_id, card)?;

            Ok(())
        } else {
            Err(())
        }

    }

    pub fn beat(&mut self, player_id: &str, beating: Card, beatable: Card) -> Result<i32, ()> {
        if self.target_player_id.is_none() {
            return Err(())
        }

        if self.can_beat(player_id) {
            let defender = &self.target_player_id.clone().unwrap();

            Ok(self.deck_manager.beat(defender, beating, beatable)?)
        } else {
            Err(())
        }
    }

    pub fn confirm_beat(&mut self, player_id: &str) -> Result<(), ()> {
        if self.target_player_id.is_none() {
            return Err(())
        }

        if self.can_confirm_beat(player_id) {
            self.deck_manager.confirm_beat(player_id.to_string())?;

            Ok(())
        } else {
            Err(())
        }
    }

    pub fn is_all_confirmed(&self) -> bool {
        self.deck_manager.is_all_confirmed(self.target_player_id.clone().unwrap().as_str())
    }

    pub fn toss(&mut self, attacker_id: &str, card: Card) -> Result<i32, ()> {
        if (self.can_toss(attacker_id)) {
            Ok(self.deck_manager.toss(attacker_id, card)?)
        } else {
            Err(())
        }
    }

    pub fn finish_with_take(&mut self) -> Result<(), ()> {
        if self.target_player_id.is_none() {
            return Err(())
        }

        if self.can_take_table(&self.target_player_id.clone().unwrap()) {
            let defender = &self.target_player_id.clone().unwrap();

            self.status = GameLoopState::BeforeTurn;

            self.deck_manager.take_table(defender)?;

            let next_attacker = self.deck_manager.player_after(defender).unwrap();

            self.target_player_id = Some(self.deck_manager.player_after(&next_attacker).unwrap());

            self.attacker_player_id = Some(next_attacker);

            Ok(())
        } else {
            Err(())
        }
    }

    pub fn finish_with_discard(&mut self) -> Result<(), ()> {
        if self.target_player_id.is_none() {
            return Err(())
        }

        if self.can_discard_table(&self.target_player_id.clone().unwrap()) {
            let defender = &self.target_player_id.clone().unwrap();

            self.deck_manager.discard_table()?;

            self.status = GameLoopState::BeforeTurn;

            self.target_player_id = Some(self.deck_manager.player_after(&defender).unwrap());

            self.attacker_player_id = Some(defender.to_string());

            Ok(())
        } else {
            Err(())
        }
    }

    pub fn deal_more(&mut self, defending_player_id: &str) -> Result<Vec<(String, Vec<Card>)>, ()> {
        self.deck_manager.deal_more(defending_player_id)
    }

    pub fn target_player_id(&self) -> Option<String> {
        self.target_player_id.clone()
    }

    pub fn set_target_player_id(&mut self, player_id: String) {
        self.target_player_id = Some(player_id);
    }

    pub fn set_next_player_id(&mut self, player_id: String) {
        self.next_player_id = Some(player_id);
    }

    pub fn set_attacker_player_id(&mut self, player_id: String) {
        self.attacker_player_id = Some(player_id);
    }

    pub fn start(&mut self) {
        self.status = GameLoopState::BeforeTurn;
    }

    fn can_init_table(&self, player_id: &str) -> bool {
        if self.status != GameLoopState::BeforeTurn {
            return false;
        }
        if let Some(attacker) = &self.attacker_player_id {
            player_id == attacker
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

    fn can_confirm_beat(&self, player_id: &str) -> bool {
        if self.status != GameLoopState::Turn {
            return false;
        }
        if let Some(target) = &self.target_player_id {
            player_id != target
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
            player_id == target
                && self.deck_manager.is_table_beaten()
                && self.deck_manager.can_discard(target)
        } else {
            false
        }
    }
}
