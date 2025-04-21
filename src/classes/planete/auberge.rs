use std::{thread};
use crate::classes::entite::personnage_principal::PersonnagePrincipal;

pub struct Auberge {
    prix_repos: u32, // Prix pour se reposer
}

impl Auberge {
    /// Créer une nouvelle auberge avec un prix fixe.
    pub fn new(prix_repos: u32) -> Self {
        Self { prix_repos }
    }

    pub fn proposer_repos(&self, personnage: &mut PersonnagePrincipal, choix: Option<u8>) {
        println!(
            "Bienvenue à l'auberge. Le prix pour se reposer est de {} pièces.",
            self.prix_repos
        );

        // Vérifier si le personnage a assez d'argent
        if personnage.inventaire.get_monnaie() < self.prix_repos {
            println!("Vous n'avez pas assez d'argent pour vous reposer.");
            return;
        }
        else if personnage.entite.get_points_de_vie() == personnage.entite.get_points_de_vie_max() {
            println!("Vous êtes en pleine forme ! [{}/{}]",personnage.entite.get_points_de_vie(), personnage.entite.get_points_de_vie_max());
            return;
        }

        // Si le choix est fourni, on utilise directement cette valeur (pour les tests)
        let reponse = if let Some(value) = choix {
            value
        } else {
            // Sinon, on demande à l'utilisateur
            println!("Souhaitez-vous vous reposer ? [1] Oui / [2] Non");

            let mut choix_utilisateur = String::new();
            std::io::stdin().read_line(&mut choix_utilisateur).unwrap();
            choix_utilisateur.trim().parse::<u8>().unwrap_or(2) // Par défaut, refuser
        };

        if reponse == 1 {
            // Déduire le prix et soigner le personnage
            personnage.inventaire.remove_monnaie(self.prix_repos);
            println!("Vous vous reposez...");
            let trois_secondes = std::time::Duration::from_secs(3);
            thread::sleep(trois_secondes);

            personnage.entite.soigner_completement();
            println!("Vous êtes complètement soigné !");
        } else {
            println!("Très bien, peut-être une autre fois.");
        }
    }
}