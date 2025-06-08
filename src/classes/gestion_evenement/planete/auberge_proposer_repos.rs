use std::rc::Rc;
use std::cell::RefCell;

use crate::classes::{entite::personnage_principal::PersonnagePrincipal, gestion_evenement::evenement::Evenement, planete::auberge::Auberge};

pub struct AubergeProposerRepos<'a> {
    auberge: &'a Auberge,
    personnage: Rc<RefCell<PersonnagePrincipal>>,
}

impl<'a> AubergeProposerRepos<'a> {
    pub fn new(auberge: &'a Auberge, personnage: Rc<RefCell<PersonnagePrincipal>>) -> Self {
        Self { auberge, personnage }
    }
}

impl<'a> Evenement for AubergeProposerRepos<'a> {
    fn action(&mut self) {
        self.auberge.proposer_repos(Rc::clone(&self.personnage), None);
    }
}