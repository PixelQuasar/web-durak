use crate::game::Game;
use crate::player::Player;
use crate::utils::gen_special_id;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum LobbyStatus {
    INACTIVE,
    ACTIVE,
    STARTED,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Lobby {
    id: String,
    status: LobbyStatus,
    owner_id: String,
    public: bool,
    player_list: Vec<String>,
    max_players: usize,
    pub game: Option<Game>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PopulatedLobby {
    id: String,
    status: LobbyStatus,
    owner_id: Player,
    public: bool,
    player_list: Vec<Player>,
    max_players: usize,
    pub game: Option<Game>,
}

impl Lobby {
    pub fn new(is_public: bool) -> Lobby {
        dotenv::dotenv().ok();
        Lobby {
            id: gen_special_id(&dotenv::var("PREFIX_LOBBY").unwrap()),
            status: LobbyStatus::ACTIVE,
            public: is_public,
            owner_id: String::new(),
            player_list: vec![],
            max_players: 6,
            game: None,
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn player_list(&self) -> &Vec<String> {
        &self.player_list
    }

    pub fn player_add(&mut self, player_id: &str) {
        if self.player_list.len() == 0 {
            self.owner_id = player_id.to_string();
        }
        self.player_list.push(player_id.to_string());
    }

    pub fn player_remove(&mut self, id: &str) {
        let index = self.player_list.iter().position(|item| item == id).unwrap();
        self.player_list.remove(index);
    }

    pub fn init_game(&mut self, player_ids: Vec<String>) {
        let mut game = Game::new(self.player_list.clone());

        game.deck_manager.deal_six(player_ids);

        let first_target_player = game.deck_manager.get_first_target_player();

        game.set_target_player_id(first_target_player.clone());

        game.set_next_player_id(
            game.deck_manager
                .player_after(&first_target_player)
                .unwrap(),
        );

        game.set_attacker_player_id(game.deck_manager.get_first_attacker_player());

        game.start();

        self.status = LobbyStatus::STARTED;

        self.game = Some(game);
    }

    pub fn finish_game(&mut self) {
        self.status = LobbyStatus::ACTIVE;

        self.game = None;
    }

    pub fn players_num(&self) -> usize {
        return self.player_list.len();
    }

    pub fn can_join(&self) -> bool {
        self.status == LobbyStatus::STARTED || self.status == LobbyStatus::ACTIVE
    }

    pub fn get_owner_id(&self) -> &str {
        &self.owner_id
    }
}

impl PopulatedLobby {
    pub fn from_lobby(lobby: Lobby, players: Vec<Player>, owner: Player) -> PopulatedLobby {
        PopulatedLobby {
            id: lobby.id,
            status: lobby.status,
            public: lobby.public,
            owner_id: owner,
            player_list: players,
            max_players: 6,
            game: lobby.game,
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn players_num(&self) -> usize {
        self.player_list.len()
    }

    pub fn player_list(&self) -> Vec<Player> {
        self.player_list.clone()
    }

    pub fn can_join(&self) -> bool {
        self.status == LobbyStatus::STARTED || self.status == LobbyStatus::ACTIVE
    }

    pub fn max_capacity(&self) -> usize {
        6
    }
}
