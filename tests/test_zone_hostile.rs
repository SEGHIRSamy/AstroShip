#[cfg(test)]
mod tests {
    use serde_json::to_string;
    use astroship::classes::marchandage::butin::Butin;
    use astroship::classes::entite::ennemie::Ennemi;
    use astroship::classes::marchandage::butin::Rarete;
    use astroship::classes::marchandage::objet::Objet;
    use astroship::classes::planete::zone_hostile::ZoneHostile;


    fn creer_ennemi_mock(nom: &str) -> Ennemi {
        let or = Objet::new("Or", "Une pièce d'or brillante.",1);
        let potion = Objet::new("Potion", "Une potion de soin.",1);
        let butin_or = Butin::new(or, 10, 1.0, Rarete::Commun);
        let butin_potion = Butin::new(potion, 1, 1.0, Rarete::Rare);

        let ennemi = Ennemi::new(
            nom,
            50,
            50,
            10,
            5,
            3,
            vec![butin_or.clone()],
            vec![butin_potion.clone()],
            "Intro".to_string(),
            "Attaque".to_string(),
        );
        ennemi
    }

    #[test]
    fn test_explorer_auto_interruption_apres_premier() {
        let ennemis = vec![
            creer_ennemi_mock("Gobelin", ),
            creer_ennemi_mock("Orc", ),
        ];
        let mut zone = ZoneHostile::new("Zone Test", ennemis);

        // on arrête après le premier ennemi
        let butins = zone.explorer_auto(|i| i == 0);

        assert_eq!(butins, vec!["Or","Or"]);
    }

    #[test]
    fn test_zone_hostile_creation() {
        let ennemi = creer_ennemi_mock("Bob de l'espace");
        let zone = ZoneHostile::new("Mars sombre", vec![ennemi]);
        assert_eq!(zone.get_nom(), "Mars sombre");
    }
}
