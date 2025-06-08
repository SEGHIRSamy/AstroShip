#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use astroship::classes::entite::personnage_principal::PersonnagePrincipal;
    use astroship::classes::planete::auberge::Auberge;

    fn creer_personnage(pv: u32, pv_max: u32, monnaie: u32) -> Rc<RefCell<PersonnagePrincipal>> {
        let mut perso = PersonnagePrincipal::new("Héros", pv, pv_max, 10, 10, 10, 5, 0);
        perso.inventaire.set_monnaie(monnaie);
        Rc::new(RefCell::new(perso))
    }


    /// Tester le cas où le personnage n'a pas assez d'argent pour se reposer
    #[test]
    fn test_pas_assez_d_argent() {
        let personnage = creer_personnage(50, 100, 10); // Le personnage a seulement 10 pièces

        let auberge = Auberge::new(30, vec!["test".to_string(), "test".to_string()]); // Prix du repos : 30 pièces

        auberge.proposer_repos(Rc::clone(&personnage), Some(1)); // Simuler que le joueur accepte

        let perso =  personnage.borrow();
        // Les points de vie ne doivent pas changer
        assert_eq!(perso.entite.get_points_de_vie(), 50);
        // La monnaie reste inchangée
        assert_eq!(perso.inventaire.get_monnaie(), 10);
    }

    /// Tester le cas où le personnage est déjà en pleine santé
    #[test]
    fn test_deja_en_pleine_sante() {
        let personnage = creer_personnage(100, 100, 100); // Le personnage a assez de monnaie

        let auberge = Auberge::new(30, vec!["test".to_string(), "test".to_string()]); // Prix du repos : 30 pièces

        auberge.proposer_repos(Rc::clone(&personnage), Some(1)); // Simuler que le joueur accepte

        let perso =  personnage.borrow();
        // Les points de vie ne doivent pas changer
        assert_eq!(perso.entite.get_points_de_vie(), 100);
        // La monnaie reste inchangée
        assert_eq!(perso.inventaire.get_monnaie(), 100);
    }

    /// Tester le cas où le repos est réussi (le personnage paie et est soigné)
    #[test]
    fn test_repos_reussi() {
        let personnage = creer_personnage(50, 100, 50); // Le personnage a 50 pièces

        let auberge = Auberge::new(30, vec!["test".to_string(), "test".to_string()]); // Prix du repos : 30 pièces

        auberge.proposer_repos(Rc::clone(&personnage), Some(1)); // Simuler que le joueur accepte

        let perso =  personnage.borrow();
        // Les points de vie doivent être restaurés à leur maximum
        assert_eq!(perso.entite.get_points_de_vie(), 100);
        // La monnaie restante doit être correcte : 50 - 30 = 20
        assert_eq!(perso.inventaire.get_monnaie(), 20);
    }

    /// Tester le cas où le personnage refuse le repos
    #[test]
    fn test_refus_repos() {
        let personnage = creer_personnage(50, 100, 50); // Le personnage a 50 pièces

        let auberge = Auberge::new(30, vec!["test".to_string(), "test".to_string()]); // Prix du repos : 30 pièces

        auberge.proposer_repos(Rc::clone(&personnage), Some(2)); // Simuler que le joueur accepte // Simuler que le joueur refuse

        let perso =  personnage.borrow();

        // Les points de vie ne doivent pas changer
        assert_eq!(perso.entite.get_points_de_vie(), 50);
        // La monnaie reste inchangée
        assert_eq!(perso.inventaire.get_monnaie(), 50);
    }
}