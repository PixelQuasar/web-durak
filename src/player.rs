use crate::utils::gen_special_id;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    id: String,
    name: String,
    active: bool,
    ready: bool,
}

impl Player {
    pub fn new(player_name: String) -> Player {
        dotenv::dotenv().ok();
        Self::new_with_id(
            gen_special_id(&dotenv::var("PREFIX_PLAYER").unwrap()),
            player_name,
        )
    }

    pub fn new_with_id(new_id: String, player_name: String) -> Player {
        Player {
            id: new_id,
            name: player_name.parse().unwrap(),
            active: false,
            ready: false,
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }
}
