use crate::classes::affichage::affiche_texte::AfficheTexte;
use crate::classes::gestion_evenement::lancer_dice::LancerDice;
use std::io::{self, Write};
use crate::classes::entite::entite::Personnage;
use crate::classes::entite::inventaire::Inventaire;
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
        intro: &str,
        mut pv_ennemi: u32,
        attaque_ennemi: u32,
        vitesse_ennemi: u32,
    ) -> bool {
        // Chargement initial des stats du joueur
        let sauvegarde: Sauvegarde = Sauvegarde::new();
        let mut charge_player : PersonnagePrincipal = sauvegarde.charge("personnage_principal.json".to_string()).unwrap();
        let mut pv_max : u32 = charge_player.entite.get_points_de_vie_max();
        let mut pv_joueur : u32 = charge_player.entite.get_points_de_vie();
        let mut attaque_joueur : u32 = charge_player.entite.get_force();
        let mut vitesse_joueur : u32 = charge_player.entite.get_vitesse();

        let mut update_player : PersonnagePrincipal;

        AfficheTexte::affiche(intro.to_string(), 25);

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
                    "Ennemi : PV : {} | Attaque : {} | Vitesse : {}\n",
                    pv_ennemi, attaque_ennemi, vitesse_ennemi
                ), 15,
            );
            AfficheTexte::affiche("[1] Attaquer".to_string(), 10);
            AfficheTexte::affiche("[2] Fuir".to_string(), 10);
            AfficheTexte::affiche("[3] Inventaire".to_string(), 10);

            print!("Votre choix : ");
            io::stdout().flush().unwrap();

            let mut choix = String::new();
            io::stdin().read_line(&mut choix).unwrap();

            match choix.trim() {
                // Attaquer
                "1" => {
                    let lancer = LancerDice::lancer_console_combat(true);
                    let degats =
                        Combat::calculer_degats(attaque_joueur, attaque_ennemi, lancer);
                    // On met a jour els pv de l'ennemi apres qu'on l'avoir attaqué
                    pv_ennemi = pv_ennemi.saturating_sub(degats);
                    AfficheTexte::affiche(
                        format!("Vous infligez {} dégâts. PV Ennemi restants : {}", degats, pv_ennemi),
                        15,
                    );
                }
                // Fuir
                "2" => {
                    let lancer = LancerDice::lancer_console_combat(true);

                    if Combat::tenter_fuite(vitesse_joueur, vitesse_ennemi, lancer) {
                        AfficheTexte::affiche("✅ Vous avez réussi à fuir !".to_string(), 20);
                        // On arrive a fuir : le combat est fini donc on save les stats et l'inventaire du perso dans le json
                        update_player = PersonnagePrincipal::new(
                            charge_player.entite.get_nom(),
                            pv_joueur,
                            pv_max,
                            attaque_joueur,
                            charge_player.entite.get_intelligence(),
                            vitesse_joueur,
                            charge_player.chance,
                            charge_player.get_uranium()
                        );
                        // #A_faire_combat gérer l'obtention du loot passif si on fuit le combat
                        update_player.inventaire.set_instance(charge_player.inventaire.get_instance().clone());

                        sauvegarde.sauvegarde("personnage_principal.json".to_string(), update_player).expect("Enregistrement Personnage échoué");
                        return true;
                    } else {
                        AfficheTexte::affiche("❌ Vous n'avez pas réussi à fuir.".to_string(), 20);
                    }
                }
                // Inventaire
                "3" => {
                    let a_consomme = charge_player.inventaire.afficher_inventaire_interactif();
                    // Si on a rien consommé dans l'inventaire on repart dans la boucle sans sauvegarder les stats dans le json
                    if !a_consomme { continue }
                    // Si on a consommé alors on save les stats du perso et son inventaire
                    charge_player = sauvegarde.charge("personnage_principal.json".to_string()).unwrap();
                    pv_max = charge_player.entite.get_points_de_vie_max();
                    pv_joueur = charge_player.entite.get_points_de_vie();
                    attaque_joueur = charge_player.entite.get_force();
                    vitesse_joueur = charge_player.entite.get_vitesse();
                }
                _ => {
                    AfficheTexte::affiche("❗ Choix invalide !".to_string(), 20);
                    continue;
                }
            }

            if pv_ennemi == 0 {
                AfficheTexte::affiche("🎉 Ennemi vaincu !".to_string(), 20);
                update_player = PersonnagePrincipal::new(
                    charge_player.entite.get_nom(),
                    pv_joueur,
                    pv_max,
                    attaque_joueur,
                    charge_player.entite.get_intelligence(),
                    vitesse_joueur,
                    charge_player.chance,
                    charge_player.get_uranium()
                );
                // #A_faire_combat gérer l'obtention du loot hostile quand on tue l'ennemi
                update_player.inventaire.set_instance(charge_player.inventaire.get_instance().clone());

                sauvegarde.sauvegarde("personnage_principal.json".to_string(), update_player).expect("Enregistrement Personnage échoué");
                return true;
            }

            std::thread::sleep(std::time::Duration::from_millis(1500));
            AfficheTexte::affiche("\n--- Tour de l'ennemi ---".to_string(), 20);
            let lancer = LancerDice::lancer_console_combat(false);

            let degats = Combat::calculer_degats(attaque_ennemi, attaque_joueur, lancer);
            // L'ennemi nous a tpé donc on met les pv du joueur a jour de maniere locale
            pv_joueur = pv_joueur.saturating_sub(degats);
            AfficheTexte::affiche(
                format!(
                    "L'ennemi vous inflige {} dégâts. Vos PV restants : {}",
                    degats, pv_joueur
                ),
                15,
            );

            if pv_joueur == 0 {
                AfficheTexte::affiche("💀 Vous êtes vaincu...".to_string(), 20);
                return false;
            }
        }
    }

}
