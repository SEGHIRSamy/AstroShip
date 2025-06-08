use std::rc::Rc;
use std::cell::RefCell;

use crate::classes::affichage::affichage_deplacement::AffichageDeplacement;
use crate::classes::gestion_evenement::evenement::Evenement;

pub struct StopChoix {
    nom: String,
    phrase_arrive: Vec<String>,
    stop: Rc<RefCell<bool>>,
}

impl StopChoix {
    pub fn new(nom: String, phrase_arrive: Vec<String>, stop: Rc<RefCell<bool>>) -> Self {
        Self { nom, phrase_arrive, stop }
    }

    pub fn stop_choix(&self) {
        println!("Vous quittez la planète {}.", self.nom);
        AffichageDeplacement::lancer_animation_spatiale("depart", self.phrase_arrive.clone());
        *self.stop.borrow_mut() = true;
    }
}

impl Evenement for StopChoix {
    fn action(&mut self) {
        self.stop_choix();
    }
}
