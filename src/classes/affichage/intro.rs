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
        //Changer ca quon on pourra recupe dans le fichier directement
        let choix_menu_principal = Choix::new("Nouvelle partie/Charger partie/Quitter".to_string());
        choix_menu_principal.lancer_choix();
    }
}