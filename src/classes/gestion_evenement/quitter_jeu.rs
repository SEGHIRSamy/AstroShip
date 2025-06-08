use crate::classes::affichage::affiche_texte::AfficheTexte;
use crate::classes::gestion_evenement::evenement::Evenement;

pub struct QuitterJeu {}

impl QuitterJeu {
    pub fn new() -> QuitterJeu { QuitterJeu {}}

    pub fn quitter_jeu(&self) {
        AfficheTexte::affiche("Merci d'avoir joué, à bientot !".to_string(), 30);
        std::process::exit(0);
    }
}

impl Evenement for QuitterJeu {
    fn action(&mut self) {
        self.quitter_jeu();
    }
}