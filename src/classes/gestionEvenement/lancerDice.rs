use rand::Rng;

pub struct LancerDice;

impl LancerDice {
    pub fn lancer() -> u32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(1..=20)
    }
}