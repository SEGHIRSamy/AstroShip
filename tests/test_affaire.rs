#[cfg(test)]
mod tests {
    use astroship::classes::marchandage::objet::Objet;
    use astroship::classes::marchandage::affaire::Affaire;

    #[test]
    fn test_creation_affaire() {
        // Création d'un Objet
        let objet = Objet::new("Épée magique", "Description de l'épée",1);

        // Création d'une Affaire
        let affaire = Affaire::new(50000, objet, false, 10);

        // Vérification des valeurs
        assert_eq!(*affaire.get_prix(), 50000); // Test du prix
        assert_eq!(affaire.get_instance().get_nom(), "Épée magique"); // Test du nom de l'objet
        assert_eq!(affaire.get_instance().get_description(), "Description de l'épée"); // Test de la description de l'objet
        assert_eq!(*affaire.get_infini(), false); // Test de la valeur d'infini
        assert_eq!(*affaire.get_quantite(), 10); // Test de la quantité
    }

    #[test]
    fn test_modification_prix() {
        // Création de l'Affaire
        let objet = Objet::new("Bouclier", "Un bouclier puissant",1);
        let mut affaire = Affaire::new(10000, objet, false, 5);

        // Modification du prix
        affaire.set_prix(80000);

        // Vérification après modification
        assert_eq!(*affaire.get_prix(), 80000);
    }

    #[test]
    fn test_modification_instance() {
        // Création des objets
        let objet_initial = Objet::new("Potion", "Une potion rare",1);
        let objet_modifie = Objet::new("Potion de soin", "Sert à soigner les blessures",1);

        // Création de l'Affaire
        let mut affaire = Affaire::new(3000, objet_initial, false, 1);

        // Modification de l'objet dans l'affaire
        affaire.set_instance(objet_modifie);

        // Vérification après modification
        assert_eq!(affaire.get_instance().get_nom(), "Potion de soin");
        assert_eq!(affaire.get_instance().get_description(), "Sert à soigner les blessures");
    }

    #[test]
    fn test_modification_infini() {
        // Création de l'Affaire
        let objet = Objet::new("Pierre rare", "Une pierre précieuse et rare",1);
        let mut affaire = Affaire::new(100, objet, false, 50);

        // Modification de la valeur "infini"
        affaire.set_infini(true);

        // Vérification après modification
        assert_eq!(*affaire.get_infini(), true);
    }

    #[test]
    fn test_modification_quantite() {
        // Création de l'Affaire
        let objet = Objet::new("Arc", "Un arc de haute qualité",1);
        let mut affaire = Affaire::new(2000, objet, false, 10);

        // Modification de la quantité
        affaire.set_quantite(20);

        // Vérification après modification
        assert_eq!(*affaire.get_quantite(), 20);
    }
}
