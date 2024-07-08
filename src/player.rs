use serde::{Deserialize, Serialize};
use crate::utils::{gen_special_id};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    id: String,
    name: String,
    active: bool,
    ready: bool
}

impl Player {
    pub fn new(player_name: String) -> Player {
        Player {
            id: gen_special_id("PLAYER"),
            name: player_name.parse().unwrap(),
            active: false,
            ready: false
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }
}
