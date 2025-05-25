use crate::classes::{
    planete::auberge::Auberge,
    planete::magasin::Magasin,
    planete::zone_hostile::ZoneHostile,
    entite::personnage_principal::PersonnagePrincipal,
};

#[allow(dead_code)]
pub struct Planete {
    pub nom: String,
    pub auberge: Auberge,
    pub magasin: Magasin,
    pub zone_hostile: ZoneHostile,
}

#[allow(dead_code)]
impl Planete {
    pub fn new(nom: &str, auberge: Auberge, magasin: Magasin, zone_hostile: ZoneHostile) -> Self {
        Self {
            nom: nom.to_string(),
            auberge,
            magasin,
            zone_hostile,
        }
    }

    /// Proposer les 3 choix au joueur
    pub fn visiter(&mut self, personnage: &mut PersonnagePrincipal) {
        loop {
            println!("\nBienvenue sur la planète {} !", self.nom);
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
                    break;
                }
                _ => println!("Choix invalide."),
            }
        }
    }
}
