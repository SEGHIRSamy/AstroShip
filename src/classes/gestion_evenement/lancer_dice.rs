use rand::Rng;
use crate::classes::affichage::affiche_texte::AfficheTexte;

#[allow(dead_code)]
pub struct LancerDice;
#[allow(dead_code)]
impl LancerDice {
    pub fn lancer() -> u32 {
        let mut rng = rand::rng();
        rng.random_range(1..=20)
    }

    // Pour lancer le dé et afficher le résultat dans la console lors des combats
    pub fn lancer_console_combat(tour_joueur: bool) -> u32 {
        let lancer = LancerDice::lancer();
        AfficheTexte::affiche("Lancement du dé 🎲".to_string(), 20);
        AfficheTexte::affiche("...".to_string(), 1000);
        std::thread::sleep(std::time::Duration::from_millis(200));
        if tour_joueur {
            AfficheTexte::affiche(format!("🎲 Vous avez lancé un dé : {}", lancer), 20);
        }
        else {
            AfficheTexte::affiche(format!("🎲 L'ennemi a lancé un dé : {}", lancer), 20);
        }

        // petit message si on fait une reussite ou un echec critique
        if lancer == 20 {
            AfficheTexte::affiche("REUSSITE CRITIQUE !!!".to_string(), 20);
        }
        if lancer == 1 {
            AfficheTexte::affiche("ECHEC CRITIQUE !!!".to_string(), 20);
        }
        lancer
    }
}