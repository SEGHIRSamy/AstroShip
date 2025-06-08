use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;
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

    pub fn lancer_outro() {
        let vaisseau = r#"
                            |
                           -+-
                          /-|-\
                          | O |
                          |   |
                         /-----\
                        |_______|
"#;

        let frames = vec![
            r#"
                                                 *        *

                           *           *                   *
       *       *                           *       *

"#,
            r#"
                          *      *                 *         *
       *       *              *           *              *

                        *               *         *
"#,
            r#"
           *          *     *         *      *      *       *

                     *       *        *          *        *

"#,
            r#"
                     *     *     *     *     *     *     *
                                            *     *     *     *     *     *     *
                         *     *     *     *     *     *     *
"#,
            r#"



"#,r#"
                         *
          *                             *
                    *         *
       *      *           *         *      *
                            *
    *         *   *      *       *        *
               *                  *

               *                  *
    *        *     *     *     *     *
         *      *         *       *
   *               *                  *
"#,
            r#"
          *          *         *
       *       *          *         *      *



       *          *         *       *    *
   *         *         *       *        *
"#,
        ];
        AfficheTexte::affiche("Vous avez réussi à accumuler assez d'uranium, vous pressez sur le bouton d'hyper espace pour commencer votre voyage retour".to_string(), 25);
        thread::sleep(Duration::from_millis(5000));

        for _ in 0..5 {
            for stars in &frames {
                print!("\x1B[2J\x1B[1;1H"); // Clear screen
                println!("{}{}\n{}", stars, stars,vaisseau);
                stdout().flush().unwrap();
                thread::sleep(Duration::from_millis(300));
            }
        }

        println!("Entrée réussie dans l'hyperespace !");
        println!("Vous apercevez votre planète, Astro ne peut s'emppêcher d'être ému en disant que \nc'est les émotions !");
    }
}
impl Evenement for Intro {
    fn action(&self) {
       self.lancer_intro();
    }
}