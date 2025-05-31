#[derive(Clone)]
pub struct VoyagePlanete {
    pub nom: String,
    pub cout_voyage: u32,
}

#[allow(dead_code)]
impl VoyagePlanete {
    pub fn new(nom: &str, cout_voyage: u32) -> Self {


        Self {
            nom: nom.to_string(),
            cout_voyage
        }
    }
}
