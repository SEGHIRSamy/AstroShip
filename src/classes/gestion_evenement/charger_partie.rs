use crate::classes::gestion_evenement::evenement::Evenement;
use crate::classes::gestion_evenement::boucle_jeu::BoucleJeu;

pub struct ChargerPartie {}

impl ChargerPartie {
    pub fn new() -> ChargerPartie { ChargerPartie {}}

    pub fn charger_partie(&self) {
        let mut partie = BoucleJeu::new(false);
        partie.boucle_jeu();
    }
}

impl Evenement for ChargerPartie {
    fn action(&self) {
        self.charger_partie();
    }
}