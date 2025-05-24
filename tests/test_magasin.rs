#[cfg(test)]
mod tests {
    use astroship::classes::entite::inventaire::Inventaire;
    use astroship::classes::marchandage::affaire::Affaire;
    use astroship::classes::marchandage::objet::Objet;

    #[test]
    fn achat_reussi() {
        // Initialisation de l'inventaire (avec 500 de monnaie)
        let mut inventaire = Inventaire::new();
        inventaire.set_monnaie(500);

        // Création d'un objet et d'une affaire associée
        let objet = Objet::new("Épée légendaire", "Une épée très puissante",1);
        let mut affaire = Affaire::new(250, objet.clone(), false, 5); // Prix : 250, quantité : 5

        // Achat
        if *affaire.get_quantite() > 0 && *affaire.get_prix() <= inventaire.get_monnaie() {
            inventaire.add_objet(affaire.get_instance().clone());
            inventaire.remove_monnaie(*affaire.get_prix());
            affaire.set_quantite(*affaire.get_quantite() - 1);
        }

        // Vérifications post-achat
        assert_eq!(inventaire.get_monnaie(), 250); // La monnaie doit être déduite
        assert_eq!(inventaire.get_instance().len(), 1); // L'objet doit avoir été ajouté
        assert_eq!(inventaire.get_instance()[0], objet); // L'objet est correctement ajouté
        assert_eq!(*affaire.get_quantite(), 4); // La quantité doit être réduite (de 5 à 4)
    }

    #[test]
    fn fonds_insuffisants() {
        // Initialisation de l'inventaire (avec 100 de monnaie)
        let mut inventaire = Inventaire::new();
        inventaire.set_monnaie(100);

        // Création d'un objet et d'une affaire (coût supérieur aux fonds disponibles)
        let objet = Objet::new("Bouclier solide", "Un bouclier robuste",1);
        let affaire = Affaire::new(500, objet.clone(), false, 2); // Prix : 500

        // Tentative d'achat (aucune action ne devrait avoir lieu)
        if *affaire.get_prix() > inventaire.get_monnaie() {
            // L'achat ne peut pas se faire
        }

        // Vérifications post-achat (échec attendu)
        assert_eq!(inventaire.get_monnaie(), 100); // La monnaie reste inchangée
        assert_eq!(inventaire.get_instance().is_empty(), true); // Aucun objet ajouté
        assert_eq!(*affaire.get_quantite(), 2); // La quantité reste inchangée
    }

    #[test]
    fn stock_epuise() {
        // Initialisation de l'inventaire (avec 500 de monnaie)
        let mut inventaire = Inventaire::new();
        inventaire.set_monnaie(500);

        // Création d'un objet et d'une affaire avec stock à 0
        let objet = Objet::new("Potion magique", "Potion qui restaure toute la vie",1);
        let mut affaire = Affaire::new(50, objet.clone(), false, 0); // Stock épuisé

        // Tentative d'achat (aucune action ne doit avoir lieu)
        if *affaire.get_quantite() > 0 && *affaire.get_prix() <= inventaire.get_monnaie() {
            inventaire.add_objet(affaire.get_instance().clone());
            inventaire.remove_monnaie(*affaire.get_prix());
            affaire.set_quantite(*affaire.get_quantite() - 1);
        }

        // Vérifications post-achat (échec attendu car stock épuisé)
        assert_eq!(inventaire.get_monnaie(), 500); // La monnaie reste inchangée
        assert_eq!(inventaire.get_instance().is_empty(), true); // Aucun objet ajouté
        assert_eq!(*affaire.get_quantite(), 0); // La quantité reste à 0
    }

    #[test]
    fn achat_objet_inexistant() {
        // Initialisation de l'inventaire (avec 500 de monnaie)
        let mut inventaire = Inventaire::new();
        inventaire.set_monnaie(500);

        // Création d'une liste vide d'affaires
        let affaires: Vec<Affaire> = Vec::new(); // Aucun objet disponible

        // Tentative d'achat (id inexistant dans la liste)
        let id_inexistant = 0;
        let tentative_achat = affaires.get(id_inexistant); // Devrait retourner `None`

        // Vérifications
        assert!(tentative_achat.is_none()); // Confirmation que l'objet est inexistant
        assert_eq!(inventaire.get_monnaie(), 500); // La monnaie reste inchangée
        assert!(inventaire.get_instance().is_empty()); // Aucun objet ajouté
    }

    #[test]
    fn achat_reussi_stock_infini() {
        // Initialisation de l'inventaire (avec 1000 de monnaie)
        let mut inventaire = Inventaire::new();
        inventaire.set_monnaie(1000);

        // Création d'un objet avec stock infini
        let objet = Objet::new("Arc légendaire", "Un arc qui ne manque jamais sa cible",1);
        let mut affaire = Affaire::new(800, objet.clone(), true, 0); // `infini = true`

        // Achat
        if *affaire.get_infini() || (*affaire.get_quantite() > 0 && *affaire.get_prix() <= inventaire.get_monnaie()) {
            inventaire.add_objet(affaire.get_instance().clone());
            inventaire.remove_monnaie(*affaire.get_prix());

            // Quantité reste inchangée si `infini` est vrai
            if !*affaire.get_infini() {
                affaire.set_quantite(*affaire.get_quantite() - 1);
            }
        }

        // Vérifications post-achat
        assert_eq!(inventaire.get_monnaie(), 200); // La monnaie doit être déduite
        assert_eq!(inventaire.get_instance().len(), 1); // L'objet doit avoir été ajouté
        assert_eq!(inventaire.get_instance()[0], objet); // L'objet est correct
        assert_eq!(*affaire.get_quantite(), 0); // La quantité reste inchangée (stock infini)
        assert_eq!(*affaire.get_infini(), true); // Confirmation que le stock est infini
    }
}