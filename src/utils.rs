use rand::Rng;

pub fn generate_id() -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen::<u64>()
}
