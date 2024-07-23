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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PopulatedLobby {
    id: String,
    status: LobbyStatus,
    public: bool,
    player_list: Vec<Option<Player>>,
    game: Option<Game>
}

impl Lobby {
    pub fn new(is_public: bool) -> Lobby {
        dotenv::dotenv().ok();
        Lobby {
            id: gen_special_id(&dotenv::var("PREFIX_LOBBY").unwrap()),
            status: LobbyStatus::INACTIVE,
            public: is_public,
            player_list: vec![],
            game: None
        }
    }

    pub fn new_private() -> Lobby {
        dotenv::dotenv().ok();
        Lobby::new(false)
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn status(&self) -> &LobbyStatus {
        &self.status
    }

    pub fn is_public(&self) -> &bool {
        &self.public
    }

    pub fn player_list(&self) -> &Vec<String> {
        &self.player_list
    }

    pub fn game(&self) -> &Option<Game> {
        &self.game
    }

    pub fn player_add(&mut self, player_id: &str) {
        self.player_list.push(player_id.to_string());
    }

    pub fn player_remove(&mut self, id: &str) {
        let index = self.player_list
            .iter()
            .position(|item| item == id)
            .unwrap();
        self.player_list.remove(index);
    }

    pub fn init_game(&mut self) {
        self.game = Some(Game::new(self.player_list.clone()));
    }

    pub fn players_num(&self) -> usize {
        return self.player_list.len()
    }
}

impl PopulatedLobby {
    pub fn from_lobby(lobby: Lobby, players: Vec<Option<Player>>) -> PopulatedLobby {
        PopulatedLobby {
            id: lobby.id,
            status: lobby.status,
            public: lobby.public,
            player_list: players,
            game: lobby.game
        }
    }
}
