use std::cell::RefCell;
use std::rc::Rc;
use std::thread;

use crate::classes::affichage::affiche_texte::AfficheTexte;
use crate::classes::entite::personnage_principal::PersonnagePrincipal;
use crate::classes::gestion_evenement::evenement::Evenement;

pub struct ReposOui {
    prix_repos: u32,
    personnage: Rc<RefCell<PersonnagePrincipal>>
}

impl ReposOui {
    pub fn new(personnage: Rc<RefCell<PersonnagePrincipal>>, prix_repos: u32) -> Self {
        Self { personnage, prix_repos }
    }

    pub fn accepter(&mut self) {
        let mut perso = self.personnage.borrow_mut();
        let delais = 10;
        perso.inventaire.remove_monnaie(self.prix_repos);
        AfficheTexte::affiche("Vous vous reposez...".to_string(), delais);
        let trois_secondes = std::time::Duration::from_secs(3);
        thread::sleep(trois_secondes);

        perso.entite.soigner_completement();
        AfficheTexte::affiche("Vous êtes complètement soigné !".to_string(), delais);
    }
}

impl Evenement for ReposOui {
    fn action(&mut self) {
        self.accepter();
    }
}
