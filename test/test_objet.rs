// tests/test_objet.rs
use AstroShip::Objet; // Remplace "AstroShip" par le nom de ton package
#[cfg(test)]
mod tests {
    #[test]
    fn test_creation_objet() {
        // Création d'un objet
        let objet = Objet::new("Épée magique", "Une épée puissante imprégnée de magie.");

        // Test des valeurs après création
        assert_eq!(objet.get_nom(), "Épée magique");
        assert_eq!(objet.get_description(), "Une épée puissante imprégnée de magie.");
    }

    #[test]
    fn test_set_description() {
        // Création d'un objet
        let mut objet = Objet::new("Bouclier", "Un bouclier robuste.");

        // Modification de la description
        objet.set_description("Un bouclier en métal renforcé.".to_string());

        // Vérification après modification
        assert_eq!(objet.get_description(), "Un bouclier en métal renforcé.");
    }

    #[test]
    fn test_set_nom() {
        // Création d'un objet
        let mut objet = Objet::new("Hache", "Une hache de combat.");

        // Modification du nom
        objet.set_nom("Hache de guerre".to_string());

        // Vérification après modification
        assert_eq!(objet.get_nom(), "Hache de guerre");
    }

    #[test]
    fn test_get_nom() {
        // Création d'un objet
        let objet = Objet::new("Arc", "Un arc en bois d'érable.");

        // Vérification du getter
        assert_eq!(objet.get_nom(), "Arc");
    }

    #[test]
    fn test_get_description() {
        // Création d'un objet
        let objet = Objet::new("Épée", "Une épée fine et légère.");

        // Vérification du getter
        assert_eq!(objet.get_description(), "Une épée fine et légère.");
    }
}
