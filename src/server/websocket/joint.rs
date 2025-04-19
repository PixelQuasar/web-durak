use injoint::codegen::{reducer_actions, Broadcastable};
use serde::Serialize;
use injoint::joint::axum::AxumWSJoint;
use crate::game::deck_manager::{Card};
use crate::game::{Game};

#[derive(Serialize, Debug, Default, Clone, Broadcastable)]
struct State {
    pub game: Game
}

#[derive(Default, Serialize, Clone, Broadcastable)]
struct Reducer {
    state: State,
}

impl Reducer {
    pub fn new() -> Self {
        Reducer {
            state: State {
                game: Game::new(vec![]),
            },
        }
    }
}

#[reducer_actions(State)]
impl Reducer {
    pub async fn game_create(&mut self, client_id: u64, players: String)-> Result<String, String>  {
        let player_ids: Vec<String> = players.split(",").map(|item| item.to_string()).collect();
        self.state.game = Game::new(player_ids);
        Ok("game created".to_string())
    }

    pub async fn turn_init_table(&mut self, client_id: u64, card: Card)-> Result<String, String>  {
        self.state.game.init_table(&client_id.to_string(), card.clone())
        .map_err(|_| "game machine error")?;

        Ok("success".to_string())
    }

    pub async fn turn_beat(&mut self, client_id: u64, beating: Card, beatable: Card)-> Result<String, String>  {
        let table_element_id = self.state.game
            .beat(&client_id.to_string(), beating, beatable)
            .map_err(|_| "beating error")?;

        Ok(table_element_id.to_string())
    }

    pub async fn turn_toss(&mut self, client_id: u64, card: Card)-> Result<String, String>  {
        let table_element_id = self.state.game.toss(&client_id.to_string(), card)
        .map_err(|_| "game machine error")?;

        Ok(table_element_id.to_string())
    }

    pub async fn turn_transfer(&mut self, client_id: u64, card: Card)-> Result<String, String>  {
        self.state.game.transfer(&client_id.to_string(), card)
        .map_err(|_| "game machine error")?;

        Ok(self.state.game.table_size().to_string())
    }

    pub async fn turn_discard(&mut self, client_id: u64)-> Result<String, String>  {
        let table_size = self.state.game.deck_manager.get_table_size();

        self.state.game.finish_with_discard()
            .map_err(|_| "game machine error")?;

        Ok(format!("{} discard", client_id).to_string())
    }

    pub async fn turn_take(&mut self, client_id: u64)-> Result<String, String>  {
        let table_size = self.state.game.deck_manager.get_table_size();

        self.state.game.finish_with_take()
            .map_err(|_| "game machine error")?;

        Ok(format!("{} take", client_id).to_string())
    }

    pub async fn turn_confirm_beat(&mut self, client_id: u64)-> Result<String, String>  {
        self.state.game.confirm_beat(&client_id.to_string())
            .map_err(|_| "game machine error")?;

        Ok(format!("{} confirm", client_id).to_string())
    }
}

pub async fn build_joint() -> AxumWSJoint<Reducer> {
    let reducer = Reducer::new();

    AxumWSJoint::<Reducer>::new(reducer)
}
