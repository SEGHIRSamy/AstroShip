use serde::{Deserialize, Serialize};
use crate::classes::{
    planete::auberge::Auberge,
    planete::magasin::Magasin,
    planete::zone_hostile::ZoneHostile,
    entite::personnage_principal::PersonnagePrincipal,
    sauvegarde::sauvegarde::Sauvegarde,
    affichage::affichage_deplacement::AffichageDeplacement
};


// Permet de sérialiser/désérialiser la planète pour l'enregistrer ou la lire depuis un fichier JSON
#[allow(dead_code)]
// Permet de sérialiser/désérialiser la planète pour l'enregistrer ou la lire depuis un fichier JSON
#[derive(Serialize, Deserialize)]
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
            println!("\nBienvenue sur la planète {} !", self.nom);
            println!("\nVotre réserve de carburant : [{}]", personnage.get_carburant());
            println!("Que souhaitez-vous faire ?");
            println!("[1] Explorer une zone hostile");
            println!("[2] Aller à l'auberge");
            println!("[3] Marchander avec le magasin");
            println!("[4] Quitter la planète");

            let mut choix = String::new();
            std::io::stdin().read_line(&mut choix).unwrap();

            match choix.trim() {
                "1" => self.zone_hostile.explorer(),
                "2" => self.auberge.proposer_repos(personnage, None),
                "3" => self.magasin.interaction_magasin(personnage),
                "4" => {
                    println!("Vous quittez la planète {}.", self.nom);

                    AffichageDeplacement::lancer_animation_spatiale("depart", self.phrase_arrive.clone());
                    break;
                }
                _ => println!("Choix invalide."),
            }
        }
    }

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
