use crate::player::Player;
use std::cell::RefCell;
use crate::utils::generate_id::generate_id;

pub enum GameLoopState {
    START, PAUSE, FINISH, TURN,
}

#[derive(Debug)]
pub struct Game {
    id: u64,
    participants: Vec<RefCell<Player>>,
    target_player: Option<RefCell<Player>>,
    turn_queue: Vec<RefCell<Player>>
}

impl Game {
    pub fn new() -> Game {
        Game {
            id: generate_id::<u64>(),
            participants: vec![],
            target_player: None,
            turn_queue: vec![]
        }
    }
}
