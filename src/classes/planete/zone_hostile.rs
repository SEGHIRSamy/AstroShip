use rand::{SeedableRng};
use rand::prelude::StdRng;
use serde::{Deserialize, Serialize};
use crate::classes::entite::ennemie::Ennemi;
use crate::classes::gestion_evenement::combat::Combat;

#[allow(dead_code)]
#[derive(Serialize, Deserialize,Clone)]
pub struct ZoneHostile {
    ennemis: Vec<Ennemi>,
    nom: String,
}
#[allow(dead_code)]
impl ZoneHostile {
    pub fn new(nom: &str, ennemis: Vec<Ennemi>) -> Self {
        Self {
            ennemis,
            nom: nom.to_string(),
        }
    }

    pub fn get_nom(&self) -> &str {
        &self.nom
    }

    pub fn explorer(&mut self) {
        println!("Vous venez de vous aventurer dans la zone hostile :");

        let mut index = 0;
        loop {

            if index >= self.ennemis.len() {
                index = 0; // Recommence au début du vecteur
            }

            let ennemi = &mut self.ennemis[index];
            Combat::lancer_combat(ennemi);

            println!("Souhaitez-vous continuer à explorer ? (oui/non)");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

            if input.trim().to_lowercase() == "non" {
                println!("Vous quittez la zone hostile.");
                break;
            }

            index += 1;
        }

        println!("Exploration terminée.");
    }


    // Fonction de test
    pub fn explorer_auto(&mut self, continuer: impl Fn(usize) -> bool) -> Vec<String> {
        let mut rng = StdRng::seed_from_u64(42); // valeur fixe
        let mut butins_log = vec![];

        for (i, ennemi) in self.ennemis.iter_mut().enumerate() {
            let nom_ennemi = ennemi.get_base().get_nom();
            println!("\n{} apparaît :", nom_ennemi);

            let resultat = true; // simulé pour l'instant

            let butins = if resultat {
                ennemi.interaction(&mut rng)
            } else {
                println!("Vous avez fui ou perdu.");
                break;
            };

            for butin in &butins {
                let nom_butin = butin.objet.get_nom();
                butins_log.push(nom_butin.to_string());
                println!(" - {}", nom_butin);
            }

            if !continuer(i) {
                println!("Exploration arrêtée par le joueur.");
                break;
            }
        }

        butins_log
    }
}
