// src/main.rs

use serde::{Deserialize, Serialize};
use crate::classes::entite::entite::Entite;
use crate::classes::sauvegarde::sauvegarde::Sauvegarde;
use crate::classes::entite::personnage_principal::PersonnagePrincipal;

#[derive(PartialEq, Debug)] // Ajout de Debug pour la comparaison dans les tests
#[derive(Clone)]
#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
/// La structure `Objet` représente un objet générique avec un nom et une description.
pub struct Objet {
    nom: String,
    description: String,
    quantite: u32,

    // Multiplicateurs appliqués si l'objet est un consommable
    #[serde(skip_serializing_if = "Option::is_none")]
    multiplicateur_pv: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multiplicateur_pv_max: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multiplicateur_force: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multiplicateur_vitesse: Option<f32>,
}
#[allow(dead_code)]
impl Objet {
    /// Crée une nouvelle instance d'un `Objet`.
    ///
    /// # Arguments
    /// * `nom` - Le nom de l'objet (chaîne de caractères).
    /// * `description` - Une description du rôle et des caractéristiques de l'objet (chaîne de caractères).
    ///
    /// # Retourne
    /// Une instance de la structure `Objet` initialisée avec `nom` et `description`.
    pub fn new(nom: &str, description: &str,quantite: u32) -> Self {
        Objet {
            nom: nom.to_string(),
            description: description.to_string(),
            quantite,
            multiplicateur_pv: None,
            multiplicateur_pv_max: None,
            multiplicateur_force: None,
            multiplicateur_vitesse: None,
        }
    }

    /// Retourne une référence immuable à la description de l'objet.
    ///
    /// # Retourne
    /// * Une référence immuable vers une chaîne de caractères décrivant l'objet.
    pub fn get_description(&self) -> &str {
        &self.description
    }

    /// Retourne une référence immuable au nom de l'objet.
    ///
    /// # Retourne
    /// * Une référence immuable vers une chaîne de caractères représentant le nom.
    pub fn get_nom(&self) -> &str {
        &self.nom
    }

    /// Modifie la description de l'objet.
    ///
    /// # Arguments
    /// * `desc` - Une nouvelle chaîne de caractères représentant la description de l'objet.
    pub fn set_description(&mut self, desc: String) {
        self.description = desc;
    }

    /// Modifie le nom de l'objet.
    ///
    /// # Arguments
    /// * `n` - Une nouvelle chaîne de caractères représentant le nom de l'objet.
    pub fn set_nom(&mut self, n: String) {
        self.nom = n;
    }

    pub fn get_quantite(&self) -> u32 {
        self.quantite
    }

    pub fn set_quantite(&mut self, q: u32) {
        self.quantite = q;
    }

    pub fn get_multiplicateur_pv(&self) -> Option<f32> {
        self.multiplicateur_pv
    }

    pub fn set_multiplicateur_pv(&mut self, pv: f32) {
        self.multiplicateur_pv = Some(pv);
    }
    pub fn get_multiplicateur_force(&self) -> Option<f32> {
        self.multiplicateur_force
    }
    pub fn set_multiplicateur_force(&mut self, force: f32) {
        self.multiplicateur_force = Some(force);
    }

    pub fn get_multiplicateur_vitesse(&self) -> Option<f32> {
        self.multiplicateur_vitesse
    }

    pub fn set_multiplicateur_vitesse(&mut self, vitesse: f32) {
        self.multiplicateur_vitesse = Some(vitesse);
    }

    pub fn get_multiplicateur_pv_max(&self) -> Option<f32> {
        self.multiplicateur_pv_max
    }

    pub fn set_multiplicateur_pv_max(&mut self, pv: f32) {
        self.multiplicateur_pv_max = Some(pv);
    }

    pub fn consommer(&self, cible: &mut Entite) {
        if let Some(mul) = self.multiplicateur_pv {
            cible.add_points_de_vie( ((cible.get_points_de_vie() as f32) * mul) as u32);
        }
        if let Some(mul) = self.multiplicateur_force {
            cible.add_force(((cible.get_force() as f32) * mul) as u32);
        }
        if let Some(mul) = self.multiplicateur_vitesse {
            cible.add_vitesse( ((cible.get_vitesse() as f32) * mul) as u32);
        }
        if let Some(mul) = self.multiplicateur_pv_max {
            cible.add_points_de_vie_max( ((cible.get_points_de_vie_max() as f32) * mul) as u32);
        }
    }

