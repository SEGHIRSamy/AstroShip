use serde::{Deserialize, Serialize};
use crate::classes::entite::entite::{Entite, Personnage};
use crate::classes::entite::inventaire::Inventaire;


#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
#[derive(Clone)]
pub struct PersonnagePrincipal {
    pub entite: Entite,      // Composition, réutilise `Entite`
    pub inventaire: Inventaire, // Ajout d'un inventaire
    pub chance: u32,             // Nouvelle statistique
    pub uranium: u32,
    pub planete: String,
    pub carburant: u32
}

#[allow(dead_code)]
impl PersonnagePrincipal {
    pub fn new(
        nom: &str,
        points_de_vie: u32,
        points_de_vie_max: u32,
        force: u32,
        intelligence: u32,
        vitesse: u32,
        chance: u32,
        uranium: u32,
        carburant: u32,
        planete: String,
    ) -> Self {
        Self {
            entite: Entite::new(nom, points_de_vie, points_de_vie_max, force, intelligence, vitesse),
            inventaire: Inventaire::new(),
            chance,
            uranium,
            planete,
            carburant,
        }
    }

    pub fn get_uranium(&self) -> u32 {
        self.uranium
    }

    pub fn add_uranium(&mut self) {
        for obj in self.inventaire.get_instance() {
            if obj.get_nom().to_string() == "Uranium" {
                self.uranium = obj.get_quantite();
            }
        }
    }

    pub fn add_carburant(&mut self) {
        for obj in self.inventaire.get_instance() {
            if obj.get_nom().to_string() == "Carburant" {
                self.carburant = obj.get_quantite();
            }
        }
    }

    pub fn get_planete_nom(&self) -> &str {
        &self.planete
    }

    pub fn set_planete(&mut self, planete: &str) {
        self.planete = planete.to_string();
    }

    pub fn set_carburant(&mut self, carburant: u32) {
        self.carburant = carburant;
    }

    pub fn get_carburant(&self) -> u32 {
        self.carburant
    }

    pub fn get_chance(&self) -> u32 {
        self.chance
    }

}

#[allow(dead_code)]
impl Personnage for PersonnagePrincipal {
    fn augmentation_niveau(&mut self, choix_statistique: &str) {
        if choix_statistique == "chance" {
            self.chance += 1;
            println!("Votre chance a augmenté !");
        } else {
            // Appelle `augmentation_niveau` de l'entité sous-jacente
            self.entite.augmentation_niveau(choix_statistique);
        }
    }

    fn afficher_statistiques(&self) {
        self.entite.afficher_statistiques();
        println!("Chance : {}", self.chance);

        println!("\n=== Inventaire ===");
        self.inventaire.afficher_inventaire(); // Affiche le contenu de l'inventaire
    }

}

