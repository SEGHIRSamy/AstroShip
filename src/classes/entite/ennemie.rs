use rand::Rng;
use serde::{Deserialize, Serialize};
use crate::classes::entite::entite::Entite;
use crate::classes::marchandage::butin::Butin;

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
pub struct Ennemi {
    pub base: Entite,                 // L'entité de base pour les statistiques
    pub butins_passifs: Vec<Butin>,   // Butins obtenu si on le laisse vivre
    pub butins_hostiles: Vec<Butin>,  // Butins obtenu si on le tue
    pub phrase_intro: String,          // Phrase d'intro lancée au début du combat
    pub phrase_attaque: String,         // Phrase lancée quand l'ennemi attaque
}

#[allow(dead_code)]
impl Ennemi {
    /// Créer un nouvel ennemi
    pub fn new(
        nom: &str,
        points_de_vie: u32,
        points_de_vie_max: u32,
        force: u32,
        intelligence: u32,
        vitesse: u32,
        butins_passifs: Vec<Butin>,
        butins_hostiles: Vec<Butin>,
        phrase_intro: String,
        phrase_attaque: String,
    ) -> Self {
        Self {
            base: Entite::new(nom, points_de_vie, points_de_vie_max, force, intelligence, vitesse),
            butins_passifs,
            butins_hostiles,
            phrase_intro,
            phrase_attaque,
        }
    }

    /// Interagir avec l'ennemi : décider de tuer ou d'épargner (
    pub fn interaction<R: Rng>(&self, rng: &mut R) -> Vec<Butin> {
        if self.base.get_points_de_vie() <= 0 {
            self.obtenir_butins_hostiles(rng)
        } else {
            self.obtenir_butins_passifs(rng)
        }
    }

    /// tirer des résultats aléatoires en situation réelle
    pub fn interaction_par_defaut(&self) -> Vec<Butin> {
        let mut rng = rand::rng();
        self.interaction(&mut rng)
    }

    pub fn get_base(&self) -> &Entite {
        &self.base
    }

    pub fn obtenir_butins_passifs<R: Rng>(&self, rng: &mut R) -> Vec<Butin> {
        self.butins_passifs
            .iter()
            .filter(|butin| butin.est_obtenu(rng)) // `rng` est maintenant passé
            .cloned()
            .collect()
    }

    pub fn obtenir_butins_hostiles<R: Rng>(&self, rng: &mut R) -> Vec<Butin> {
        self.butins_hostiles
            .iter()
            .filter(|butin| butin.est_obtenu(rng)) // `rng` est maintenant passé
            .cloned()
            .collect()
    }

}