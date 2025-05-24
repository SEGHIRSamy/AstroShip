use rand::rng;
use std::io;
use crate::classes::entite::ennemie::Ennemi;

#[allow(dead_code)]
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

    pub fn explorer(&mut self) {
        println!("Bienvenue dans la zone hostile : {}", self.nom);

        let mut rng = rng();

        for ennemi in &mut self.ennemis {
            println!("\nUn ennemi apparaît : {}", ennemi.get_base().get_nom());

            // 🥊 Appelle ton système de combat ici
            //let resultat = lancer_combat(ennemi.get_base());

            let resultat = true;
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

            // 🔁 Demande au joueur s'il veut continuer
            println!("Souhaitez-vous continuer à explorer ? (o/n)");
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            if input.trim().to_lowercase() != "o" {
                println!("Vous quittez la zone hostile.");
                break;
            }
        }

        println!("Exploration terminée.");
    }
}
