#[cfg(test)]
mod tests {
    use astroship::classes::entite::inventaire::Inventaire;
    use astroship::classes::marchandage::objet::Objet;

    #[test]
    fn test_creation_inventaire_vide() {
        let inventaire = Inventaire::new();
        assert_eq!(inventaire.get_monnaie(), 0);
        assert!(inventaire.get_instance().is_empty());
    }

    #[test]
    fn test_ajout_monnaie() {
        let mut inventaire = Inventaire::new();
        inventaire.add_monnaie(100);
        assert_eq!(inventaire.get_monnaie(), 100);

        // Ajout supplémentaire de monnaie
        inventaire.add_monnaie(50);
        assert_eq!(inventaire.get_monnaie(), 150);
    }

    #[test]
    fn test_retrait_monnaie() {
        let mut inventaire = Inventaire::new();
        inventaire.add_monnaie(100);

        // Retirer une partie de la monnaie
        inventaire.remove_monnaie(30);
        assert_eq!(inventaire.get_monnaie(), 70);
    }

    #[test]
    #[should_panic]
    fn test_retrait_monnaie_negative() {
        let mut inventaire = Inventaire::new();

        // Essai de retirer plus que le montant disponible
        inventaire.remove_monnaie(50); // Devrait causer un panic
    }

    #[test]
    fn test_verification_inventaire_vide() {
        let inventaire = Inventaire::new();

        // Vérifier qu'il est vide par défaut
        assert_eq!(inventaire.is_empty(), true);
    }

    #[test]
    fn test_verification_inventaire_plein() {
        let mut inventaire = Inventaire::new();
        inventaire.set_monnaie(u32::MAX);

        // Vérifier qu'il est plein
        assert_eq!(inventaire.is_full(), true);
    }

    #[test]
    fn test_ajout_objets() {
        let mut inventaire = Inventaire::new();

        let objet1 = Objet::new("Épée", "Une épée tranchante",1);
        let objet2 = Objet::new("Bouclier", "Un bouclier solide",1);

        inventaire.set_instance(vec![objet1.clone()]);
        assert_eq!(inventaire.get_instance().len(), 1);
        assert_eq!(inventaire.get_instance()[0], objet1);

        // Ajout d'un autre objet (directement avec set_instance)
        inventaire.set_instance(vec![objet1.clone(), objet2.clone()]);
        assert_eq!(inventaire.get_instance().len(), 2);
    }

    #[test]
    fn test_remplacement_objets() {
        let mut inventaire = Inventaire::new();

        let ancien_objet = Objet::new("Épée", "Une épée ancienne",1);
        let nouvel_objet = Objet::new("Potion", "Potion de soin",1);

        inventaire.set_instance(vec![ancien_objet.clone()]);
        assert_eq!(inventaire.get_instance()[0], ancien_objet);

        // Remplacer l'objet
        inventaire.set_instance(vec![nouvel_objet.clone()]);
        assert_eq!(inventaire.get_instance().len(), 1);
        assert_eq!(inventaire.get_instance()[0], nouvel_objet);
    }

    #[test]
    fn test_inventaire_avec_objets_multiple() {
        let mut inventaire = Inventaire::new();

        let objet1 = Objet::new("Épée", "Une épée tranchante",1);
        let objet2 = Objet::new("Potion", "Potion de soins",1);
        let objet3 = Objet::new("Bouclier", "Un bouclier solide",1);

        // Ajouter plusieurs objets à une liste
        inventaire.set_instance(vec![objet1.clone(), objet2.clone(), objet3.clone()]);

        assert_eq!(inventaire.get_instance().len(), 3);
        assert_eq!(inventaire.get_instance()[0], objet1);
        assert_eq!(inventaire.get_instance()[1], objet2);
        assert_eq!(inventaire.get_instance()[2], objet3);
    }

    #[test]
    fn test_affichage_inventaire() {
        let mut inventaire = Inventaire::new();
        let objet = Objet::new("Épée magique", "Une épée légendaire",1);

        inventaire.set_monnaie(500);
        inventaire.set_instance(vec![objet.clone()]);

        // On peut tester l'affichage en vérifiant la sortie : ici à titre d'exemple
        println!("=== Test affichage inventaire ===");
        inventaire.afficher_inventaire();
    }
}