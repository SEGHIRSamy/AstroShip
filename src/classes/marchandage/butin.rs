use rand::Rng;
use serde::{Deserialize, Serialize};
use crate::classes::marchandage::objet::Objet;

/// Enumération pour représenter la rareté d'un butin

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub enum Rarete {
    Commun,
    Rare,
    Epique,
    Legendaire,
}

#[allow(dead_code)]
impl Rarete {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "commun" => Some(Rarete::Commun),
            "rare" => Some(Rarete::Rare),
            "epique" => Some(Rarete::Epique),
            "legendaire" => Some(Rarete::Legendaire),
            _ => None,
        }
    }
}

#[allow(dead_code)]
#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct Butin {
    pub objet: Objet,          // L'objet associé au butin
    pub quantite: u32,         // Quantité de l'objet dans le butin
    pub probabilite: f32,      // Probabilité d'obtenir cet objet (entre 0.0 et 1.0)
    pub rarete: Rarete,        // Niveau de rareté de l'objet
}

#[allow(dead_code)]
impl Butin {
    pub fn new(objet: Objet, quantite: u32, probabilite: f32, rarete: Rarete) -> Self {
        if probabilite < 0.0 || probabilite > 1.0 {
            panic!("La probabilité doit être entre 0.0 et 1.0 !");
        }

        Self {
            objet,
            quantite,
            probabilite,
            rarete,
        }
    }

    /// Vérifie si le butin est obtenu à partir d'un tirage aléatoire
    pub fn est_obtenu<R: Rng>(&self, rng: &mut R) -> bool {
        let tirage: f32 = rng.random(); // Tire un nombre aléatoire entre 0.0 et 1.0
        tirage <= self.probabilite
    }

    pub fn get_objet(&self) -> Objet {
        self.objet.clone()
    }

}