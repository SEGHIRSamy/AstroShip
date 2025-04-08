use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Test {
  name: String,
  age: u32
}

impl Test {
  pub fn eq(&self, other: Test) -> bool {
    return self.name == other.name && self.age == other.age;
  }

  pub fn new(name: &str, age: u32) -> Self {
    Test {
      name: name.to_string(),
      age,
    }
  }
}

#[cfg(test)]
mod tests {
  use astroship::classes::sauvegarde::sauvegarde::Sauvegarde;
  use crate::Test;

  fn vec_compare(va: &[Test], vb: &[Test]) -> bool {
    (va.len() == vb.len()) &&  // zip stops at the shortest
    va.iter()
      .zip(vb)
      .all(|(a,b)| a == b)
  }


  #[test]
  fn test_sauvegarde_et_chargement() {
    let mut tests: Vec<Test> = Vec::new();

    // Ajouter des éléments à ce vecteur
    tests.push(Test::new("Alice", 30));
    tests.push(Test::new("Bob", 25));
    tests.push(Test::new("Charlie", 35));

    // On créer le service de sauvegarde
    let sauvegarde: Sauvegarde = Sauvegarde::new();

    // On sauvegarder les données dans un fichier json
    sauvegarde.sauvegarde("test.json".to_string(), &tests).unwrap();

    // On les charge pour vérifier qu'il est correctement sauvegarder
    let tests_charge: Vec<Test> = sauvegarde.charge("test.json".to_string()).unwrap();

    assert!(vec_compare(&tests, &tests_charge)); // On test si les deux vecteur sont identique
  }

  #[test]
  fn test_erreur_chargement() {
    // On créer le service de sauvegarde
    let sauvegarde: Sauvegarde = Sauvegarde::new();

    // On donne un fichier qui n'existe pas
    let resultat: Result<Test, serde_json::Error> = sauvegarde.charge("testEchec.json".to_string());

    // On test si il y a une erreur
    assert!(resultat.is_err());
  }
}