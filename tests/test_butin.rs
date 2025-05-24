#[cfg(test)]
mod tests {
    use rand::{SeedableRng}; // Pour un générateur de nombres aléatoires avec graine
    use rand_chacha::ChaCha8Rng; // Générateur RNG déterministe pour tester les probabilités

    use astroship::classes::marchandage::butin::{Butin, Rarete};
    use astroship::classes::marchandage::objet::Objet;

    #[test]
    fn test_creation_butin() {
        let objet = Objet::new("Épée légendaire", "Une arme ancienne dotée de pouvoirs mystiques.",1);
        let butin = Butin::new(objet.clone(), 1, 0.8, Rarete::Legendaire);

        assert_eq!(butin.objet.get_nom(), "Épée légendaire");
        assert_eq!(butin.objet.get_description(), "Une arme ancienne dotée de pouvoirs mystiques.");
        assert_eq!(butin.quantite, 1);
        assert!((butin.probabilite - 0.8).abs() < f32::EPSILON, "La probabilité devrait être 0.8");
        assert_eq!(butin.rarete, Rarete::Legendaire);
    }

    #[test]
    fn test_probabilite_hors_limites() {
        let objet = Objet::new("Potion rare", "Une potion très puissante.",1);

        // Une probabilité invalide (< 0.0 ou > 1.0) doit provoquer une panique
        let result = std::panic::catch_unwind(|| Butin::new(objet.clone(), 2, 1.5, Rarete::Rare));
        assert!(result.is_err(), "La probabilité hors limites (> 1.0) aurait dû provoquer une panique");
    }

    #[test]
    fn test_est_obtenu_probabilite_100() {
        let objet = Objet::new("Bouclier indestructible", "Un bouclier légendaire.",1);
        let butin = Butin::new(objet, 1, 1.0, Rarete::Legendaire);
        let mut rng = ChaCha8Rng::seed_from_u64(42);

        // Avec une probabilité de 100 %, le butin doit toujours être obtenu
        for _ in 0..10 {
            assert!(butin.est_obtenu(&mut rng), "Le butin devrait toujours être obtenu avec une probabilité de 100 %");
        }
    }

    #[test]
    fn test_est_obtenu_probabilite_0() {
        let objet = Objet::new("Cendres", "Des cendres ordinaires.",1);
        let butin = Butin::new(objet, 3, 0.0, Rarete::Commun);
        let mut rng = ChaCha8Rng::seed_from_u64(42);

        // Avec une probabilité de 0 %, le butin ne doit jamais être obtenu
        for _ in 0..10 {
            assert!(!butin.est_obtenu(&mut rng), "Le butin ne devrait jamais être obtenu avec une probabilité de 0 %");
        }
    }

    #[test]
    fn test_est_obtenu_deterministe() {
        let objet = Objet::new(
            "Anneau d'invisibilité",
            "Un anneau magique permettant à son porteur de devenir invisible.",
        1);
        let butin = Butin::new(objet, 1, 0.5, Rarete::Rare);

        // Utilisation d'un RNG déterministe pour des tests reproductibles
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let tirages: Vec<bool> = (0..5).map(|_| butin.est_obtenu(&mut rng)).collect();

        // Tester manuellement les tirages avec la même graine
        assert_eq!(tirages, vec![true, false, true, false, false]);
    }
}
