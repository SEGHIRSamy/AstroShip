use rand::Rng;
use crate::classes::entite::entite::Entite;
use crate::classes::marchandage::butin::Butin;

#[allow(dead_code)]
pub struct Ennemi {
    base: Entite,                 // L'entité de base pour les statistiques
    pub butins_passifs: Vec<Butin>,   // Butins obtenus si on le laisse vivre
    pub butins_hostiles: Vec<Butin>,  // Butins obtenus si on le tue
}

#[allow(dead_code)]
impl Ennemi {
    /// Créer un nouvel ennemi
    pub fn new(
        nom: &str,
        points_de_vie: i32,
        points_de_vie_max: i32,
        force: i32,
        intelligence: i32,
        vitesse: i32,
        butins_passifs: Vec<Butin>,
        butins_hostiles: Vec<Butin>,
    ) -> Self {
        Self {
            base: Entite::new(nom, points_de_vie, points_de_vie_max, force, intelligence, vitesse),
            butins_passifs,
            butins_hostiles,
        }
    }

    /// Interagir avec l'ennemi : décider de tuer ou d'épargner (
    pub fn interaction<R: Rng>(&self, rng: &mut R) -> Vec<Butin> {
        if self.base.get_points_de_vie() <= 0 {
            println!("{} a été vaincu !", self.base.get_nom());
            self.obtenir_butins_hostiles(rng)
        } else {
            println!("{} a été épargné.", self.base.get_nom());
            self.obtenir_butins_passifs(rng)
        }
    }

    /// tirer des résultats aléatoires en situation réelle
    pub fn interaction_par_defaut(&self) -> Vec<Butin> {
        let mut rng = rand::thread_rng();
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