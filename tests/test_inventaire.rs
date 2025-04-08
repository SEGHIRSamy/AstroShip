#[cfg(test)]
mod tests {
    use astroship::classes::entite::inventaire::Inventaire;
    use astroship::classes::marchandage::objet::Objet;

    /*

     */
    #[test]
    /// Test de création d'un inventaire.
    /// Vérifie que la monnaie et l'objet sont correctement initialisés.
    fn test_creation_inventaire() {
        let objet = Objet::new("Épée magique", "Une épée puissante imprégnée de magie.");

        let inventaire = Inventaire::new(100, objet);

        assert_eq!(inventaire.get_monnaie(), 100);
        assert_eq!(inventaire.get_instance().get_nom(), "Épée magique");
    }


    #[test]
    /// Test de modification de la monnaie via `set_monnaie`.
    fn test_set_monnaie() {
        let objet = Objet::new("Bouclier", "Un bouclier robuste.");

        let mut inventaire = Inventaire::new(50, objet);

        inventaire.set_monnaie(200);
        assert_eq!(inventaire.get_monnaie(), 200);
    }

    #[test]
    /// Test de modification de l'objet dans l'inventaire via `set_instance`.
    fn test_set_instance() {
        let objet1 = Objet::new("Arc", "Un arc en bois d'érable.");
        let objet2 = Objet::new("Hache", "Une hache de combat.");

        let mut inventaire = Inventaire::new(300, objet1);

        inventaire.set_instance(objet2.clone());
        assert_eq!(inventaire.get_instance(), &objet2);
    }

    #[test]
    /// Test de l'ajout de monnaie via `add_monnaie`.
    fn test_add_monnaie() {
        let objet = Objet::new("Épée", "Une épée fine et légère.");

        let mut inventaire = Inventaire::new(500, objet);

        inventaire.add_monnaie(250);
        assert_eq!(inventaire.get_monnaie(), 750);
    }

    #[test]

    /// Test de suppression de monnaie via `remove_monnaie`.
    fn test_remove_monnaie() {
        let objet = Objet::new("Épée", "Une épée fine et légère.");

        let mut inventaire = Inventaire::new(1000, objet);

        inventaire.remove_monnaie(400);
        assert_eq!(inventaire.get_monnaie(), 600);
    }

    #[test]
    /// Test de vérification si l'inventaire est vide avec `is_empty`.
    fn test_is_empty() {
        let objet = Objet::new("Épée", "Une épée fine et légère.");

        let inventaire_plein = Inventaire::new(100, objet.clone());
        let inventaire_vide = Inventaire::new(0, objet.clone());

        assert!(!inventaire_plein.is_empty());
        assert!(inventaire_vide.is_empty());
    }

    #[test]
    /// Test de vérification si l'inventaire est plein avec `is_full`.
    fn test_is_full() {
        let objet = Objet::new("Épée", "Une épée fine et légère.");

        let inventaire_plein = Inventaire::new(u32::MAX, objet.clone());
        let inventaire_non_plein = Inventaire::new(50, objet);

        assert!(inventaire_plein.is_full());
        assert!(!inventaire_non_plein.is_full());
    }
}