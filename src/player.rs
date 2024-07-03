use serde::{Deserialize, Serialize};
use crate::utils::generate_id;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    id: u64,
    name: String,
    active: bool,
    ready: bool
}

impl Player {
    pub fn new(player_name: String) -> Player {
        Player {
            id: generate_id(),
            name: player_name.parse().unwrap(),
            active: false,
            ready: false
        }
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }
}
