use rand::Rng;

pub fn roll(d: u8) -> u8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..d + 1)
}