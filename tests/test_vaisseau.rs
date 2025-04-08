use astroship::classes::spaciale::{planete::Planete, vaisseau::Vaisseau};

/// Teste la création d'un vaisseau avec des valeurs initiales
#[test]
fn test_creation_vaisseau() {
    let vaisseau = Vaisseau::new(100, 10, None);
    assert_eq!(vaisseau.get_carburant(), 100);
    assert_eq!(vaisseau.get_uranium(), 10);
    assert!(vaisseau.get_position().is_none());
}

/// Teste un voyage réussi vers une planète avec assez de carburant
#[test]
fn test_voyager_succes() {
    let mut vaisseau = Vaisseau::new(100, 10, None);
    let mars = Planete::new("Mars", 50);

    let result = vaisseau.voyager(&mars);

    assert!(result);
    assert_eq!(vaisseau.get_carburant(), 50);
    assert!(vaisseau.get_position().is_some());
    assert_eq!(vaisseau.get_position().unwrap().nom, "Mars");
}

/// Teste un échec de voyage si le carburant est insuffisant
#[test]
fn test_voyager_echec() {
    let mut vaisseau = Vaisseau::new(30, 10, None);
    let jupiter = Planete::new("Jupiter", 50);

    let result = vaisseau.voyager(&jupiter);

    assert!(!result);
    assert_eq!(vaisseau.get_carburant(), 30);
    assert!(vaisseau.get_position().is_none());
}
