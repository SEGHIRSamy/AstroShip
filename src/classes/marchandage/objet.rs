// src/main.rs

#[derive(PartialEq, Debug)] // Ajout de Debug pour la comparaison dans les tests
#[derive(Clone)]
#[allow(dead_code)]
/// La structure `Objet` représente un objet générique avec un nom et une description.
pub struct Objet {
    nom: String,
    description: String,
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
    pub fn new(nom: &str, description: &str) -> Self {
        Objet {
            nom: nom.to_string(),
            description: description.to_string(),
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
}
