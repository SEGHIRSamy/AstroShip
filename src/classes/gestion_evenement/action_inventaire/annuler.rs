use std::rc::Rc;
use std::cell::RefCell;

use crate::classes::gestion_evenement::evenement::Evenement;

pub struct Annuler {
    consomer: Rc<RefCell<bool>>,
}

impl Annuler {
    pub fn new(consomer: Rc<RefCell<bool>>) -> Self {
        Self { consomer }
    }

    pub fn annuler(&self) {
        println!("Retour à l'inventaire.");
        *self.consomer.borrow_mut() = false;
    }
}

impl Evenement for Annuler {
    fn action(&mut self) {
        self.annuler();
    }
}
