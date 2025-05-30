// src/main.rs

use serde::{Deserialize, Serialize};
use crate::classes::entite::entite::Entite;

#[derive(PartialEq, Debug)] // Ajout de Debug pour la comparaison dans les tests
#[derive(Clone)]
#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
/// La structure `Objet` représente un objet générique avec un nom et une description.
pub struct Objet {
    nom: String,
    description: String,
    quantite: u8,

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
    pub fn new(nom: &str, description: &str,quantite: u8) -> Self {
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

    pub fn get_quantite(&self) -> u8 {
        self.quantite
    }

    pub fn set_quantite(&mut self, q: u8) {
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
}
