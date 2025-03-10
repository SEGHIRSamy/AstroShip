// On va importer les structures nécessaires depuis le module source
use crate::classes::spaciale::vaisseau::Vaisseau;
use crate::classes::spaciale::planete::Planete;

#[cfg(test)] // Cette ligne indique que ce module ne sera compilé que pour les tests
mod tests {
    use super::*; // Cela permet d'utiliser les structures Vaisseau et Planete dans ce module

    // Test pour un voyage réussi
    #[test]
    fn test_voyage_reussi() {
        // Création des planètes
        let terre = Planete::new("Terre", 0);
        let mars = Planete::new("Mars", 30);

        // Création d'un vaisseau avec 100 unités de carburant et 50 de charge utile
        let mut vaisseau = Vaisseau::new(100, 50, Some(terre));

        // On fait voyager le vaisseau de la Terre à Mars
        let success = vaisseau.voyager(&mars);

        // On vérifie que le voyage a réussi
        assert!(success);

        // On vérifie que le carburant a bien diminué de 30 unités (distance entre Terre et Mars)
        assert_eq!(vaisseau.carburant, 70);

        // On vérifie que la position du vaisseau est bien Mars
        assert_eq!(vaisseau.position.unwrap().nom, "Mars");
    }

    // Test pour un voyage échoué (pas assez de carburant)
    #[test]
    fn test_voyage_echec() {
        // Création des planètes
        let terre = Planete::new("Terre", 0);
        let jupiter = Planete::new("Jupiter", 120);

        // Création d'un vaisseau avec 50 unités de carburant (pas assez pour rejoindre Jupiter)
        let mut vaisseau = Vaisseau::new(50, 50, Some(terre));

        // Tentative de voyage vers Jupiter
        let success = vaisseau.voyager(&jupiter);

        // On vérifie que le voyage a échoué
        assert!(!success);

        // On vérifie que le carburant est toujours à 50 (pas de changement)
        assert_eq!(vaisseau.carburant, 50);

        // On vérifie que la position du vaisseau est toujours la Terre
        assert_eq!(vaisseau.position.unwrap().nom, "Terre");
    }

    // Test pour vérifier un vaisseau sans position initiale dans l'espace
    #[test]
    fn test_vaisseau_dans_espace() {
        // Création d'un vaisseau sans position initiale
        let mut vaisseau = Vaisseau::new(100, 50, None);

        // On vérifie que le vaisseau est bien dans l'espace (pas de planète assignée)
        assert!(vaisseau.position.is_none());

        // Création de la planète Mars
        let mars = Planete::new("Mars", 30);

        // On tente de voyager vers Mars
        let success = vaisseau.voyager(&mars);

        // On vérifie que le voyage est réussi
        assert!(success);

        // On vérifie que la position du vaisseau est bien Mars
        assert_eq!(vaisseau.position.unwrap().nom, "Mars");
    }
}
