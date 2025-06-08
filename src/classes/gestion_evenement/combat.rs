use crate::classes::affichage::affiche_texte::AfficheTexte;
use crate::classes::gestion_evenement::action_combat::attaquer::Attaquer;
use crate::classes::gestion_evenement::action_combat::fuir::Fuir;
use crate::classes::gestion_evenement::action_combat::inventaire_interaction::InventaireInteraction;
use crate::classes::gestion_evenement::choix::Choix;
use crate::classes::gestion_evenement::lancer_dice::LancerDice;
use std::cell::RefCell;
use std::rc::Rc;
use rand::rng;
use crate::classes::entite::ennemie::Ennemi;
use crate::classes::entite::personnage_principal::PersonnagePrincipal;
use crate::classes::sauvegarde::sauvegarde::Sauvegarde;

#[allow(dead_code)]
pub struct Combat;
#[allow(dead_code)]
impl Combat {
    pub fn calculer_degats(attaque_attaquant: u32, attaque_defenseur: u32, lancer_de: u32) -> u32 {
        let degats = match lancer_de {
            1 => 1, // Échec critique
            20 => 999_999, // Réussite critique
            2..=19 => {
                let multiplicateur = 0.5 + (lancer_de as f32 / 20.0);
                let base = attaque_attaquant as f32;
                let defense = attaque_defenseur as f32;
                let reduction = defense / (defense + 50.0); // plus la défense est haute, plus elle réduit
                let degats_calc = (base * (1.0 - reduction) * multiplicateur).round() as u32;
                degats_calc.max(1)
            },
            _ => {
                panic!("Le lancer de dé doit être entre 1 et 20 !");
            }
        };

        degats
    }


    pub fn tenter_fuite(vitesse_fuyard: u32, vitesse_adversaire: u32, lancer_de: u32) -> bool {
        let resultat = match lancer_de {
            1 => {
                false
            },
            20 => {
                true
            },
            2..=19 => {
                let diff_vitesse = vitesse_fuyard as i32 - vitesse_adversaire as i32;
                let mut seuil = 10 - diff_vitesse / 2;

                if seuil < 5 {
                    seuil = 5;
                } else if seuil > 18 {
                    seuil = 18;
                }

                AfficheTexte::affiche(format!("Pour réussir la fuite, il faut faire {} ou plus au lancer de dé.", seuil), 20);

                lancer_de as i32 >= seuil
            },
            _ => {
                panic!("Le lancer de dé doit être entre 1 et 20 !");
            }
        };

        resultat
    }


