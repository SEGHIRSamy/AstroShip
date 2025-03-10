use crate::classes::affichage::affiche_texte::AfficheTexte;

pub struct Choix{
    texte: String,
}

impl Choix{
    pub fn new(texte: String) -> Choix {
        Choix {
            texte
        }
    }

    pub fn affiche(&self) {
        let choix_parts: Vec<&str> = self.texte.split('/').collect();
        for choix in choix_parts {
            let option = AfficheTexte::new(choix.trim().to_string());
            option.affiche(30);
        }
    }
}