use std::rc::Rc;
use std::cell::RefCell;

use crate::classes::affichage::affiche_texte::AfficheTexte;
use crate::classes::gestion_evenement::evenement::Evenement;

pub struct StopExplorer {
    stop: Rc<RefCell<bool>>,
}

impl StopExplorer {
    pub fn new(stop: Rc<RefCell<bool>>) -> Self {
        Self { stop }
    }

    pub fn stop_choix(&self) {
        AfficheTexte::affiche("Vous quittez la zone hostile.".to_string(), 10);
        *self.stop.borrow_mut() = true;
    }
}

impl Evenement for StopExplorer {
    fn action(&mut self) {
        self.stop_choix();
    }
}
