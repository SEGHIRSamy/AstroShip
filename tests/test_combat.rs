use astroship::classes::gestion_evenement::combat::Combat;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_degats_echec_critique() {
        let degats = Combat::calculer_degats(50, 30, 1);
        assert_eq!(degats, 1, "Échec critique devrait donner 1 dégât");
    }

    #[test]
    fn test_degats_reussite_critique() {
        let degats = Combat::calculer_degats(50, 30, 20);
        assert_eq!(degats, 999_999, "Réussite critique devrait donner 999_999 dégâts");
    }

    #[test]
    fn test_degats_normal_valeurs_positives() {
        let degats = Combat::calculer_degats(200, 180, 10);
        assert!(
            degats > 0 && degats < 999_999,
            "Dégâts normaux doivent être raisonnables, obtenu : {}",
            degats
        );
    }

    #[test]
    fn test_degats_avec_attaquant_plus_faible() {
        let degats = Combat::calculer_degats(20, 60, 15);
        assert!(
            degats >= 1 && degats < 999_999,
            "Même un attaquant plus faible devrait infliger quelques dégâts, obtenu : {}",
            degats
        );
    }

    #[test]
    fn test_degats_avec_attaquant_plus_fort() {
        let degats = Combat::calculer_degats(80, 20, 12);
        assert!(
            degats > 0 && degats < 999_999,
            "Un attaquant bien plus fort doit infliger des dégâts cohérents, obtenu : {}",
            degats
        );
    }

    #[test]
    #[should_panic(expected = "Le lancer de dé doit être entre 1 et 20 !")]
    fn test_degats_lancer_invalide() {
        Combat::calculer_degats(50, 30, 21);
    }

    #[test]
    fn test_fuite_reussite_critique() {
        let reussi = Combat::tenter_fuite(10, 10, 20);
        assert!(reussi, "Un 20 doit toujours réussir la fuite");
    }

    #[test]
    fn test_fuite_echec_critique() {
        let reussi = Combat::tenter_fuite(10, 10, 1);
        assert!(!reussi, "Un 1 doit toujours échouer la fuite");
    }

    #[test]
    fn test_fuite_normale_reussie() {
        let reussi = Combat::tenter_fuite(15, 25, 19);
        assert!(reussi, "Le fuyard plus rapide avec un bon lancer devrait fuir");
    }

    #[test]
    fn test_fuite_normale_ratee() {
        let reussi = Combat::tenter_fuite(10, 15, 5);
        assert!(!reussi, "Le fuyard plus lent avec un mauvais lancer devrait échouer");
    }

    #[test]
    #[should_panic(expected = "Le lancer de dé doit être entre 1 et 20 !")]
    fn test_fuite_lancer_invalide() {
        Combat::tenter_fuite(10, 10, 0);
    }
}
