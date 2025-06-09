use std::cell::RefCell;
use std::rc::Rc;

use crate::classes::entite::personnage_principal::PersonnagePrincipal;
use crate::classes::gestion_evenement::evenement::Evenement;
use crate::classes::planete::zone_hostile::ZoneHostile;
use crate::classes::sauvegarde::sauvegarde::Sauvegarde;

pub struct ExplorerZoneHostile<'a> {
    zone_hostile: &'a mut ZoneHostile,
    personnage: Rc<RefCell<PersonnagePrincipal>>
}

impl<'a> ExplorerZoneHostile<'a> {
    pub fn new(zone_hostile: &'a mut ZoneHostile, personnage: Rc<RefCell<PersonnagePrincipal>>) -> Self { Self {zone_hostile, personnage}}
}

impl<'a> Evenement for ExplorerZoneHostile<'a> {
    fn action(&mut self) {
        let sauvegarde = Sauvegarde::new();
        self.zone_hostile.explorer();
        let tmp_joueur : PersonnagePrincipal = sauvegarde.charge("personnage_principal.json".to_string()).unwrap();
        self.personnage.borrow_mut().inventaire.set_monnaie(tmp_joueur.inventaire.get_monnaie());
    }
}