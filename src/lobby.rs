use crate::utils::generate_id;
use crate::player::Player;
use crate::utils::generate_id::generate_id;

#[derive(Debug)]
pub enum LobbyStatus {
    INACTIVE, ACTIVE, STARTED
}

#[derive(Debug)]
pub struct Lobby {
    id: u64,
    status: LobbyStatus,
    public: bool,
    player_list: Vec<Player>
}

impl Lobby {
    pub fn new() -> Lobby {
        Lobby {
            id: generate_id::<u64>(),
            status: LobbyStatus::INACTIVE,
            public: false,
            player_list: vec![]
        }
    }
}
