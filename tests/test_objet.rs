// tests/test_objet.rs

#[cfg(test)]
mod tests {
    use astroship::classes::entite::entite::Entite;
    use astroship::classes::marchandage::objet::Objet;

    #[test]
    fn test_creation_objet() {
        // Création d'un objet
        let objet = Objet::new("Épée magique", "Une épée puissante imprégnée de magie.",1);

        // Test des valeurs après création
        assert_eq!(objet.get_nom(), "Épée magique");
        assert_eq!(objet.get_description(), "Une épée puissante imprégnée de magie.");
    }

    #[test]
    fn test_set_description() {
        // Création d'un objet
        let mut objet = Objet::new("Bouclier", "Un bouclier robuste.",1);

        // Modification de la description
        objet.set_description("Un bouclier en métal renforcé.".to_string());

        // Vérification après modification
        assert_eq!(objet.get_description(), "Un bouclier en métal renforcé.");
    }

    #[test]
    fn test_set_nom() {
        // Création d'un objet
        let mut objet = Objet::new("Hache", "Une hache de combat.",1);

        // Modification du nom
        objet.set_nom("Hache de guerre".to_string());

        // Vérification après modification
        assert_eq!(objet.get_nom(), "Hache de guerre");
    }

    #[test]
    fn test_get_nom() {
        // Création d'un objet
        let objet = Objet::new("Arc", "Un arc en bois d'érable.",1);

        // Vérification du getter
        assert_eq!(objet.get_nom(), "Arc");
    }

    #[test]
    fn test_get_description() {
        // Création d'un objet
        let objet = Objet::new("Épée", "Une épée fine et légère.",1);

        // Vérification du getter
        assert_eq!(objet.get_description(), "Une épée fine et légère.");
    }
    #[test]
    fn test_objet_empty_text() {
        let mut objet = Objet::new("Nom initial", "Description initiale",1);
        objet.set_nom("".to_string());
        objet.set_description("".to_string());
        objet.set_quantite(1);
        assert_eq!(objet.get_nom(), "");
        assert_eq!(objet.get_description(), "");
        assert_eq!(objet.get_quantite(), 1);
    }
    #[test]
    fn test_objet_long_text() {
        let long_text = "a".repeat(1000);
        let objet = Objet::new(&long_text, &long_text,1);
        assert_eq!(objet.get_nom(), long_text);
        assert_eq!(objet.get_description(), long_text);
    }
    #[test]
    fn test_objet_getter_dynamique() {
        let mut objet = Objet::new("Initial", "Description",1);
        objet.set_nom("Nouveau nom".to_string());
        objet.set_description("Nouvelle description".to_string());
        objet.set_quantite(50);
        assert_eq!(objet.get_nom(), "Nouveau nom");
        assert_eq!(objet.get_description(), "Nouvelle description");
        assert_eq!(objet.get_quantite(), 50);

    }
    #[test]
    fn test_objet_identique() {
        let objet = Objet::new("Nom identique", "Nom identique",1);
        assert_eq!(objet.get_nom(), "Nom identique");
        assert_eq!(objet.get_description(), "Nom identique");

    }

    #[test]
    fn test_objet_new_empty() {
        let objet = Objet::new("", "",0);
        assert_eq!(objet.get_nom(), "");
        assert_eq!(objet.get_description(), "");
        assert_eq!(objet.get_quantite(), 0);

    }

    #[test]
    fn test_set_get_multiplicateurs() {
        let mut obj = Objet::new("Potion spéciale", "Augmente les stats", 1);

        obj.set_multiplicateur_pv(1.5);
        obj.set_multiplicateur_force(2.0);
        obj.set_multiplicateur_vitesse(1.2);

        assert_eq!(obj.get_multiplicateur_pv(), Some(1.5));
        assert_eq!(obj.get_multiplicateur_force(), Some(2.0));
        assert_eq!(obj.get_multiplicateur_vitesse(), Some(1.2));
    }

    #[test]
    fn test_consommer_applique_multiplicateurs() {
        let mut entite = Entite::new("Héros", 100, 100, 15, 10, 12);
        let mut obj = Objet::new("Boisson énergétique", "Boost temporaire", 1);

        obj.set_multiplicateur_pv(0.5);
        obj.set_multiplicateur_force(1.0);
        obj.set_multiplicateur_vitesse(0.0);
        obj.set_multiplicateur_pv_max(0.05);

        obj.consommer(&mut entite);

        assert_eq!(entite.get_points_de_vie(), 150);
        assert_eq!(entite.get_force(), 30);
        assert_eq!(entite.get_vitesse(), 12);
        assert_eq!(entite.get_points_de_vie_max(), 105);
    }

    #[test]
    fn test_consommer_partiel() {
        let mut entite = Entite::new("Héros", 100, 100, 15, 10, 12);
        let mut obj = Objet::new("Potion de vitesse", "Booste vitesse", 1);

        obj.set_multiplicateur_vitesse(2.0); // +100% → +25 = 50

        obj.consommer(&mut entite);

        assert_eq!(entite.get_points_de_vie(), 100); // inchangé
        assert_eq!(entite.get_force(), 15);         // inchangé
        assert_eq!(entite.get_vitesse(), 36);       // changé
        assert_eq!(entite.get_points_de_vie_max(), 100);       // changé
    }

    #[test]
    fn test_serialisation_objet_avec_multiplicateurs() {
        let mut obj = Objet::new("Potion ultime", "Multiboost", 3);
        obj.set_multiplicateur_pv(1.5);
        obj.set_multiplicateur_force(2.0);
        obj.set_multiplicateur_vitesse(0.8);

        let json = serde_json::to_string(&obj).expect("Erreur de sérialisation");

        // Vérifie que tous les champs sont bien présents
        assert!(json.contains("\"nom\":\"Potion ultime\""));
        assert!(json.contains("\"description\":\"Multiboost\""));
        assert!(json.contains("\"quantite\":3"));
        assert!(json.contains("\"multiplicateur_pv\":1.5"));
        assert!(json.contains("\"multiplicateur_force\":2.0"));
        assert!(json.contains("\"multiplicateur_vitesse\":0.8"));
    }





}
