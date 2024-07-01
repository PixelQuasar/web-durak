use crate::utils::generate_id::generate_id;

#[derive(Debug, Clone)]
pub struct Player {
    id: u64,
    name: String,
    active: bool,
    ready: bool
}

impl Player {
    pub fn new(player_name: &str) -> Player {
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