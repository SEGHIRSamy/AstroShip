use crate::classes::gestion_evenement::boucleJeu::BoucleJeu;
use crate::classes::gestion_evenement::evenement::Evenement;

#[allow(dead_code)]
pub struct LancerPartie {}
#[allow(dead_code)]
impl LancerPartie {
    pub fn new() -> LancerPartie { LancerPartie {}}

    pub fn lancer_partie(&self) {
        let mut partie = BoucleJeu::new(true);
        partie.boucleJeu();
    }
}

impl Evenement for LancerPartie {
    fn action(&self) {
        self.lancer_partie();
    }
}