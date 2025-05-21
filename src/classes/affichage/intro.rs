use crate::classes::gestionEvenement::quitterJeu::quitterJeu;
use crate::classes::gestionEvenement::lancerPartie::lancerPartie;
use crate::classes::gestionEvenement::evenement::Evenement;
use crate::classes::affichage::affiche_texte::AfficheTexte;
use crate::classes::gestionEvenement::chargerPartie::chargerPartie;
use crate::classes::gestionEvenement::choix::Choix;


pub struct Intro {}

impl Intro {
    pub fn new() -> Intro {
        Intro {}
    }

    pub fn lancer_intro(&self) {
        AfficheTexte::affiche(r#"
                              /================================================================================\
                              ||   █████╗ ███████╗████████╗██████╗  ██████╗     ███████╗██╗  ██╗██╗██████╗    ||
                              ||  ██╔══██╗██╔════╝╚══██╔══╝██╔══██╗██╔═══██╗    ██╔════╝██║  ██║██║██╔══██╗   ||
                              ||  ███████║███████╗   ██║   ██████╔╝██║   ██║    ███████╗███████║██║██████╔╝   ||
                              ||  ██╔══██║╚════██║   ██║   ██╔══██╗██║   ██║    ╚════██║██╔══██║██║██╔═══╝    ||
                              ||  ██║  ██║███████║   ██║   ██║  ██║╚██████╔╝    ███████║██║  ██║██║██║        ||
                              ||  ╚═╝  ╚═╝╚══════╝   ╚═╝   ╚═╝  ╚═╝ ╚═════╝     ╚══════╝╚═╝  ╚═╝╚═╝╚═╝        ||
                              \================================================================================/
                                                                |
                                                               -+-
                                                              /-|-\
                                                              | O |
                                                              |   |
                                                             /-----\
                                                            |_______|
                                  "#.to_string(), 1);

        let choix_menu_principal =  Choix::new(vec![
            ("Nouvelle partie".to_string(), Box::new(lancerPartie::new())),
            ("Charger Partie".to_string(), Box::new(chargerPartie::new())),
            ("Quitter".to_string(), Box::new(quitterJeu::new()))
        ]);
        choix_menu_principal.lancer_choix();
    }
}
impl Evenement for Intro {
    fn action(&self) {
       self.lancer_intro();
    }
}