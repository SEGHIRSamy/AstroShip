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

        println!("Dégâts infligés : {}", degats);
        degats
    }


    pub fn tenter_fuite(vitesse_fuyard: u32, vitesse_adversaire: u32, lancer_de: u32) -> bool {
        let resultat = match lancer_de {
            1 => {
                println!("Échec critique : la fuite échoue automatiquement !");
                false
            },
            20 => {
                println!("Réussite critique : la fuite réussit automatiquement !");
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

                println!("Pour réussir la fuite, il faut faire {} ou plus au lancer de dé.", seuil);

                lancer_de as i32 >= seuil
            },
            _ => {
                panic!("Le lancer de dé doit être entre 1 et 20 !");
            }
        };

        if resultat {
            println!("La fuite a réussi !");
        } else {
            println!("La fuite a échoué !");
        }

        resultat
    }

}
