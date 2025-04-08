use astroship::classes::spaciale::planete::Planete;

/// Teste la création d'une planète avec un nom et un coût de voyage.
#[test]
fn test_creation_planete() {
    let planete = Planete::new("Saturne", 80);

    assert_eq!(planete.nom, "Saturne");
    assert_eq!(planete.cout_voyage, 80);
}
