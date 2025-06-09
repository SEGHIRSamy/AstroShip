use crate::classes::gestion_evenement::evenement::Evenement;

pub struct Continuer {
}

impl Continuer {
    pub fn new() -> Self {
        Self { }
    }

    pub fn continuer(&self) {
    }
}

impl Evenement for Continuer {
    fn action(&mut self) {
        self.continuer();
    }
}
