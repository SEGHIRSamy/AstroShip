use crate::classes::marchandage::objet::Objet;

/// La structure `Inventaire` représente un inventaire contenant
/// une certaine somme d'argent (`monnaie`) et un objet spécifique (`instance`).
#[allow(dead_code)]
pub struct Inventaire {
    monnaie : u32, // valeur maximal dans le jeu : 99,999,999
    instance : Objet, // variable contenant le type d'objet
}

#[allow(dead_code)]
impl Inventaire {
    /// Constructeur pour créer une nouvelle instance d'`Inventaire`.
    /// - `monnaie` : Montant initial de monnaie.
    /// - `instance` : Objet initial associé à l'inventaire.
    pub fn new(monnaie : u32, instance : Objet) -> Inventaire {
        Inventaire {
            monnaie,
            instance
        }
    }
    /// Retourne la quantité de monnaie actuelle dans l'inventaire.
    pub fn get_monnaie(&self) -> u32 {
        self.monnaie
    }

    /// Retourne une référence immuable à l'objet présent dans l'inventaire.
    pub fn get_instance(&self) -> &Objet {
        &self.instance
    }

    /// Met à jour l'objet de l'inventaire avec un nouvel objet.
    /// - `instance` : L'objet qui remplacera l'objet existant.
    pub fn set_instance(&mut self, instance : Objet) {
        self.instance = instance;
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

}