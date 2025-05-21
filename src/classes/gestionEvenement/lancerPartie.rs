use crate::classes::affichage::affiche_texte::AfficheTexte;
use crate::classes::gestionEvenement::evenement::Evenement;

pub struct lancerPartie {}

impl lancerPartie {
    pub fn new() -> lancerPartie {lancerPartie {}}

    pub fn lancerPartie(&self) {
        AfficheTexte::affiche("Vous lancez la partie !".to_string(), 30);
    }
}

impl Evenement for lancerPartie {
    fn action(&self) {
        self.lancerPartie();
    }
}