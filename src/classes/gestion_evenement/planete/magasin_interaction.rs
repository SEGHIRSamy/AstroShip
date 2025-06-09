use std::cell::RefCell;
use std::rc::Rc;

use crate::classes::entite::personnage_principal::PersonnagePrincipal;
use crate::classes::gestion_evenement::evenement::Evenement;
use crate::classes::planete::magasin::Magasin;

pub struct MagasinInteraction<'a> {
    magasin: &'a mut Magasin,
    personnage: Rc<RefCell<PersonnagePrincipal>>
}

impl<'a> MagasinInteraction<'a> {
    pub fn new(magasin: &'a mut Magasin, personnage: Rc<RefCell<PersonnagePrincipal>>) -> Self { Self {magasin, personnage}}
}

impl<'a> Evenement for MagasinInteraction<'a> {
    fn action(&mut self) {
        let mut personnage = self.personnage.borrow_mut();
        self.magasin.interaction_magasin(&mut personnage);
    }
}