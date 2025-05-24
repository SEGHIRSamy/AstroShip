use crate::classes::affichage::affiche_texte::AfficheTexte;
use std::io::{self, Write};
use crate::classes::gestion_evenement::evenement::Evenement;

pub struct Choix {
    choix: Vec<(String, Box<dyn Evenement>)>,
}

impl Choix {
    pub fn new(choix: Vec<(String, Box<dyn Evenement>)>) -> Self {
        Choix { choix }
    }

    pub fn affiche(&self) {
        for (index, (texte, _)) in self.choix.iter().enumerate() {
            let option = format!("[{}] - {}", index + 1, texte);
            AfficheTexte::affiche(option, 30);
        }
    }

    pub fn demander_choix(&self) -> &dyn Evenement {
        let total_choix = self.choix.len();

        loop {
            print!("=> 1-{} : ", total_choix);
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            match input.trim().parse::<usize>() {
                Ok(num) if (1..=total_choix).contains(&num) => {
                    return self.choix[num - 1].1.as_ref();
                }
                _ => println!("Entrée invalide. Essayez encore."),
            }
        }
    }

    pub fn lancer_choix(&self) {
        self.affiche();
        let evenement = self.demander_choix();
        evenement.action();
    }

}