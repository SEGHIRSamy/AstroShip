use std::rc::Rc;
use std::cell::RefCell;

use crate::classes::gestion_evenement::evenement::Evenement;
use crate::classes::marchandage::objet::Objet;

pub struct Consommer {
    consomer: Rc<RefCell<bool>>,
    consommable: bool,
    index: usize,
    instance_rc: Rc<RefCell<Vec<Objet>>>
}

impl Consommer {
    pub fn new(consomer: Rc<RefCell<bool>>, consommable: bool, index: usize, instance_rc: Rc<RefCell<Vec<Objet>>> ) -> Self {
        Self { consomer, consommable, index, instance_rc }
    }

    pub fn consomer(&self) {
        if !self.consommable {
            println!("Cet objet ne peut pas être consommé.");
            *self.consomer.borrow_mut() = false; 
            return;
        }

        let mut instance = self.instance_rc.borrow_mut();

        let objet = &mut instance[self.index - 1];
        let nom_objet = objet.get_nom().to_string();
        objet.consommer_perso_principal(&nom_objet);
        println!("Objet consommé !");

        if objet.get_quantite() > 1 {
            objet.set_quantite(objet.get_quantite() - 1);
        } else {
            instance.remove(self.index - 1);
            println!("Objet supprimé.");
        }

        *self.consomer.borrow_mut() = true;
    }
}

impl Evenement for Consommer {
    fn action(&mut self) {
        self.consomer();
    }
}