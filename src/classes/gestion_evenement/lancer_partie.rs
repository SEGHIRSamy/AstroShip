use crate::classes::gestion_evenement::boucle_jeu::BoucleJeu;
use crate::classes::gestion_evenement::evenement::Evenement;

#[allow(dead_code)]
pub struct LancerPartie {}
#[allow(dead_code)]
impl LancerPartie {
    pub fn new() -> LancerPartie { LancerPartie {}}

    pub fn lancer_partie(&self) {
        let mut partie = BoucleJeu::new(true);
        partie.boucle_jeu();
    }
}

impl Evenement for LancerPartie {
    fn action(&mut self) {
        self.lancer_partie();
    }
}