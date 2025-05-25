mod classes;
use classes::affichage::intro::Intro;
use crate::classes::gestion_evenement::evenement::Evenement;
use classes::gestion_evenement::combat::Combat;

fn main() {
    Intro::new().action();


    // Test de la fonction combat temporaire dans le main car les tests ne print qu'a la fin du test
    // Statistiques de base
    let joueur_pv = 100;
    let joueur_attaque = 30;
    let joueur_vitesse = 15;

    let ennemi_pv = 80;
    let ennemi_attaque = 35;
    let ennemi_vitesse = 10;

    let intro = "Un terrible ennemi apparaît ! Préparez-vous au combat !";

    let resultat = Combat::lancer_combat(
        intro,
        joueur_pv,
        joueur_attaque,
        joueur_vitesse,
        ennemi_pv,
        ennemi_attaque,
        ennemi_vitesse,
    );

    println!();
    if resultat {
        println!("✅ Le combat s'est terminé avec votre victoire (ou fuite réussie) !");
    } else {
        println!("❌ Vous avez été vaincu...");
    }
}
