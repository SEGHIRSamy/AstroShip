use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ObjetJson {
    pub nom: String,
    pub description: String,
    pub quantite: u8,
}

#[derive(Serialize, Deserialize)]
pub struct InventaireJson {
    pub monnaie: u32,
    pub objets: Vec<ObjetJson>,
}

#[derive(Serialize, Deserialize)]
pub struct PersonnagePrincipalJson {
    pub nom: String,
    pub points_de_vie: u32,
    pub points_de_vie_max: u32,
    pub force: u32,
    pub intelligence: u32,
    pub vitesse: u32,
    pub chance: u32,
    pub inventaire: InventaireJson,
}
