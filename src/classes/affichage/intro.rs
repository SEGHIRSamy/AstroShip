use crate::classes::affichage::affiche_texte::AfficheTexte;
use crate::classes::affichage::choix::Choix;

pub struct Intro {}

impl Intro {
    pub fn new() -> Intro {
        Intro {}
    }

    pub fn lancer_intro(&self) {
        let message = AfficheTexte::new(r#"
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
		"#.to_string());
        message.affiche(1);

        let choix_menu_principal = Choix::new("[1] Nouvelle partie/[2] Charger partie/[3] Quitter".to_string());
        choix_menu_principal.affiche();
    }
}