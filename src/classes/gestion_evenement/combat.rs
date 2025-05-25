use crate::classes::affichage::affiche_texte::AfficheTexte;
use crate::classes::gestion_evenement::lancer_dice::LancerDice;
use std::io::{self, Write};
use crate::classes::entite::personnage_principal::PersonnagePrincipal;

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
        Va lancer un combat, pour le moment on lance avec les stats bruts du joueur et de l'ennemi a remplacer quand on gerera les json
     */
    pub fn lancer_combat(
        intro: &str,
        mut pv_joueur: u32,
        attaque_joueur: u32,
        vitesse_joueur: u32,
        mut pv_ennemi: u32,
        attaque_ennemi: u32,
        vitesse_ennemi: u32,
    ) -> bool {
        AfficheTexte::affiche(intro.to_string(), 25);

        loop {
            std::thread::sleep(std::time::Duration::from_millis(1500));
            AfficheTexte::affiche(
                format!(
                    "\n--- Tour du joueur ---\nVos PV : {} | PV Ennemi : {}",
                    pv_joueur, pv_ennemi
                ),
                15,
            );
            AfficheTexte::affiche("[1] Attaquer".to_string(), 10);
            AfficheTexte::affiche("[2] Fuir".to_string(), 10);

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
                        return true;
                    } else {
                        AfficheTexte::affiche("❌ Vous n'avez pas réussi à fuir.".to_string(), 20);
                    }
                }
                _ => {
                    AfficheTexte::affiche("❗ Choix invalide !".to_string(), 20);
                    continue;
                }
            }

            if pv_ennemi == 0 {
                AfficheTexte::affiche("🎉 Ennemi vaincu !".to_string(), 20);;
                let charge_player = PersonnagePrincipal::charger_personnage_principal_depuis_json().unwrap();
                let mut update_player = PersonnagePrincipal::new(
                    charge_player.entite.get_nom(),
                    pv_joueur,
                    charge_player.entite.get_points_de_vie_max(),
                    charge_player.entite.get_force(),
                    charge_player.entite.get_intelligence(),
                    charge_player.entite.get_vitesse(),
                    charge_player.chance
                );
                update_player.inventaire.set_instance(charge_player.inventaire.get_instance().clone());
                PersonnagePrincipal::sauvegarder_personnage_principal_vers_json(&update_player).expect("Enregistrement Personnage échoué");
                return true;
            }

            std::thread::sleep(std::time::Duration::from_millis(1500));
            AfficheTexte::affiche("\n--- Tour de l'ennemi ---".to_string(), 20);
            let lancer = LancerDice::lancer_console_combat(false);

            let degats = Combat::calculer_degats(attaque_ennemi, attaque_joueur, lancer);
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
