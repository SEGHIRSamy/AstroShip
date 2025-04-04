use crate::classes::affichage::affiche_texte::AfficheTexte;
use std::io::{self, Write};

pub struct Choix {
    choix: Vec<String>,
}

impl Choix {
    pub fn new(texte: String) -> Choix {
        let choix = texte
            .split('/')
            .map(|s| s.trim().to_string())
            .collect();

        Choix { choix }
    }

    pub fn affiche(&self) {
        for (index, choix) in self.choix.iter().enumerate() {
            let option = format!("[{}] - {}", index + 1, choix);
            let affiche_option = AfficheTexte::new(option);
            affiche_option.affiche(30);
        }
    }

    pub fn demander_choix(&self) -> usize {
        let total_choix = self.choix.len();

        loop {
            print!("=> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            match input.trim().parse::<usize>() {
                Ok(num) if (1..=total_choix).contains(&num) => return num,
                _ => println!("Entrée invalide. Essayez encore."),
            }
        }
    }

    pub fn lancer_choix(&self) -> usize {
        self.affiche();
        self.demander_choix()
    }
}
