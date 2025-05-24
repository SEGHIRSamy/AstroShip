use rand::Rng;
#[allow(dead_code)]
pub struct LancerDice;
#[allow(dead_code)]
impl LancerDice {
    pub fn lancer() -> u32 {
        let mut rng = rand::rng();
        rng.random_range(1..=20)
    }
}