// src/main.rs
pub struct Objet {
    pub nom: String,
    pub description: String,
}

impl Objet {
    pub fn new(nom: &str, description: &str) -> Self {
        Objet {
            nom: nom.to_string(),
            description: description.to_string(),
        }
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn get_nom(&self) -> &str {
        &self.nom
    }

    pub fn set_description(&mut self, desc: String) {
        self.description = desc;
    }

    pub fn set_nom(&mut self, n: String) {
        self.nom = n;
    }
}
