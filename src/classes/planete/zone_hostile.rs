use rand::{rng, SeedableRng};
use std::io;
use rand::prelude::StdRng;
use serde::{Deserialize, Serialize};
use crate::classes::sauvegarde::sauvegarde::Sauvegarde;
use crate::classes::entite::ennemie::Ennemi;
use crate::classes::entite::personnage_principal::PersonnagePrincipal;
use crate::classes::gestion_evenement::combat::Combat;

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
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
        println!("Vous venez de vous aventurer dans la zone hostile : ");

        let mut rng = rng();

        for ennemi in &mut self.ennemis {
            println!("\n {} apparaît : ", ennemi.get_base().get_nom());

            let intro = "Un terrible ennemi apparaît ! Préparez-vous au combat !";
            let sauvegarde: Sauvegarde = Sauvegarde::new();
            let personnage_principale : PersonnagePrincipal  = sauvegarde.charge("personnage_principal.json".to_string()).unwrap();

            let resultat = Combat::lancer_combat(
                intro,
                personnage_principale.entite.get_points_de_vie(),
                personnage_principale.entite.get_force(),
                personnage_principale.entite.get_vitesse(),
                ennemi.get_base().get_points_de_vie(),
                ennemi.get_base().get_force(),
                ennemi.get_base().get_vitesse(),
            );
            // ⚔️ Combat terminé : interaction
            let butins = match resultat {
                true => {
                    // Ennemi vaincu
                    ennemi.interaction(&mut rng)
                }
                false => {
                    println!("Vous avez fui ou perdu le combat.");
                    break;
                }
            };

            println!("Butins obtenus :");
            for butin in butins {
                println!(" - {}", butin.objet.get_nom());
            }

            // Demande au joueur s'il veut continuer
            println!("Souhaitez-vous continuer à explorer ? (oui/non)");
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            if input.trim().to_lowercase() == "non" {
                println!("Vous quittez la zone hostile.");
                break;
            }
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
