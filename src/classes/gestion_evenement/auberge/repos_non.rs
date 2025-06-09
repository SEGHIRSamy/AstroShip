use crate::classes::affichage::affiche_texte::AfficheTexte;
use crate::classes::gestion_evenement::evenement::Evenement;

pub struct ReposNon {

}

impl ReposNon {
    pub fn new() -> Self {
        Self { }
    }

    pub fn refuser(&self) {
        AfficheTexte::affiche("Très bien, peut-être une autre fois.".to_string(), 50);
    }
}

impl Evenement for ReposNon {
    fn action(&mut self) {
        self.refuser();
    }
}
