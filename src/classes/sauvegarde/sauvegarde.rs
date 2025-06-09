use serde::de::Error;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs::File;
use std::io::Write;

pub struct Sauvegarde {
  chemin: String,
}

#[allow(dead_code)]
impl Sauvegarde
{
  pub fn new() -> Self {
    Self {
      chemin:"JSON/".to_string(),
    }
  }

  /// Charge un objet sérialisé depuis un fichier JSON donné.
  ///
  /// # Type Parameters
  /// - `T`: Le type de l'objet à désérialiser. Il doit implémenter le trait [`Deserialize`].
  ///
  /// # Arguments
  /// - `file`: Le nom du fichier (relatif au chemin stocké dans `self.chemin`) à lire.
  ///
  /// # Returns
  /// - `Ok(T)`: L'objet désérialisé.
  /// - `Err(serde_json::Error)`: Si le fichier est introuvable ou si la désérialisation échoue.
  ///
  /// # Panics
  /// Cette fonction panique si la désérialisation échoue, car elle utilise `unwrap()`.
  ///
  /// # Improvements
  /// Pour plus de robustesse, il serait préférable d'utiliser `?` au lieu de `unwrap()`.
  #[allow(dead_code)]
  pub fn charge<T>(&self, file: String) -> Result<T>
    where T: for<'de> Deserialize<'de>
  {
    let file = File::open(self.chemin.clone() + &file);
    if file.is_err() {
      return Err(serde_json::Error::custom("Fichier non trouvé"));
    }
    let reader = std::io::BufReader::new(file.unwrap());
    let obj: T = serde_json::from_reader(reader).unwrap();
    Ok(obj)
  }

  /// Sauvegarde un objet sérialisable dans un fichier JSON avec une mise en forme lisible.
  ///
  /// # Type Parameters
  /// - `T`: Le type de l'objet à sérialiser. Il doit implémenter le trait [`Serialize`].
  ///
  /// # Arguments
  /// - `file`: Le nom du fichier (relatif à `self.chemin`) dans lequel sauvegarder.
  /// - `obj`: L'objet à sérialiser et à écrire dans le fichier.
  ///
  /// # Returns
  /// - `Ok(())`: Si la sauvegarde est réussie.
  /// - `Err(serde_json::Error)`: Jamais retourné actuellement, mais la signature le permet.
  ///
  /// # Panics
  /// Cette fonction panique si la sérialisation ou l'écriture dans le fichier échoue (`unwrap()`).
  ///
  /// # Improvements
  /// Il serait recommandé de remplacer tous les `unwrap()` par `?` pour un traitement d'erreurs plus sûr.
  #[allow(dead_code)]
  pub fn sauvegarde<T>(&self, file: String, obj: T) -> Result<()>
    where T: Serialize
  {
    let json_data = serde_json::to_string_pretty(&obj).unwrap();
    let file = File::create(self.chemin.clone() + &file);
    file.unwrap().write(json_data.as_bytes()).unwrap();
    Ok(())
  }
}