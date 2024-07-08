use std::ops::Deref;
use serde::{Deserialize, Serialize};
use crate::player::Player;
use crate::game::Game;
use crate::utils::{gen_special_id};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum LobbyStatus {
    INACTIVE, ACTIVE, STARTED
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Lobby {
    id: String,
    status: LobbyStatus,
    public: bool,
    player_list: Vec<String>,
    game: Option<Game>
}

impl Lobby {
    pub fn new(is_public: bool) -> Lobby {
        Lobby {
            id: gen_special_id("LOBBY"),
            status: LobbyStatus::INACTIVE,
            public: is_public,
            player_list: vec![],
            game: None
        }
    }

    pub fn new_private() -> Lobby {
        Lobby {
            id: gen_special_id("LOBBY"),
            status: LobbyStatus::INACTIVE,
            public: false,
            player_list: vec![],
            game: None
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn player_add(&mut self, player: Player) {
        self.player_list.push(player.get_id().to_string());
    }

    pub fn player_remove(&mut self, id: String) {
        let index = self.player_list
            .iter()
            .position(|item| item.deref() == id)
            .unwrap();
        self.player_list.remove(index);
    }

    pub fn init_game(&mut self) {
        self.game = Some(Game::new(self.player_list.clone()));
    }
}
