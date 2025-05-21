use crate::classes::affichage::affiche_texte::AfficheTexte;
use crate::classes::gestionEvenement::evenement::Evenement;

pub struct quitterJeu {}

impl quitterJeu {
    pub fn new() -> quitterJeu {quitterJeu {}}

    pub fn quitterJeu(&self) {
        AfficheTexte::affiche("Merci d'avoir joué, à bientot !".to_string(), 30);
        std::process::exit(0);
    }
}

impl Evenement for quitterJeu {
    fn action(&self) {
        self.quitterJeu();
    }
}