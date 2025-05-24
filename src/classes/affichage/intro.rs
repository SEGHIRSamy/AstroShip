use crate::classes::gestion_evenement::quitter_jeu::QuitterJeu;
use crate::classes::gestion_evenement::lancer_partie::LancerPartie;
use crate::classes::gestion_evenement::evenement::Evenement;
use crate::classes::affichage::affiche_texte::AfficheTexte;
use crate::classes::gestion_evenement::charger_partie::ChargerPartie;
use crate::classes::gestion_evenement::choix::Choix;


#[allow(dead_code)]
pub struct Intro {}

#[allow(dead_code)]
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
            ("Nouvelle partie".to_string(), Box::new(LancerPartie::new())),
            ("Charger Partie".to_string(), Box::new(ChargerPartie::new())),
            ("Quitter".to_string(), Box::new(QuitterJeu::new()))
        ]);
        choix_menu_principal.lancer_choix();
    }
}
impl Evenement for Intro {
    fn action(&self) {
       self.lancer_intro();
    }
}