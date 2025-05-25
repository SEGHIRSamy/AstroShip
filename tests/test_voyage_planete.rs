use astroship::classes::spaciale::voyage_planete::VoyagePlanete;

/// Teste la création d'une planète avec un nom et un coût de voyage.
#[test]
fn test_creation_planete() {
    let planete = VoyagePlanete::new("Saturne", 80);

    assert_eq!(planete.nom, "Saturne");
    assert_eq!(planete.cout_voyage, 80);
}
