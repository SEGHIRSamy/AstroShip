use astroship::classes::entite::entite::{Entite, Personnage};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation_entite() {
        let entite = Entite::new("Héros", 100, 100, 15, 10, 12);
        assert_eq!(entite.get_nom(), "Héros");
        assert_eq!(entite.get_points_de_vie(), 100);
        assert_eq!(entite.get_force(), 15);
        assert_eq!(entite.get_intelligence(), 10);
        assert_eq!(entite.get_vitesse(), 12);
    }

    #[test]
    fn test_subir_degats_entite() {
        let mut entite = Entite::new("Héros", 100, 100, 15, 10, 12);
        entite.subir_degats(50);
        assert_eq!(entite.get_points_de_vie(), 50);

        entite.subir_degats(60);
        assert_eq!(entite.get_points_de_vie(), 0); // PV ne doivent pas descendre en-dessous de 0
    }

    #[test]
    fn test_soigner_entite() {
        let mut entite = Entite::new("Héros", 50, 100, 15, 10, 12);
        entite.soigner(20);
        assert_eq!(entite.get_points_de_vie(), 70); // Les PV augmentent après le soin

        entite.soigner(40); // Test pour éviter de dépasser les PV max
        assert!(entite.get_points_de_vie() <= entite.get_points_de_vie_max());
    }

    #[test]
    fn test_augmentation_niveau_force() {
        let mut entite = Entite::new("Héros", 100, 100, 15, 10, 12);
        entite.augmentation_niveau("force");
        assert_eq!(entite.get_force(), 16); // Force + 1
        assert_eq!(entite.get_points_de_vie_max(), 105); // PV max + 5
        assert_eq!(entite.get_points_de_vie(), 105); // PV normaux après ajustement
    }

    #[test]
    fn test_augmentation_niveau_vitesse() {
        let mut entite = Entite::new("Héros", 100, 100, 15, 10, 12);
        entite.augmentation_niveau("vitesse");
        assert_eq!(entite.get_vitesse(), 13); // Vitesse + 1
        assert_eq!(entite.get_points_de_vie_max(), 105); // PV max + 5
        assert_eq!(entite.get_points_de_vie(), 105); // PV normaux après ajustement
    }

    #[test]
    fn test_augmentation_niveau_statistique_invalide() {
        let mut entite = Entite::new("Héros", 100, 100, 15, 10, 12);
        entite.augmentation_niveau("inconnue");
        assert_eq!(entite.get_force(), 15); // Aucune statistique ne doit changer
        assert_eq!(entite.get_vitesse(), 12);
        assert_eq!(entite.get_intelligence(), 10);
    }

    #[test]
    fn test_vivant_mort() {
        let mut entite = Entite::new("Héros", 50, 100, 15, 10, 12);
        assert_eq!(entite.est_mort(), false);

        entite.subir_degats(60); // Mort
        assert_eq!(entite.get_points_de_vie(), 0);
        assert_eq!(entite.est_mort(), true);
    }
}