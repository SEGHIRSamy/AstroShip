use serde::{Deserialize, Serialize};
use crate::classes::marchandage::objet::Objet;
#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
pub struct Affaire {
    prix : u32, // valeur maximal dans le jeu : 99,999,999
    instance : Objet, // variable contenant le type d'objet
    infini : bool, // variable contenant le type d'objet
    quantite : u8 // quantité
}
#[allow(dead_code)]
impl Affaire {

    /// Crée une nouvelle instance d'`Affaire`.
    ///
    /// # Arguments
    /// * `prix` - Le prix de l'affaire (u32).
    /// * `instance` - L'objet lié à l'affaire (de type `Objet`).
    /// * `infini` - Indique si la quantité est illimitée (booléen).
    /// * `quantite` - La quantité de l'objet dans l'affaire (u8).
    ///
    /// # Retourne
    /// Retourne une nouvelle structure `Affaire`.
    pub fn new(prix : u32, instance : Objet, infini : bool, quantite : u8) -> Self {
        Affaire {
            prix,
            instance,
            infini,
            quantite,
        }
    }

    /// Retourne une référence au prix de l'affaire.
    ///
    /// # Retourne
    /// Référence immuable au champ `prix` (u32).
    pub fn get_prix(&self) -> &u32 {
        &self.prix
    }

    /// Retourne une référence à l'objet associé à l'affaire.
    ///
    /// # Retourne
    /// Référence immuable au champ `instance` (de type `Objet`).
    pub fn get_instance(&self) -> &Objet {
        &self.instance
    }

    /// Retourne une référence au champ `infini`, qui indique si la quantité est infinie.
    ///
    /// # Retourne
    /// Référence immuable à la valeur booléenne `infini`.
    pub fn get_infini(&self) -> &bool {
        &self.infini
    }

    /// Retourne une référence à la quantité de l'objet dans l'affaire.
    ///
    /// # Retourne
    /// Référence immuable au champ `quantite` (u8).
    pub fn get_quantite(&self) -> &u8 {
        &self.quantite
    }

    /// Définit une nouvelle valeur pour le prix de l'affaire.
    ///
    /// # Arguments
    /// * `prix` - La nouvelle valeur du prix (u32).
    pub fn set_prix(&mut self, prix : u32) {
        self.prix = prix;
    }

    /// Définit un nouvel objet associé à l'affaire.
    ///
    /// # Arguments
    /// * `instance` - Le nouvel objet (de type `Objet`) à affecter au champ `instance`.
    pub fn set_instance(&mut self, instance : Objet) {
        self.instance = instance;
    }

    /// Définit une nouvelle valeur pour le champ `infini`.
    ///
    /// # Arguments
    /// * `infini` - Booléen indiquant si la quantité devient infinie (`true`) ou non (`false`).
    pub fn set_infini(&mut self, infini : bool) {
        self.infini = infini;
    }

    /// Définit une nouvelle valeur pour la quantité d'objet dans l'affaire.
    ///
    /// # Arguments
    /// * `quantite` - La nouvelle valeur de la quantité (u8).
    pub fn set_quantite(&mut self, quantite : u8) {
        self.quantite = quantite;
    }

}