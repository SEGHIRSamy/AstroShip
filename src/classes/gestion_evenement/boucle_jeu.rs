use std::{fs, io};
use crate::classes::entite::personnage_principal::PersonnagePrincipal;
use crate::classes::planete::planete::Planete;
use crate::classes::affichage::affiche_texte::AfficheTexte;
use crate::classes::affichage::intro::Intro;
use crate::classes::gestion_evenement::charger_partie::ChargerPartie;
use crate::classes::gestion_evenement::choix::Choix;
use crate::classes::gestion_evenement::quitter_jeu::QuitterJeu;
use crate::classes::sauvegarde::sauvegarde::Sauvegarde;
use crate::classes::spaciale::vaisseau::Vaisseau;
use crate::classes::spaciale::voyage_planete::VoyagePlanete;

pub struct BoucleJeu {
  personnage: PersonnagePrincipal,
  vaisseau: Vaisseau
}

impl BoucleJeu {
  pub fn new(nouvelle_partie: bool) -> BoucleJeu {
    let delais = 30;
    let sauvegarde: Sauvegarde = Sauvegarde::new();
    if nouvelle_partie {
      AfficheTexte::affiche("Bienvenue, aventurier des étoiles.
        Votre vaisseau vient de subir une panne critique : plus une goutte d'uranium, le précieux carburant qui alimente votre propulsion hyperespace.
        Vous dérivez désormais au cœur d’une galaxie inconnue, isolé, vulnérable... mais pas sans ressources.
        Votre mission : explorer, survivre et trouver suffisamment d’uranium pour rallumer vos moteurs et rentrer enfin chez vous.
        ".to_string(), delais);
      // un truc qui lit le nom des planètes dynamiquement
      let planete_init = BoucleJeu::noms_fichiers_sans_extensions("JSON/planete_default");

      for pla in planete_init  {
        let tmp_planete_init = Planete::charge_planete(&pla,true);
        Planete::sauvegarde_planete(tmp_planete_init);
      }

      let personnage: PersonnagePrincipal = sauvegarde.charge("nouveau_personnage.json".to_string()).unwrap();
      sauvegarde.sauvegarde("personnage_principal.json".to_string(), &personnage).unwrap();
      let vaisseau = Vaisseau::new(personnage.get_carburant() , personnage.get_uranium() , None);
      BoucleJeu {
        personnage,
        vaisseau
      }
    } else {
      AfficheTexte::affiche("Chargement de la partie sauvegardée...".to_string(), delais);
      let personnage: PersonnagePrincipal = sauvegarde.charge("personnage_principal.json".to_string()).unwrap();
      if personnage.get_planete_nom() != "" && personnage.get_planete_nom() != "None" {
        let planete: Planete = sauvegarde.charge("planete_json/".to_owned()+&personnage.get_planete_nom()+&".json".to_string()).unwrap();
        let voyage = VoyagePlanete::new(personnage.get_planete_nom(),planete.get_cout_voyage());
        let vaisseau = Vaisseau::new(personnage.get_carburant(), personnage.get_uranium(), Option::from(voyage));
        BoucleJeu {
          personnage,
          vaisseau
        }
      }
      else {
        let vaisseau = Vaisseau::new(personnage.get_carburant(), personnage.get_uranium(), None);
        BoucleJeu {
          personnage,
          vaisseau
        }
      }
    }
  }

  /// Récupère les noms de fichiers sans extensions dans un dossier donné.
  pub fn noms_fichiers_sans_extensions(dossier: &str) -> Vec<String> {
    let mut noms = Vec::new();

    if let Ok(entries) = fs::read_dir(dossier) {
        for entry in entries.flatten() {
            let chemin = entry.path();

            if chemin.is_file() {
                if let Some(nom_fichier) = chemin.file_stem() {
                    if let Some(nom_str) = nom_fichier.to_str() {
                        noms.push(nom_str.to_string());
                    }
                }
            }
        }
    }

    noms
  }

  /// Récupère la liste des noms de fichiers (sans extensions) présents dans un dossier donné.
  ///
  /// Cette fonction lit le contenu du répertoire spécifié, filtre uniquement les fichiers (en
  /// ignorant les dossiers et fichiers illisibles), extrait le nom de chaque fichier sans son
  /// extension, et retourne la liste de ces noms sous forme de `Vec<String>`.
  ///
  /// # Arguments
  ///
  /// * `dossier` - Un chemin vers le répertoire à parcourir (ex: `"./mes_fichiers"`).
  ///
  /// # Retour
  ///
  /// Un `Vec<String>` contenant les noms de fichiers sans extensions.  
  /// Si le répertoire n'existe pas ou ne peut pas être lu, la fonction retourne une liste vide.
  ///
  /// # Exemple
  ///
  /// ```rust
  /// let fichiers = noms_fichiers_sans_extensions("./data");
  /// for nom in fichiers {
  ///     println!("{}", nom);
  /// }
  /// ```
  ///
  /// # Remarques
  ///
  /// - Les fichiers dont le nom n’est pas valide en UTF-8 sont ignorés.
  /// - Cette fonction **n'explore pas les sous-dossiers** (pas de récursivité).
  /// - Elle **ignore les erreurs silencieusement** (fichiers illisibles, permissions...).
  pub fn boucle_jeu(&mut self) {
    let sauvegarde: Sauvegarde = Sauvegarde::new();
    let nbr_uranium_demande: u32 = 30;
    let mut jeu_en_cours: bool = true;

    let liste_nom_planete = BoucleJeu::noms_fichiers_sans_extensions("JSON/planete_json");

    let mut planetes_disponibles: Vec<VoyagePlanete> = Vec::new();

    for pla in liste_nom_planete {
        let tmp_planete = Planete::charge_planete(&pla, false);
        let voyage = VoyagePlanete::new(&tmp_planete.nom, tmp_planete.get_cout_voyage());
        planetes_disponibles.push(voyage);
    }

    while (jeu_en_cours && self.personnage.entite.get_points_de_vie() > 0) &&
        (self.personnage.get_uranium() < nbr_uranium_demande) {

      if !self.vaisseau.get_position().is_none() && self.personnage.get_planete_nom() != "Dans l'espace" {
        let mut plat = Planete::charge_planete(self.personnage.get_planete_nom(),false);
        plat.visiter(&mut self.personnage);
        self.vaisseau.set_position(None);
        if self.personnage.get_uranium() >= nbr_uranium_demande {
          break;
        }
      }

      if self.personnage.get_carburant() <= 0 && self.vaisseau.afficher_etat() == "Dans l'espace".to_string() {
        println!("Malheuresement vous êtes bloqué dans l'espace, vous n'avez pas su gérer votre budget carburant\n\
         cela vous a offert une croisière dans l'espace jusqu'à la fin de vos jours.\n\
         Entre autre : ");
        self.personnage.entite.set_points_de_vie(0);
        break;
      }

      let sauvegarde: Sauvegarde = Sauvegarde::new();

      //TODO voir pour comment faire pour charger directement la planete quand on charge le jeu
      println!("\n=== Menu de navigation ===");
      self.vaisseau.afficher_etat();

      println!("Choisissez une planète à visiter :");
      for (i, p) in planetes_disponibles.iter().enumerate() {
        println!("[{}] {} Carburant nécessaire : {}", i + 1, p.nom, p.cout_voyage);
      }
      println!("[0] Quitter");

      let mut choix = String::new();
      io::stdin().read_line(&mut choix).unwrap();

      match choix.trim().parse::<usize>() {
        Ok(0) => {
          println!("Vous avez quitté le jeu.");
          sauvegarde.sauvegarde("personnage_principal.json".to_string(), self.personnage.clone()).expect("Enregistrement Personnage échoué");
          jeu_en_cours = false;
        }
        Ok(index) if index >= 1 && index <= planetes_disponibles.len() => {
          let planete_selectionnee = &planetes_disponibles[index - 1];
          println!("Vous avez choisi de voyager vers {}", planete_selectionnee.nom);
          self.personnage.set_planete(&*planete_selectionnee.nom.clone());
          self.personnage.set_carburant(self.personnage.get_carburant()-planete_selectionnee.cout_voyage.clone());
          sauvegarde.sauvegarde("personnage_principal.json".to_string(), self.personnage.clone()).expect("Enregistrement boucle jeu 98 raté");
          self.vaisseau.voyager(planete_selectionnee);
          let mut plat = Planete::charge_planete(self.personnage.get_planete_nom(),false);
          plat.visiter(&mut self.personnage);
          self.vaisseau.set_position(None);
        }
        _ => {
          println!("Choix invalide. Veuillez entrer un nombre valide.");
        }
      }
    }

    if self.personnage.entite.get_points_de_vie() <= 0 {
      AfficheTexte::affiche("\nVous êtes mort !".to_string(), 30);
      let mut choix_mort = Choix::new(vec![
        ("Charger la dernière sauvegarde".to_string(), Box::new(ChargerPartie::new())),
        ("Quitter".to_string(), Box::new(QuitterJeu::new()))
      ]);
      choix_mort.lancer_choix();
    }
    else if self.personnage.get_uranium() >= nbr_uranium_demande {
      sauvegarde.sauvegarde("personnage_principal.json".to_string(), &self.personnage).unwrap();
      Intro::lancer_outro();
      AfficheTexte::affiche("Vous avez gagné !!!!!!!".to_string(), 50);
    }
  }


}