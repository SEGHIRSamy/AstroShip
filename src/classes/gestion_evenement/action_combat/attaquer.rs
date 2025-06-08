use std::cell::RefCell;
use std::rc::Rc;

use crate::classes::affichage::affiche_texte::AfficheTexte;
use crate::classes::gestion_evenement::combat::Combat;
use crate::classes::gestion_evenement::evenement::Evenement;
use crate::classes::gestion_evenement::lancer_dice::LancerDice;

pub struct Attaquer {
    pv_ennemi_rc: Rc<RefCell<u32>>,
    attaque_joueur: u32,
    attaque_ennemi: u32,
}

impl Attaquer {
    pub fn new(pv_ennemi_rc: Rc<RefCell<u32>>, attaque_joueur: u32, attaque_ennemi: u32) -> Self {
        Self { pv_ennemi_rc, attaque_joueur, attaque_ennemi }
    }

    pub fn attaquer(&mut self) {
        let pv_ennemi: u32 = *self.pv_ennemi_rc.borrow();
        let lancer = LancerDice::lancer_console_combat(true);
        let degats =
            Combat::calculer_degats(self.attaque_joueur, self.attaque_ennemi, lancer);
        // On met a jour els pv de l'ennemi apres qu'on l'avoir attaqué
        let nouvelle_vie = if degats > pv_ennemi {
            0
        } else {
            pv_ennemi - degats
        };
        //ennemi.base.set_points_de_vie(nouvelle_vie);
        *self.pv_ennemi_rc.borrow_mut() = nouvelle_vie;
        AfficheTexte::affiche(
            format!("Vous infligez {} dégâts. PV Ennemi restants : {}", degats, nouvelle_vie),
            15,
        );
    }
}

impl Evenement for Attaquer {
    fn action(&mut self) {
        self.attaquer();
    }
}
