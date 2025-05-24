use crate::classes::affichage::affiche_texte::AfficheTexte;
use crate::classes::gestion_evenement::evenement::Evenement;

#[allow(dead_code)]
pub struct LancerPartie {}
#[allow(dead_code)]
impl LancerPartie {
    pub fn new() -> LancerPartie { LancerPartie {}}

    pub fn lancer_partie(&self) {
        AfficheTexte::affiche("Vous lancez la partie !".to_string(), 30);
    }
}

impl Evenement for LancerPartie {
    fn action(&self) {
        self.lancer_partie();
    }
}