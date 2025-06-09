#[cfg(test)]
mod tests {

    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;
    use astroship::classes::entite::ennemie::Ennemi;
    use astroship::classes::marchandage::butin::{Butin, Rarete};
    use astroship::classes::marchandage::objet::Objet;

    #[test]
    fn test_creation_ennemi() {
        let or = Objet::new("Or", "Une pièce d'or brillante.",1);
        let potion = Objet::new("Potion", "Une potion de soin.",1);
        let butin_or = Butin::new(or, 10, 0.8, Rarete::Commun);
        let butin_potion = Butin::new(potion, 1, 0.3, Rarete::Rare);

        let ennemi = Ennemi::new(
            "Gobelin",
            50,
            50,
            10,
            5,
            3,
            vec![butin_or.clone()],
            vec![butin_potion.clone()],
            "Intro".to_string(),
            "Attaque".to_string(),
            10
        );

        assert_eq!(ennemi.get_base().get_nom(), "Gobelin");
        assert_eq!(ennemi.get_base().get_points_de_vie(), 50);
        assert_eq!(ennemi.get_base().get_points_de_vie_max(), 50);
        assert_eq!(ennemi.get_base().get_force(), 10);
        assert_eq!(ennemi.get_base().get_intelligence(), 5);
        assert_eq!(ennemi.get_base().get_vitesse(), 3);
        assert_eq!(ennemi.butins_passifs.len(), 1);
        assert_eq!(ennemi.butins_hostiles.len(), 1);
    }

    #[test]
    fn test_interaction_epargne() {
        let or = Objet::new("Or", "Une pièce d'or brillante.",1);
        let potion = Objet::new("Potion", "Une potion de soin.",1);
        let butin_or = Butin::new(or.clone(), 10, 1.0, Rarete::Commun);
        let butin_potion = Butin::new(potion.clone(), 1, 0.0, Rarete::Rare);

        let ennemi = Ennemi::new(
            "Bandit",
            30,
            30,
            8,
            4,
            2,
            vec![butin_or.clone(), butin_potion.clone()],
            vec![],
            "intro".to_string(),
            "Attaque".to_string(),
            10
        );

        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let butins = ennemi.interaction(&mut rng);
        for butin in &butins {
            println!("Vous avez obtenu : {} x{}", butin.objet.get_nom(), butin.quantite);
        }

        assert_eq!(
            butins.iter().any(|b| b.objet.get_nom() == "Or"),
            true,
            "L'or aurait dû être obtenu."
        );
        assert_eq!(
            butins.iter().any(|b| b.objet.get_nom() == "Potion"),
            false,
            "La potion ne devrait pas être obtenue (probabilité de 0 %)."
        );
    }

    #[test]
    fn test_interaction_tuer() {
        let or = Objet::new("Or", "Une pièce d'or brillante.",1);
        let epee = Objet::new("Épée", "Une épée rouillée.",1);
        let butin_or = Butin::new(or.clone(), 10, 0.8, Rarete::Commun);
        let butin_epee = Butin::new(epee.clone(), 1, 0.5, Rarete::Rare);

        let ennemi = Ennemi::new(
            "Orc",
            0, // Pas de points de vie : ennemi vaincu
            60,
            15,
            3,
            5,
            vec![],
            vec![butin_or.clone(), butin_epee.clone()],
            "intro".to_string(),
            "Attaque".to_string(),
            10
        );

        // Utilisation d'un générateur déterministe
        let mut rng = ChaCha8Rng::seed_from_u64(12345);

        // Appel de la méthode interaction
        let butins = ennemi.interaction(&mut rng);

        println!("Butins hostiles obtenus :");
        for butin in &butins {
            println!("  => {} x{}", butin.objet.get_nom(), butin.quantite);
        }

        // Vérifiez l'épée
        assert_eq!(
            butins.iter().any(|b| b.objet.get_nom() == "Épée"),
            true,
            "L'épée devrait être obtenue avec une probabilité déterministe de 0.5."
        );

        // Vérifiez l'or
        assert_eq!(
            butins.iter().any(|b| b.objet.get_nom() == "Or"),
            true,
            "L'or devrait être obtenu avec une probabilité déterministe de 0.8."
        );
    }


    #[test]
    fn test_determination_butins_deterministes() {
        let epee = Objet::new("Épée", "Une épée ancienne, mais encore tranchante.",1);
        let bouclier = Objet::new("Bouclier", "Un bouclier robuste, orné d'inscriptions mystérieuses.",1);
        let butin_epee = Butin::new(epee.clone(), 1, 0.6, Rarete::Rare);
        let butin_bouclier = Butin::new(bouclier.clone(), 1, 0.4, Rarete::Rare);

        let ennemi = Ennemi::new(
            "Guerrier Squelette",
            0, // Ennemi vaincu au départ (important pour générer un butin hostile)
            100,
            14,
            6,
            7,
            vec![],
            vec![butin_epee.clone(), butin_bouclier.clone()],
            "Intro".to_string(),
            "Attaque".to_string(),
            10
        );

        // Utilisation d'un générateur déterministe
        let mut rng = ChaCha8Rng::seed_from_u64(42);

        // Appel de la méthode `interaction` via le générateur déterministe
        let butins = ennemi.interaction(&mut rng);

        println!("Butins hostiles obtenus :");
        for butin in &butins {
            println!("  => {} x{}", butin.objet.get_nom(), butin.quantite);
        }

        // Vérifiez que l'épée est obtenue mais pas le bouclier
        assert_eq!(
            butins.iter().any(|b| b.objet.get_nom() == "Épée"),
            true,
            "L'épée aurait dû être obtenue selon la probabilité déterministe."
        );
        assert_eq!(
            butins.iter().any(|b| b.objet.get_nom() == "Bouclier"),
            false,
            "Le bouclier ne devrait pas être obtenu selon la probabilité déterministe."
        );
    }

    #[test]
    fn test_interaction_par_defaut() {
        let butin1 = Butin::new(Objet::new("Or", "Une pièce d'or",1), 10, 1.0, Rarete::Commun); // Probabilité 100%
        let butin2 = Butin::new(Objet::new("Épée", "Une épée rouillée",1), 1, 0.0, Rarete::Rare); // Probabilité 0%

        let ennemi = Ennemi::new(
            "Orc",
            0, // Pas de points de vie => Ennemi vaincu
            100, // Points de vie max
            15, // Force
            10, // Intelligence
            3,  // Vitesse
            vec![], // Aucun butin passif
            vec![butin1, butin2], // Butins hostiles
            "intro".to_string(),
            "Attaque".to_string(),
            10
        );

        let butins = ennemi.interaction_par_defaut();

        // Vérifier qu'avec les probabilités 1.0 et 0.0, seuls certains résultats sont possibles
        assert!(butins.iter().any(|b| b.objet.get_nom() == "Or"));
        assert!(!butins.iter().any(|b| b.objet.get_nom() == "Épée"));
    }

}
