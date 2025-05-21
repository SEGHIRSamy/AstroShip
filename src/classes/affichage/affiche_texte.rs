use std::{thread, time, io::Write};
use crate::classes::gestionEvenement::evenement::Evenement;

pub struct AfficheTexte {
    texte: String,
    delay_ms: u64,
}

impl AfficheTexte {
    pub fn new(texte: String, delay_ms: u64) -> AfficheTexte {
        AfficheTexte { texte, delay_ms }
    }

    // méthode statique rapide
    pub fn affiche(texte: String, delay_ms: u64) {
        for c in texte.chars() {
            print!("{}", c);
            std::io::stdout().flush().unwrap();
            thread::sleep(time::Duration::from_millis(delay_ms));
        }
        println!();
    }
}

impl Evenement for AfficheTexte {
    fn action(&self) {
        Self::affiche(self.texte.clone(), self.delay_ms);
    }
}
