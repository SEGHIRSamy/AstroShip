use std::cell::RefCell;
use std::rc::Rc;
use std::{thread};
use serde::{Deserialize, Serialize};
use crate::classes::sauvegarde::sauvegarde::Sauvegarde;
use crate::classes::entite::personnage_principal::PersonnagePrincipal;
use crate::classes::affichage::affichage_deplacement::AffichageDeplacement;
use crate::classes::gestion_evenement::auberge::repos_non::ReposNon;
use crate::classes::gestion_evenement::auberge::repos_oui::ReposOui;
use crate::classes::gestion_evenement::choix::Choix;

#[allow(dead_code)]
#[derive(Serialize, Deserialize,Clone)]
pub struct Auberge {
    prix_repos: u32, // Prix pour se reposer
    phrase_arrive: Vec<String>,
}

#[allow(dead_code)]
impl Auberge {
    /// Créer une nouvelle auberge avec un prix fixe.
    pub fn new(prix_repos: u32, phrase_arrive: Vec<String>) -> Self {
        Self { prix_repos, phrase_arrive }
    }

    pub fn proposer_repos(&self) {
        let sauvegarde: Sauvegarde = Sauvegarde::new();
        let personnage : PersonnagePrincipal = sauvegarde.charge("personnage_principal.json".to_string()).unwrap();
        AffichageDeplacement::lancer_animation("auberge", self.phrase_arrive.clone());

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

        println!("Vos pv actuellement [{}/{}]",personnage.entite.get_points_de_vie(), personnage.entite.get_points_de_vie_max());
        // Si le choix est fourni, on utilise directement cette valeur (pour les tests)
        let personnage_rc = Rc::new(RefCell::new(personnage));
        let oui = Box::new(ReposOui::new(Rc::clone(&personnage_rc), self.get_prix_repos()));
        let non = Box::new(ReposNon::new());

        let mut choix = Choix::new(vec![
            ("Oui".to_string(), oui),
            ("Non".to_string(), non),
        ]);

        choix.lancer_choix();
        drop(choix);

    }

    pub fn proposer_repos_test(&self,  personnage: &mut PersonnagePrincipal,choix: Option<u8>) {
        println!("Bienvenue à l'auberge. Le prix pour se reposer est de {} pièces.",self.prix_repos);
        println!("Vos points de vies sont actuellement de : {}",personnage.entite.get_points_de_vie());

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

    pub fn get_prix_repos(&self) -> u32 {
        self.prix_repos
    }
}