    // Lancer un combat
    /*
        Fonction de combat, on rcupere les stats du joueur que l'on charge dans les variables
        Si on fuit ou qu'on tue l'ennemi on sauvegarde ces variables de stats dans le json
        A la fin du passgae de l'inventaire on doit recharger les stats du joueur si il a consommé des objets
     */
    pub fn lancer_combat(
        ennemi: &mut Ennemi
    ) -> bool {
        // Chargement initial des stats du joueur
        let sauvegarde: Sauvegarde = Sauvegarde::new();
        let mut charge_player : PersonnagePrincipal = sauvegarde.charge("personnage_principal.json".to_string()).unwrap();
        let mut pv_max : u32 = charge_player.entite.get_points_de_vie_max();
        let mut pv_joueur : u32 = charge_player.entite.get_points_de_vie();
        let mut attaque_joueur : u32 = charge_player.entite.get_force();
        let mut vitesse_joueur : u32 = charge_player.entite.get_vitesse();

        let mut update_player : PersonnagePrincipal;

        // on va calculer les stats de l'ennemi en fonction de celles du joueur
        let pourcentage_amelioration = Combat::pourcentage_amelioration_joueur(pv_max, attaque_joueur, vitesse_joueur);
        let mut pv_ennemi = (ennemi.get_base().get_points_de_vie() as f64 * pourcentage_amelioration) as u32;
        let pv_max_ennemi = pv_ennemi;
        let attaque_ennemi = (ennemi.get_base().get_force() as f64 * pourcentage_amelioration) as u32;
        let vitesse_ennemi = (ennemi.get_base().get_vitesse() as f64 * pourcentage_amelioration) as u32;

        AfficheTexte::affiche(ennemi.phrase_intro.to_string(), 25);

        // Boucle du combat
        loop {
            std::thread::sleep(std::time::Duration::from_millis(1500));
            AfficheTexte::affiche(
                format!(
                    "\n--- Tour du joueur ---\n",
                ),
                15,
            );
            AfficheTexte::affiche(
                format!(
                    "Vous : PV : {}/{} | Attaque : {} | Vitesse : {}\n",
                    pv_joueur, pv_max, attaque_joueur, vitesse_joueur
                ), 15,
            );
            AfficheTexte::affiche(
                format!(
                    "{} : PV : {}/{} | Attaque : {} | Vitesse : {}\n",
                    ennemi.get_base().get_nom(), pv_ennemi, pv_max_ennemi, attaque_ennemi, vitesse_ennemi
                ), 15,
            );
            let charge_player_rc = Rc::new(RefCell::new(charge_player.clone()));
            let stop = Rc::new(RefCell::new(false));
            let consome = Rc::new(RefCell::new(true));
            let pv_ennemi_rc = Rc::new(RefCell::new(pv_ennemi));

            // AfficheTexte::affiche("[1] Attaquer".to_string(), 10);
            // AfficheTexte::affiche("[2] Fuir".to_string(), 10);
            // AfficheTexte::affiche("[3] Inventaire".to_string(), 10);

            // print!("Votre choix : ");
            // io::stdout().flush().unwrap();

            // let mut choix = String::new();
            // io::stdin().read_line(&mut choix).unwrap();

            // match choix.trim() {
            //     // Attaquer
            //     "1" => {
            //         let lancer = LancerDice::lancer_console_combat(true);
            //         let degats =
            //             Combat::calculer_degats(attaque_joueur, attaque_ennemi, lancer);
            //         // On met a jour els pv de l'ennemi apres qu'on l'avoir attaqué
            //         let nouvelle_vie = if degats > pv_ennemi {
            //             0
            //         } else {
            //             pv_ennemi - degats
            //         };
            //         //ennemi.base.set_points_de_vie(nouvelle_vie);
            //         pv_ennemi = nouvelle_vie;
            //         AfficheTexte::affiche(
            //             format!("Vous infligez {} dégâts. PV Ennemi restants : {}", degats, pv_ennemi),
            //             15,
            //         );
            //     }
            //     // Fuir
            //     "2" => {
            //         let lancer = LancerDice::lancer_console_combat(true);
            //         if Combat::tenter_fuite(vitesse_joueur, vitesse_ennemi, lancer) {
            //             AfficheTexte::affiche("✅ Vous avez réussi à fuir !".to_string(), 20);
            //             // On arrive a fuir : le combat est fini donc on save les stats et l'inventaire du perso dans le json
            //             update_player = PersonnagePrincipal::new(
            //                 charge_player.entite.get_nom(),
            //                 pv_joueur,
            //                 pv_max,
            //                 attaque_joueur,
            //                 charge_player.entite.get_intelligence(),
            //                 vitesse_joueur,
            //                 charge_player.chance,
            //                 charge_player.get_uranium(),
            //                 charge_player.get_carburant(),
            //                 charge_player.get_planete_nom().to_string(),
            //             );
            //             let mut rng = rng();
            //             for obj in ennemi.interaction(&mut rng) {
            //                 charge_player.inventaire.add_objet(obj.get_objet());
            //                 obj.get_objet().afficher()
            //             }
            //             update_player.inventaire.add_monnaie(charge_player.inventaire.get_monnaie()+ennemi.get_monnaie().clone());
            //             update_player.inventaire.set_instance(charge_player.inventaire.get_instance().clone());
            //             sauvegarde.sauvegarde("personnage_principal.json".to_string(), update_player).expect("Enregistrement Personnage échoué");
            //             return true;
            //         } else {
            //             AfficheTexte::affiche("❌ Vous n'avez pas réussi à fuir.".to_string(), 20);
            //         }
            //     }
            //     // Inventaire
            //     "3" => {
            //         let a_consomme = charge_player.inventaire.afficher_inventaire_interactif();
            //         // Si on a rien consommé dans l'inventaire on repart dans la boucle sans sauvegarder les stats dans le json
            //         if !a_consomme { continue }
            //         // Si on a consommé alors on save les stats du perso et son inventaire
            //         charge_player = sauvegarde.charge("personnage_principal.json".to_string()).unwrap();
            //         pv_max = charge_player.entite.get_points_de_vie_max();
            //         pv_joueur = charge_player.entite.get_points_de_vie();
            //         attaque_joueur = charge_player.entite.get_force();
            //         vitesse_joueur = charge_player.entite.get_vitesse();
            //     }
            //     _ => {
            //         AfficheTexte::affiche("❗ Choix invalide !".to_string(), 20);
            //         continue;
            //     }
            // }

            let attaquer = Box::new(Attaquer::new(Rc::clone(&pv_ennemi_rc), attaque_joueur.clone(), attaque_ennemi.clone()));
            let fuir = Box::new(Fuir::new(Rc::clone(&stop), Rc::clone(&charge_player_rc), attaque_joueur.clone(), vitesse_joueur.clone(), pv_max.clone(), pv_joueur.clone(), ennemi.clone(), vitesse_ennemi.clone()));
            let inventaire = Box::new(InventaireInteraction::new(Rc::clone(&consome), Rc::clone(&charge_player_rc)));

            let mut choix = Choix::new(vec![
                ("Attaquer".to_string(), attaquer),
                ("Fuir".to_string(), fuir),
                ("Inventaire".to_string(), inventaire),
            ]);

            choix.lancer_choix();
            drop(choix);

            if !*consome.borrow() { continue }
            if *stop.borrow() { return true }
            pv_ennemi = *pv_ennemi_rc.borrow();

            // Récupérer les données modifiées
            charge_player = Rc::try_unwrap(charge_player_rc)
                .expect("Personnage encore emprunté ailleurs")
                .into_inner();

            if pv_ennemi == 0 {
                AfficheTexte::affiche("🎉 Ennemi vaincu !\n".to_string(), 20);
                update_player = PersonnagePrincipal::new(
                    charge_player.entite.get_nom(),
                    pv_joueur,
                    pv_max,
                    attaque_joueur,
                    charge_player.entite.get_intelligence(),
                    vitesse_joueur,
                    charge_player.chance,
                    charge_player.get_uranium(),
                    charge_player.get_carburant(),
                    charge_player.get_planete_nom().to_string()
                );

                let mut rng = rng();
                for obj in ennemi.interaction(&mut rng) {
                    charge_player.inventaire.add_objet(obj.get_objet());
                    obj.get_objet().afficher()
                }
                update_player.inventaire.add_monnaie(charge_player.inventaire.get_monnaie()+ennemi.get_monnaie().clone());
                update_player.inventaire.set_instance(charge_player.inventaire.get_instance().clone());
                sauvegarde.sauvegarde("personnage_principal.json".to_string(), update_player).expect("Enregistrement Personnage échoué");
                return true;
            }

            pv_max = charge_player.entite.get_points_de_vie_max();
            pv_joueur = charge_player.entite.get_points_de_vie();
            attaque_joueur = charge_player.entite.get_force();
            vitesse_joueur = charge_player.entite.get_vitesse();

            std::thread::sleep(std::time::Duration::from_millis(1500));
            AfficheTexte::affiche("\n--- Tour de l'ennemi ---".to_string(), 20);
            let lancer = LancerDice::lancer_console_combat(false);

            let degats = Combat::calculer_degats(attaque_ennemi, attaque_joueur, lancer);
            // L'ennemi nous a tapé donc on met les pv du joueur a jour de maniere locale
            pv_joueur = pv_joueur.saturating_sub(degats);
            AfficheTexte::affiche(
                format!(
                    "{} Vous subissez {} dégâts. Vos PV restants : {}",
                    ennemi.phrase_attaque, degats, pv_joueur
                ),
                15,
            );

            if pv_joueur == 0 {
                AfficheTexte::affiche("💀 Vous êtes vaincu...".to_string(), 20);
                return false;
            }
        }
    }


    // --- Fonction pour calculer adapter les stats de l'ennemi a celles du joueur ---
    // la fonction va calculer la difference entre les bases stats par defaut du jouer et ses bases stats actuelles
    // ensuite on va recupere un pourcentage que l'on va adapter aux bases stats de l'ennmi pour y appliquer
    // une augmentation proportionelle a celle que les stats du joueur ont subies
    fn pourcentage_amelioration_joueur(pv_max_joueur: u32, force_joueur: u32, vitesse_joueur: u32) -> f64 {
        // les bases stats cumulées du joueur sans amélioration
        let base_stat:u32 = 90;
        // les nouvelles stats du joueur
        let nv_stats:u32 = pv_max_joueur + force_joueur + vitesse_joueur;

        nv_stats as f64 / base_stat as f64
    }

}
