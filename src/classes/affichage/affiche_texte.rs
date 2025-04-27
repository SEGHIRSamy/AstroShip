use std::{thread, time, io::Write};

#[allow(dead_code)]
pub struct AfficheTexte {
    texte: String,
}
#[allow(dead_code)]
impl AfficheTexte {

    pub fn new(texte: String) -> AfficheTexte {
        AfficheTexte {
            texte
        }
    }
    pub fn affiche(&self, delay_ms: u64) {
        for c in self.texte.chars() {
            print!("{}", c);
            std::io::stdout().flush().unwrap();
            thread::sleep(time::Duration::from_millis(delay_ms));
        }
        println!();
    }


}