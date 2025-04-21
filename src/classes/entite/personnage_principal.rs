use crate::classes::entite::entite::{Entite, Personnage};
use crate::classes::entite::inventaire::Inventaire;

#[allow(dead_code)]
pub struct PersonnagePrincipal {
    pub entite: Entite,      // Composition, réutilise `Entite`
    pub inventaire: Inventaire, // Ajout d'un inventaire
    pub chance: i32,             // Nouvelle statistique
}

#[allow(dead_code)]
impl PersonnagePrincipal {
    pub fn new(
        nom: &str,
        points_de_vie: i32,
        points_de_vie_max: i32,
        force: i32,
        intelligence: i32,
        vitesse: i32,
        chance: i32,
    ) -> Self {
        Self {
            entite: Entite::new(nom, points_de_vie, points_de_vie_max, force, intelligence, vitesse),
            inventaire: Inventaire::new(),
            chance,
        }
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