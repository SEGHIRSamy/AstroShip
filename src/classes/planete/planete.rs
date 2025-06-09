use std::{cell::RefCell, rc::Rc};

use serde::{Deserialize, Serialize};
use crate::classes::{
    affichage::affichage_deplacement::AffichageDeplacement, entite::personnage_principal::PersonnagePrincipal, gestion_evenement::{choix::Choix, planete::{auberge_proposer_repos::AubergeProposerRepos, explorer_zone_hostile::ExplorerZoneHostile, magasin_interaction::MagasinInteraction, stop_choix::StopChoix}}, planete::{auberge::Auberge, magasin::Magasin, zone_hostile::ZoneHostile}, sauvegarde::sauvegarde::Sauvegarde
};
use crate::classes::gestion_evenement::action_combat::inventaire_interaction::InventaireInteraction;
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

            let consome = Rc::new(RefCell::new(false));
            let inventaire_consulte = Rc::new(RefCell::new(false));
            let inventaire = Box::new(InventaireInteraction::new(Rc::clone(&consome), Rc::clone(&tmp_personnage), Rc::clone(&inventaire_consulte)));


            let mut choix = Choix::new(vec![
                ("Explorer une zone hostile".to_string(), zone_event),
                ("Aller à l'auberge".to_string(), auberge_event),
                ("Marchander avec le magasin".to_string(), magasin_event),
                ("Inventaire".to_string(), inventaire),
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


    pub fn get_cout_voyage(&self) -> u32 {
        self.cout_voyage
    }

    pub fn add_cout_voyage(&mut self, cout_voyage: u32) {
        self.cout_voyage += cout_voyage;
    }

    pub fn charge_planete(nom: &str,flag_default: bool) -> Planete {
        let sauvegarde: Sauvegarde = Sauvegarde::new();

        if flag_default {
            sauvegarde.charge("planete_default/".to_owned() + &*nom.to_owned() + &*".json".to_string()).unwrap()
        }
        else {
            sauvegarde.charge("planete_json/".to_owned() + &*nom.to_owned() + &*".json".to_string()).unwrap()
        }
    }

    pub fn sauvegarde_planete(planete: Planete)  {
        let sauvegarde: Sauvegarde = Sauvegarde::new();
        sauvegarde.sauvegarde("planete_json/".to_owned() + &*planete.nom.to_owned() + &*".json".to_string(), planete).expect("TODO: panic message");
    }
}
