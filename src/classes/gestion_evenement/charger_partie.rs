use crate::classes::affichage::affiche_texte::AfficheTexte;
use crate::classes::gestion_evenement::evenement::Evenement;

pub struct ChargerPartie {}

impl ChargerPartie {
    pub fn new() -> ChargerPartie { ChargerPartie {}}

    pub fn charger_partie(&self) {
        AfficheTexte::affiche("Vous chargez la partie !".to_string(), 30);
    }
}

impl Evenement for ChargerPartie {
    fn action(&self) {
        self.charger_partie();
    }
}