use rand::Rng;
use crate::game::deck_manager::CardIdType;

pub fn generate_id() -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen::<u64>()
}

pub fn generate_card_id() -> CardIdType {
    let mut rng = rand::thread_rng();
    rng.gen::<CardIdType>()
}

pub fn gen_special_id(prefix: &str) -> String {
    let mut rng = rand::thread_rng();
    format!("{}{}", prefix, rng.gen::<u32>())
}
