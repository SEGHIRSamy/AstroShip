#[cfg(test)]
mod tests {
    use astroship::classes::entite::personnage_principal::PersonnagePrincipal;
    use astroship::classes::planete::auberge::Auberge;


    /// Tester le cas où le personnage n'a pas assez d'argent pour se reposer
    #[test]
    fn test_pas_assez_d_argent() {
        let mut personnage = PersonnagePrincipal::new("Héros", 50, 100, 10, 10, 10, 5,0,0,"plante".to_string());
        personnage.inventaire.set_monnaie(10); // Le personnage a seulement 10 pièces

        let auberge = Auberge::new(30, vec!["test".to_string()]); // Prix du repos : 30 pièces

        auberge.proposer_repos_test(&mut personnage, Some(1)); // Simuler que le joueur accepte

        // Les points de vie ne doivent pas changer
        assert_eq!(personnage.entite.get_points_de_vie(), 50);
        // La monnaie reste inchangée
        assert_eq!(personnage.inventaire.get_monnaie(), 10);
    }

    /// Tester le cas où le personnage est déjà en pleine santé
    #[test]
    fn test_deja_en_pleine_sante() {
        let mut personnage = PersonnagePrincipal::new("Héros", 100, 100, 10, 10, 10, 5,0,0,"plante".to_string()); // PV déjà max
        personnage.inventaire.set_monnaie(100); // Le personnage a assez de monnaie

        let auberge = Auberge::new(30, vec!["test".to_string()]); // Prix du repos : 30 pièces

        auberge.proposer_repos_test(&mut personnage, Some(1)); // Simuler que le joueur souhaite se reposer

        // Les points de vie restent au maximum
        assert_eq!(personnage.entite.get_points_de_vie(), 100);
        // La monnaie ne doit pas être déduite
        assert_eq!(personnage.inventaire.get_monnaie(), 100);
    }

    /// Tester le cas où le repos est réussi (le personnage paie et est soigné)
    #[test]
    fn test_repos_reussi() {
        let mut personnage = PersonnagePrincipal::new("Héros", 50, 100, 10, 10, 10, 5,0,0,"plante".to_string());
        personnage.inventaire.set_monnaie(50); // Le personnage a 50 pièces

        let auberge = Auberge::new(30, vec!["test".to_string()]); // Prix du repos : 30 pièces

        auberge.proposer_repos_test(&mut personnage, Some(1)); // Simuler que le joueur accepte

        // Les points de vie doivent être restaurés à leur maximum
        assert_eq!(personnage.entite.get_points_de_vie(), 100);
        // La monnaie restante doit être correcte : 50 - 30 = 20
        assert_eq!(personnage.inventaire.get_monnaie(), 20);
    }

    /// Tester le cas où le personnage refuse le repos
    #[test]
    fn test_refus_repos() {
        let mut personnage = PersonnagePrincipal::new("Héros", 50, 100, 10, 10, 10, 5,0,0,"plante".to_string());
        personnage.inventaire.set_monnaie(50); // Le personnage a 50 pièces

        let auberge = Auberge::new(30, vec!["test".to_string()]); // Prix du repos : 30 pièces

        auberge.proposer_repos_test(&mut personnage, Some(2)); // Simuler que le joueur refuse

        // Les points de vie ne doivent pas changer
        assert_eq!(personnage.entite.get_points_de_vie(), 50);
        // La monnaie reste inchangée
        assert_eq!(personnage.inventaire.get_monnaie(), 50);
    }
}