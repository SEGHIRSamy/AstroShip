
#[cfg(test)]
mod tests {
    use astroship::classes::entite::personnage_principal::PersonnagePrincipal;
    use astroship::classes::entite::entite::Personnage; // Import du trait pour accéder à ses méthodes

    #[test]
    fn test_creation_personnage_principal() {
        let personnage = PersonnagePrincipal::new("Héros", 100, 100, 15, 10, 12, 5);
        assert_eq!(personnage.entite.get_nom(), "Héros");
        assert_eq!(personnage.entite.get_points_de_vie(), 100);
        assert_eq!(personnage.chance, 5);
    }

    #[test]
    fn test_augmentation_niveau_chance() {
        let mut personnage = PersonnagePrincipal::new("Héros", 100, 100, 15, 10, 12, 5);
        personnage.augmentation_niveau("chance");
        assert_eq!(personnage.chance, 6); // La chance doit augmenter
    }

    #[test]
    fn test_augmentation_niveau_force() {
        let mut personnage = PersonnagePrincipal::new("Héros", 100, 100, 15, 10, 12, 5);
        personnage.augmentation_niveau("force");
        assert_eq!(personnage.entite.get_force(), 16); // La force doit augmenter
        assert_eq!(personnage.entite.get_points_de_vie_max(), 105); // PV max + 5
        assert_eq!(personnage.entite.get_points_de_vie(), 105); // PV actuels correspondant à PV max
    }

    #[test]
    fn test_inventaire_ajout_monnaie() {
        let mut personnage = PersonnagePrincipal::new("Héros", 100, 100, 15, 10, 12, 5);
        personnage.inventaire.add_monnaie(1000);

        assert_eq!(personnage.inventaire.get_monnaie(), 1000);
    }

    #[test]
    fn test_inventaire_ajout_objet() {
        use astroship::classes::marchandage::objet::Objet;

        let mut personnage = PersonnagePrincipal::new("Héros", 100, 100, 15, 10, 12, 5);
        let objet = Objet::new("Épée légendaire", "épée de l'ancien héros de la galaxy",1);

        personnage.inventaire.add_objet(objet);
        assert_eq!(personnage.inventaire.get_instance().len(), 1); // L'inventaire contient 1 objet
    }
}