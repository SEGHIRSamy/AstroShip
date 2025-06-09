use std::cell::RefCell;
use std::rc::Rc;

use rand::rng;

use crate::classes::affichage::affiche_texte::AfficheTexte;
use crate::classes::entite::ennemie::Ennemi;
use crate::classes::entite::personnage_principal::PersonnagePrincipal;
use crate::classes::gestion_evenement::combat::Combat;
use crate::classes::gestion_evenement::evenement::Evenement;
use crate::classes::gestion_evenement::lancer_dice::LancerDice;
use crate::classes::sauvegarde::sauvegarde::Sauvegarde;

pub struct Fuir {
    stop: Rc<RefCell<bool>>,
    charge_player_rc: Rc<RefCell<PersonnagePrincipal>>,
    attaque_joueur: u32,
    vitesse_joueur: u32,
    pv_max: u32,
    pv_joueur: u32,
    ennemi: Ennemi,
    vitesse_ennemi: u32,
}

impl Fuir {
    pub fn new(stop: Rc<RefCell<bool>>, charge_player_rc: Rc<RefCell<PersonnagePrincipal>>, attaque_joueur: u32, vitesse_joueur: u32, pv_max: u32, pv_joueur: u32, ennemi: Ennemi, vitesse_ennemi: u32) -> Self {
        Self { stop, charge_player_rc, attaque_joueur, vitesse_joueur, pv_max, pv_joueur, ennemi, vitesse_ennemi }
    }

    pub fn fuir(&mut self) {
        let lancer = LancerDice::lancer_console_combat(true);
        let mut charge_player = self.charge_player_rc.borrow_mut();

        if Combat::tenter_fuite(self.vitesse_joueur, self.vitesse_ennemi, lancer) {
            AfficheTexte::affiche("✅ Vous avez réussi à fuir !".to_string(), 20);
            // On arrive a fuir : le combat est fini donc on save les stats et l'inventaire du perso dans le json
            let mut update_player = PersonnagePrincipal::new(
                charge_player.entite.get_nom(),
                self.pv_joueur,
                self.pv_max,
                self.attaque_joueur,
                charge_player.entite.get_intelligence(),
                self.vitesse_joueur,
                charge_player.chance,
                charge_player.get_uranium(),
                charge_player.get_carburant(),
                charge_player.get_planete_nom().to_string(),
            );

            let mut rng = rng();
            for obj in self.ennemi.interaction(&mut rng) {
                charge_player.inventaire.add_objet(obj.get_objet());
                obj.get_objet().afficher()
            }
            update_player.inventaire.add_monnaie(charge_player.inventaire.get_monnaie()+self.ennemi.get_monnaie().clone());
            update_player.inventaire.set_instance(charge_player.inventaire.get_instance().clone());

            let sauvegarde = Sauvegarde::new();
            sauvegarde.sauvegarde("personnage_principal.json".to_string(), update_player.clone()).expect("Enregistrement Personnage échoué");
            *self.stop.borrow_mut() = true;

        } else {
            AfficheTexte::affiche("❌ Vous n'avez pas réussi à fuir.".to_string(), 20);
        }
    }
}

impl Evenement for Fuir {
    fn action(&mut self) {
        self.fuir();
    }
}


