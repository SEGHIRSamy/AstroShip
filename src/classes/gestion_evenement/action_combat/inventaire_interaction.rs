use std::cell::RefCell;
use std::rc::Rc;

use crate::classes::entite::personnage_principal::PersonnagePrincipal;
use crate::classes::gestion_evenement::evenement::Evenement;
use crate::classes::sauvegarde::sauvegarde::Sauvegarde;

pub struct InventaireInteraction {
    consome: Rc<RefCell<bool>>,
    charge_player_rc: Rc<RefCell<PersonnagePrincipal>>,
}

impl InventaireInteraction {
    pub fn new(consome: Rc<RefCell<bool>>, charge_player_rc: Rc<RefCell<PersonnagePrincipal>>) -> Self {
        Self { consome, charge_player_rc }
    }

    pub fn interaction(&mut self) {
        let mut charge_player = self.charge_player_rc.borrow_mut();
        let a_consomme = charge_player.inventaire.afficher_inventaire_interactif();
        // Si on a rien consommé dans l'inventaire on repart dans la boucle sans sauvegarder les stats dans le json
        *self.consome.borrow_mut() = a_consomme;
        if !a_consomme {
            return
        }
        // Si on a consommé alors on save les stats du perso et son inventaire
        let sauvegarde = Sauvegarde::new();
        *charge_player = sauvegarde.charge("personnage_principal.json".to_string()).unwrap();
    }
}

impl Evenement for InventaireInteraction {
    fn action(&mut self) {
        self.interaction();
    }
}


