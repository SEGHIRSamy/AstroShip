use crate::classes::{gestion_evenement::evenement::Evenement, planete::auberge::Auberge};

pub struct AubergeProposerRepos<'a> {
    auberge: &'a Auberge,
}

impl<'a> AubergeProposerRepos<'a> {
    pub fn new(auberge: &'a Auberge) -> Self {
        Self { auberge }
    }
}

impl<'a> Evenement for AubergeProposerRepos<'a> {
    fn action(&mut self) {
        self.auberge.proposer_repos();
    }
}