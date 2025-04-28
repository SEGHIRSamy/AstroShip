mod classes;
use classes::affichage::intro::Intro;
use classes::spaciale::vaisseau::Vaisseau;
use classes::spaciale::planete::Planete;
use crate::classes::gestionEvenement::evenement::Evenement;

fn main() {
    Intro::new().action();

    // TEST VAISSEAU
    let terre = Planete::new("Terre", 0);
    let mars = Planete::new("Mars", 30);
    let jupiter = Planete::new("Jupiter", 120);

    // Création du vaisseau sur Terre
    let mut vaisseau = Vaisseau::new(100, 50, Some(terre));

    vaisseau.afficher_etat();

    // Tentative de voyage vers Mars
    vaisseau.voyager(&mars);

    // Tentative de voyage vers Jupiter
    vaisseau.voyager(&jupiter);

    vaisseau.afficher_etat();
}
