use serde::{Deserialize, Serialize};

// Définition d'une structure pour représenter une entité (personnage, ennemi, PNJ)
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Clone)]
pub struct Entite {
    nom: String,
    points_de_vie: u32,
    points_de_vie_max: u32,
    force: u32,
    intelligence: u32,
    vitesse: u32,

}

#[allow(dead_code)]
pub trait Personnage {
    fn augmentation_niveau(&mut self, choix_statistique: &str);
    fn afficher_statistiques(&self);
}

#[allow(dead_code)]
impl Entite {
    /// Crée une nouvelle entité avec les statistiques spécifiées
    pub fn new(nom: &str, points_de_vie: u32, points_de_vie_max: u32, force: u32, intelligence: u32, vitesse: u32) -> Self {
        Self {
            nom: nom.to_string(),
            points_de_vie,
            points_de_vie_max,
            force,
            intelligence,
            vitesse
        }
    }

    /// Obtenir le nom de l'entité
    pub fn get_nom(&self) -> &str {
        &self.nom
    }

    pub fn get_force(&self) -> u32 {
        self.force
    }

    pub fn get_intelligence(&self) -> u32 {
        self.intelligence
    }

    pub fn get_vitesse(&self) -> u32 {
        self.vitesse
    }

    pub fn add_vitesse(&mut self, vitesse: u32) {
        self.vitesse += vitesse;
    }

    pub fn add_intelligence(&mut self, intelligence: u32) {
        self.intelligence += intelligence;
    }

    pub fn add_force(&mut self, force: u32) {
        self.force += force;
    }

    pub fn add_points_de_vie(&mut self, points_de_vie: u32) {
        self.points_de_vie += points_de_vie;
    }

    pub fn add_points_de_vie_max(&mut self, points_de_vie: u32) {
        self.points_de_vie_max += points_de_vie;
    }

    /// Vérifie si l'entité est vivante
    pub fn est_mort(&self) -> bool {
        self.points_de_vie <= 0
    }

    /// Obtenir les points de vie actuels de l'entité
    pub fn get_points_de_vie(&self) -> u32 {
        self.points_de_vie
    }

    /// Obtenir les points de vie actuels de l'entité
    pub fn get_points_de_vie_max(&self) -> u32 {
        self.points_de_vie_max
    }

    /// Appliquer des dégâts à l'entité
    pub fn subir_degats(&mut self, degats: u32) {
        let mut tmp_vie : i32 = self.points_de_vie as i32;
        tmp_vie -= degats as i32;
        if tmp_vie <= 0 {
            self.points_de_vie = 0;
        }
        else {
            self.points_de_vie = tmp_vie as u32;
        }
    }

    /// Soigner l'entité (ne dépasse pas les PV initiaux)
    pub fn soigner(&mut self, soin: u32) {
        if !self.est_mort() && self.points_de_vie + soin <= self.points_de_vie_max {
            self.points_de_vie += soin;
        }
    }

    /// Soigner l'entité complétement
    pub fn soigner_completement(&mut self) {
        if !self.est_mort() {
            self.points_de_vie = self.points_de_vie_max;
        }
    }



}

#[allow(dead_code)]
impl Personnage for Entite {
    fn augmentation_niveau(&mut self, choix_statistique: &str) {
        // Fonction de level-up
        // Augmenter les PV max et restaurer les PV normaux
        self.points_de_vie_max += 5;
        self.points_de_vie = self.points_de_vie_max;

        // Augmenter la statistique choisie
        match choix_statistique {
            "force" => {
                self.force += 1;
                println!("Votre force a augmenté !");
            }
            "intelligence" => {
                self.intelligence += 1;
                println!("Votre intelligence a augmenté !");
            }
            "vitesse" => {
                self.vitesse += 1;
                println!("Votre vitesse a augmenté !");
            }
            _ => {
                println!("Statistique inconnue, aucun changement effectué.");
            }
        }
        println!(
            "{} est monté en niveau ! Nouveaux points de vie max : {}.",
            self.nom, self.points_de_vie_max
        );

    }

    fn afficher_statistiques(&self) {
        println!("=== {} ===", self.nom);
        println!("Points de vie : {}/{}", self.points_de_vie, self.points_de_vie_max);
        println!("Force : {}", self.force);
        println!("Intelligence : {}", self.intelligence);
        println!("Vitesse : {}", self.vitesse);
    }
}
