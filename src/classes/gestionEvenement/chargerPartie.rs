use crate::classes::affichage::affiche_texte::AfficheTexte;
use crate::classes::gestionEvenement::evenement::Evenement;

pub struct chargerPartie {}

impl chargerPartie {
    pub fn new() -> chargerPartie {chargerPartie {}}

    pub fn chargerPartie(&self) {
        AfficheTexte::affiche("Vous chargez la partie !".to_string(), 30);
    }
}

impl Evenement for chargerPartie {
    fn action(&self) {
        self.chargerPartie();
    }
}