    pub fn est_consommable(&self) -> bool {
        self.multiplicateur_pv.is_some()
            || self.multiplicateur_pv_max.is_some()
            || self.multiplicateur_force.is_some()
            || self.multiplicateur_vitesse.is_some()
    }


    pub fn consommer_perso_principal(&self, nom_objet: &str) {
        let sauvegarde = Sauvegarde::new();
        let charge_player: PersonnagePrincipal = sauvegarde
            .charge("personnage_principal.json".to_string())
            .expect("Chargement du joueur échoué");

        // === RÉCUPÉRATION DES STATS ===
        let mut pv = charge_player.entite.get_points_de_vie();
        let mut pv_max = charge_player.entite.get_points_de_vie_max();
        let mut force = charge_player.entite.get_force();
        let mut vitesse = charge_player.entite.get_vitesse();

        // === APPLICATION DES MULTIPLICATEURS DIRECTS ===
        if let Some(mul) = self.multiplicateur_pv_max {
            pv_max = ((pv_max as f32) * mul).round().max(1.0) as u32;
        }

        if let Some(mul) = self.multiplicateur_pv {

            if mul > 1.0 {
                let ajout = ((pv_max as f32) * (mul - 1.0)).round() as u32;
                pv = (pv + ajout).min(pv_max);
            } else if mul < 1.0 {
                let retrait = ((pv_max as f32) * mul).round() as u32;
                pv = pv.saturating_sub(retrait); // évite que ça tombe en dessous de 0
            }
            if pv > pv_max {
                pv = pv_max;
            }
        }

        if let Some(mul) = self.multiplicateur_force {
            force = ((force as f32) * mul).round().max(1.0) as u32;
        }

        if let Some(mul) = self.multiplicateur_vitesse {
            vitesse = ((vitesse as f32) * mul).round().max(1.0) as u32;
        }

        // === MISE À JOUR DE L'INVENTAIRE ===
        let mut nouveaux_objets = Vec::new();
        for mut objet in charge_player.inventaire.get_instance().clone() {
            if objet.get_nom() == nom_objet {
                if objet.get_quantite() > 1 {
                    objet.set_quantite(objet.get_quantite() - 1);
                    nouveaux_objets.push(objet);
                }
                // Sinon, on le supprime
            } else {
                nouveaux_objets.push(objet);
            }
        }

        // === MISE À JOUR DU PERSONNAGE ===
        let mut update_player = PersonnagePrincipal::new(
            charge_player.entite.get_nom(),
            pv,
            pv_max,
            force,
            charge_player.entite.get_intelligence(),
            vitesse,
            charge_player.chance,
            charge_player.get_uranium(),
            charge_player.get_carburant(),
            charge_player.get_planete_nom().to_string(),
        );

        update_player.inventaire.set_instance(nouveaux_objets);

        // === SAUVEGARDE ===
        sauvegarde
            .sauvegarde("personnage_principal.json".to_string(), update_player)
            .expect("Sauvegarde du joueur échouée");
    }


    pub fn afficher(&self) {
        println!("📦 Objet : {}", self.nom);
        println!("   📖 Description : {}", self.description);
        println!("   🔢 Quantité : {}", self.quantite);

        if let Some(pv) = self.multiplicateur_pv {
            println!("   ❤ Recuperation PV : x{:.2}", pv);
        }
        if let Some(pv_max) = self.multiplicateur_pv_max {
            println!("   ❤ Multiplicateur PV Max : x{:.2}", pv_max);
        }
        if let Some(force) = self.multiplicateur_force {
            println!("   o()xxxx[@::::::::::::::::::> Multiplicateur Force : x{:.2}", force);
        }
        if let Some(vitesse) = self.multiplicateur_vitesse {
            println!("   ⚡ Multiplicateur Vitesse : x{:.2}", vitesse);
        }
    }


}
