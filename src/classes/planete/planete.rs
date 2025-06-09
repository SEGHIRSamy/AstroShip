use std::{cell::RefCell, rc::Rc};

use serde::{Deserialize, Serialize};
use crate::classes::{
    affichage::affichage_deplacement::AffichageDeplacement, entite::personnage_principal::PersonnagePrincipal, gestion_evenement::{choix::Choix, planete::{auberge_proposer_repos::AubergeProposerRepos, explorer_zone_hostile::ExplorerZoneHostile, magasin_interaction::MagasinInteraction, stop_choix::StopChoix}}, planete::{auberge::Auberge, magasin::Magasin, zone_hostile::ZoneHostile}, sauvegarde::sauvegarde::Sauvegarde
};
use crate::classes::gestion_evenement::quitter_jeu::QuitterJeu;


// Permet de sérialiser/désérialiser la planète pour l'enregistrer ou la lire depuis un fichier JSON
#[allow(dead_code)]
// Permet de sérialiser/désérialiser la planète pour l'enregistrer ou la lire depuis un fichier JSON
#[derive(Serialize, Deserialize,Clone)]
pub struct Planete {
    pub nom: String,
    pub auberge: Auberge,
    pub magasin: Magasin,
    pub cout_voyage : u32,
    //Todo zone passive
    pub zone_hostile: ZoneHostile,
    pub phrase_arrive: Vec<String>,
}

#[allow(dead_code)]
impl Planete {
    pub fn new(nom: &str, auberge: Auberge, magasin: Magasin, zone_hostile: ZoneHostile, phrase_arrive: Vec<String>) -> Self {
        Self {
            nom: nom.to_string(),
            auberge,
            magasin,
            cout_voyage: 0,
            zone_hostile,
            phrase_arrive,
        }
    }

    /// Proposer les 3 choix au joueur
    pub fn visiter(&mut self, personnage: &mut PersonnagePrincipal) {
        AffichageDeplacement::lancer_animation_spatiale("arrivee", self.phrase_arrive.clone());
        loop {
            let stop = Rc::new(RefCell::new(false));
            let tmp_personnage = Rc::new(RefCell::new(personnage.clone()));

            println!("\nBienvenue sur la planète {} !", self.nom);
            println!("\nVotre réserve de carburant : [{}]", personnage.get_carburant());
            println!("Que souhaitez-vous faire ?");

            let zone_event = Box::new(ExplorerZoneHostile::new(&mut self.zone_hostile, Rc::clone(&tmp_personnage)));
            let auberge_event = Box::new(AubergeProposerRepos::new(&self.auberge));
            let magasin_event = Box::new(MagasinInteraction::new(&mut self.magasin, Rc::clone(&tmp_personnage)));
            let stop_event = Box::new(StopChoix::new(self.nom.clone(), self.phrase_arrive.clone(), Rc::clone(&stop)));
            let quitter_jeu = Box::new(QuitterJeu::new());

            let mut choix = Choix::new(vec![
                ("Explorer une zone hostile".to_string(), zone_event),
                ("Aller à l'auberge".to_string(), auberge_event),
                ("Marchander avec le magasin".to_string(), magasin_event),
                ("Quitter la planète".to_string(), stop_event),
                ("Quitter le jeux".to_string(), quitter_jeu),
            ]);

            choix.lancer_choix();
            drop(choix);

            // Récupérer les données modifiées
            let personnage_modifie = Rc::try_unwrap(tmp_personnage)
                .expect("Personnage encore emprunté ailleurs")
                .into_inner();

            // copier dans `personnage` original si besoin
            *personnage = personnage_modifie;

            if *stop.borrow() {
                return;
            }
        }
    }

    /// Proposer les 3 choix au joueur
    // pub fn visiter(&mut self, personnage: &mut PersonnagePrincipal) {
    //     println!("\nBienvenue sur la planète {} !", self.nom);
    //     loop {
    //         println!("\nVotre réserve de carburant : [{}]", personnage.get_carburant());
    //         println!("Que souhaitez-vous faire ?");
    //         println!("[1] Explorer une zone hostile");
    //         println!("[2] Aller à l'auberge");
    //         println!("[3] Marchander avec le magasin");
    //         println!("[4] Quitter la planète");
    //         println!("[0] Arrêter de jouer");

    //         let mut choix = String::new();
    //         std::io::stdin().read_line(&mut choix).unwrap();
    //         //let choix_mort = Choix::new(vec![("[1] Explorer une zone hostile".to_string(),Box::new(self.zone_hostile.explorer())),]);
    //         let sauvegarde: Sauvegarde = Sauvegarde::new();
    //         match choix.trim() {
    //             "1" => {self.zone_hostile.explorer();
    //                 let tmp_joueur : PersonnagePrincipal = sauvegarde.charge("personnage_principal.json".to_string()).unwrap();
    //                 personnage.inventaire.set_monnaie(tmp_joueur.inventaire.get_monnaie());
    //                 },
    //             "2" => self.auberge.proposer_repos(),
    //             "3" => {self.magasin.interaction_magasin(personnage);

    //                 sauvegarde.sauvegarde("planete_json/".to_owned() + &*personnage.get_planete_nom().to_owned() + &*".json".to_string(), self.clone()).expect("TODO: panic message");
    //             },
    //             "0" => QuitterJeu::new().quitter_jeu(),
    //             "4" => {
    //                 println!("Vous quittez la planète {}.", self.nom);
    //                 break;
    //             }
    //             _ => println!("Choix invalide."),
    //         }
    //     }
    // }

    pub fn get_cout_voyage(&self) -> u32 {
        self.cout_voyage
    }

    pub fn add_cout_voyage(&mut self, cout_voyage: u32) {
        self.cout_voyage += cout_voyage;
    }

    pub fn charge_planete(nom: &str) -> Planete {
        let sauvegarde: Sauvegarde = Sauvegarde::new();

        sauvegarde.charge("planete_json/".to_owned() + &*nom.to_owned() + &*".json".to_string()).unwrap()
    }

    pub fn sauvegarde_planete(planete: Planete)  {
        let sauvegarde: Sauvegarde = Sauvegarde::new();
        sauvegarde.sauvegarde("planete_json/".to_owned() + &*planete.nom.to_owned() + &*".json".to_string(), planete).expect("TODO: panic message");
    }
}
