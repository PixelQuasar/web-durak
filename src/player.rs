use crate::utils::generate_id::generate_id;

#[derive(Debug)]
pub struct Player {
    id: u64,
    name: String,
    active: bool,
    ready: bool
}

impl Player {
    pub fn new(player_name: String) -> Player {
        Player {
            id: generate_id::<u64>(),
            name: player_name,
            active: false,
            ready: false
        }
    }
}
