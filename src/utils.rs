use rand::Rng;

pub fn generate_id() -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen::<u64>()
}

pub fn gen_special_id(prefix: &str) -> String {
    let mut rng = rand::thread_rng();
    format!("{}{}", prefix, rng.gen::<u32>())
}
