// tests/test_objet.rs

#[cfg(test)]
mod tests {
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

}
