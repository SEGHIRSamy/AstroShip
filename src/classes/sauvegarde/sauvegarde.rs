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