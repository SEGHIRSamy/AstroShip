use astroship::classes::gestionEvenement::lancerDice::LancerDice;
use std::collections::HashSet;


#[test]
fn test_lancer_dans_plage_valide() {
    for _ in 0..100 {
        let resultat = LancerDice::lancer();
        assert!(
            (1..=20).contains(&resultat),
            "Résultat hors de la plage : {}",
            resultat
        );
    }
}

#[test]
fn test_variabilite_lancer() {
    let mut resultats = HashSet::new();
    for _ in 0..100 {
        resultats.insert(LancerDice::lancer());
    }
    assert!(
        resultats.len() > 1,
        "Les résultats semblent constants : {:?}",
        resultats
    );
}
