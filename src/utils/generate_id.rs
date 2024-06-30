use rand::Rng;

pub fn generate_id<T>() -> T {
    let mut rng = rand::thread_rng();
    rng.gen::<T>()
}
