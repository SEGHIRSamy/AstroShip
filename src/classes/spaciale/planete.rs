pub struct Planete {
    pub nom: String,
    pub cout_voyage: i32,
}

impl Planete {
    pub fn new(nom: &str, cout_voyage: i32) -> Self {
        Self {
            nom: nom.to_string(),
            cout_voyage,
        }
    }
}
