use serde::{Deserialize, Serialize};
use crate::classes::marchandage::objet::Objet;
use std::io::{self};

/// La structure `Inventaire` représente un inventaire contenant
/// une certaine somme d'argent (`monnaie`) et un objet spécifique (`instance`).
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Inventaire {
    monnaie : u32, // valeur maximal dans le jeu : 99,999,999
    #[serde(rename = "objets")]
    instance : Vec<Objet>, // variable contenant le type d'objet
}

#[allow(dead_code)]
impl Inventaire {
    /// Constructeur pour créer une nouvelle instance d'`Inventaire`.
    /// - `monnaie` : Montant initial de monnaie.
    /// - `instance` : Objet initial associé à l'inventaire.
    pub fn new() -> Inventaire {
        Inventaire {
            monnaie : 0,
            instance: Vec::new(), // Liste vide

        }
    }
    /// Retourne la quantité de monnaie actuelle dans l'inventaire.
    pub fn get_monnaie(&self) -> u32 {
        self.monnaie
    }

    /// Retourne une référence immuable à l'objet présent dans l'inventaire.
    pub fn get_instance(&self) -> &Vec<Objet> {
        &self.instance
    }

    /// Met à jour l'objet de l'inventaire avec un nouvel objet.
    /// - `instance` : L'objet qui remplacera l'objet existant.
    pub fn set_instance(&mut self, instance : Vec<Objet>) {
        self.instance = instance;
    }

    pub fn add_objet(&mut self, objet: Objet) {
        self.instance.push(objet);
    }

    /// Met à jour la quantité de monnaie.
    /// - `monnaie` : La nouvelle somme à assigner dans l'inventaire.
    pub fn set_monnaie(&mut self, monnaie : u32) {
        self.monnaie = monnaie;
    }

    /// Ajoute une quantité donnée de monnaie à l'inventaire.
    /// - `monnaie` : La somme à ajouter.
    pub fn add_monnaie(&mut self, monnaie : u32) {
        self.monnaie += monnaie;
    }

    /// Réduit une quantité donnée de monnaie de l'inventaire.
    /// - `monnaie` : La somme à retirer.
    /// Attention : Cette méthode pourrait provoquer un dépassement négatif si elle n'est pas correctement encadrée.
    pub fn remove_monnaie(&mut self, monnaie : u32) {
        self.monnaie -= monnaie;
    }

    /// Vérifie si l'inventaire est vide (monnaie égale à zéro).
    pub fn is_empty(&self) -> bool {
        self.monnaie == 0
    }

    /// Vérifie si l'inventaire est plein.
    /// Actuellement, la limite est définie par `u32::MAX` au lieu de tout plafond personnalisé.
    pub fn is_full(&self) -> bool {
        if self.monnaie == u32::MAX {
            true
        } else {
            false
        }
    }

    /// Permet de supprimer un objet en fonction de son nom
    /// Supprime un objet de l'inventaire par son nom
    pub fn remove_objet_par_nom(&mut self, nom_objet: &str) {
        self.instance.retain(|o| o.get_nom() != nom_objet);
    }


    pub fn afficher_inventaire(&self) {
        println!("====================");
        println!("=== INVENTAIRE ===");
        println!("====================");
        println!("Monnaie : {}\n", self.get_monnaie());

        if self.instance.is_empty() {
            println!("Aucun objet dans l'inventaire.");
        } else {
            println!("Objets disponibles :");
            for (i, objet) in self.instance.iter().enumerate() {
                println!(
                    "  [{}] {} - {}",
                    i + 1,
                    objet.get_nom(),
                    objet.get_description()
                );
            }
        }
    }

    // AFFICHAGE DE L'INVENTAIRE


    // Afficher tout l'inventaire

    pub fn afficher_inventaire_interactif(&mut self) -> bool {
        loop {
            println!("\n====================");
            println!("=== Inventaire ===");
            println!("====================");
            println!("Monnaie : {}", self.get_monnaie());

            for (i, objet) in self.instance.iter().enumerate() {
                println!(
                    "{}. {} (x{}) - {}",
                    i + 1,
                    objet.get_nom(),
                    objet.get_quantite(),
                    objet.get_description()
                );
            }

            println!("\nEntrez le numéro d’un objet pour voir les détails, ou [Q] pour quitter :");

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            if input.eq_ignore_ascii_case("q") {
                return false; // aucun objet consommé
            }

            match input.parse::<usize>() {
                Ok(index) => {
                    if index > 0 && index <= self.instance.len() {
                        let a_consomme = self.afficher_details_objet(index);
                        if a_consomme {
                            return true; // on a consommé quelque chose → fin de l'inventaire
                        }
                        // Sinon, on boucle
                    } else {
                        println!("Numéro invalide.");
                    }
                }
                Err(_) => {
                    println!("Entrée invalide.");
                }
            }
        }
    }





    // Afficher les details d'un item

    pub fn afficher_details_objet(&mut self, index: usize) -> bool {
        if index == 0 || index > self.instance.len() {
            println!("Index invalide.");
            return false;
        }

        let objet = &self.instance[index - 1];

        println!("\n=== Détails de l'objet ===");
        println!("Nom : {}", objet.get_nom());
        println!("Description : {}", objet.get_description());

        let consommable = objet.est_consommable(); // Une méthode booléenne que tu dois créer

        if let Some(pv) = objet.get_multiplicateur_pv() {
            println!("+{:.0}% PV", pv * 100.0);
        }
        if let Some(pv_max) = objet.get_multiplicateur_pv_max() {
            println!("+{:.0}% PV max", pv_max * 100.0);
        }
        if let Some(force) = objet.get_multiplicateur_force() {
            println!("+{:.0}% Force", force * 100.0);
        }
        if let Some(vitesse) = objet.get_multiplicateur_vitesse() {
            println!("+{:.0}% Vitesse", vitesse * 100.0);
        }

        println!("\n[C] Consommer / [R] Retour");

        let mut choix = String::new();
        io::stdin().read_line(&mut choix).unwrap();

        match choix.trim().to_lowercase().as_str() {
            "c" => {
                if !consommable {
                    println!("Cet objet ne peut pas être consommé.");
                    return false;
                }

                let objet = &mut self.instance[index - 1];
                let nom_objet = objet.get_nom().to_string();
                objet.consommer_perso_principal(&nom_objet);
                println!("Objet consommé !");

                if objet.get_quantite() > 1 {
                    objet.set_quantite(objet.get_quantite() - 1);
                } else {
                    self.instance.remove(index - 1);
                    println!("Objet supprimé.");
                }

                return true;
            }
            _ => {
                println!("Retour à l'inventaire.");
                return false;
            }
        }
    }




}