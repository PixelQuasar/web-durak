use rand::Rng;

pub fn gen_special_id(prefix: &str) -> String {
    let mut rng = rand::thread_rng();
    format!("{}{}", prefix, rng.gen::<u32>())
}
