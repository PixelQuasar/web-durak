use serde::{Deserialize, Serialize};
use crate::player::Player;
use crate::game::Game;
use crate::utils::generate_id;


#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum LobbyStatus {
    INACTIVE, ACTIVE, STARTED
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Lobby {
    id: u64,
    status: LobbyStatus,
    public: bool,
    player_list: Vec<u64>,
    game: Option<Game>
}

impl Lobby {
    pub fn new(is_public: bool) -> Lobby {
        Lobby {
            id: generate_id(),
            status: LobbyStatus::INACTIVE,
            public: is_public,
            player_list: vec![],
            game: None
        }
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn player_add(&mut self, player: Player) {
        self.player_list.push(player.get_id());
    }

    pub fn player_remove(&mut self, id: u64) {
        let index = self.player_list
            .iter()
            .position(|item| *item == id)
            .unwrap();
        self.player_list.remove(index);
    }

    pub fn init_game(&mut self) {
        self.game = Some(Game::new(self.player_list.clone()));
    }
}
