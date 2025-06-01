use std::io;
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
    let sauvegarde: Sauvegarde = Sauvegarde::new();
    if nouvelle_partie {
      AfficheTexte::affiche("Création d'une nouvelle partie...".to_string(), 30);
      let personnage: PersonnagePrincipal = sauvegarde.charge("nouveau_personnage.json".to_string()).unwrap();
      sauvegarde.sauvegarde("personnage_principal.json".to_string(), &personnage).unwrap();
      let vaisseau = Vaisseau::new(personnage.get_carburant() , personnage.get_uranium() , None);
      BoucleJeu {
        personnage,
        vaisseau
      }
    } else {
      AfficheTexte::affiche("Chargement de la partie sauvegardée...".to_string(), 30);
      let personnage: PersonnagePrincipal = sauvegarde.charge("personnage_principal.json".to_string()).unwrap();
      let planete: Planete = sauvegarde.charge("planete_json/".to_owned()+&personnage.get_planete_nom()+&".json".to_string()).unwrap();


      let voyage = VoyagePlanete::new(personnage.get_planete_nom(),planete.get_cout_voyage());
      let vaisseau = Vaisseau::new(personnage.get_carburant(), personnage.get_uranium(), Option::from(voyage));
      BoucleJeu {
        personnage,
        vaisseau
      }
    }
  }

  pub fn boucle_jeu(&mut self) {
    let sauvegarde: Sauvegarde = Sauvegarde::new();
    let nbr_uranium_demande: u32 = 10;
    let mut jeu_en_cours: bool = true; // quitter le jeu avec une option quitter

    let planetes_disponibles = vec![
      VoyagePlanete::new("Mars", 20),        // Exemple
      VoyagePlanete::new("Neptune", 30),
      VoyagePlanete::new("Pluton", 40),
    ];
    
    while (jeu_en_cours && self.personnage.entite.get_points_de_vie() > 0) &&
        (self.personnage.get_uranium() < nbr_uranium_demande) {

      if (self.personnage.get_carburant()<=0) {
        println!("Malheuresement vous êtes bloqué dans l'espace, vous n'avez pas su gérer votre budget carburant\n\
         cela vous a offert une croisière dans l'espace jusqu'à la fin de vos jours.\n\
         Entre autre : ");
        self.personnage.entite.set_points_de_vie(0);
        jeu_en_cours = false;
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
          self.personnage.set_carburant(self.personnage.get_carburant()-planete_selectionnee.cout_voyage);
          self.vaisseau.voyager(planete_selectionnee);

          let mut plat = Planete::charge_planete(self.personnage.get_planete_nom());
          plat.visiter(&mut self.personnage);

        }
        _ => {
          println!("Choix invalide. Veuillez entrer un nombre valide.");
        }
      }

    }

    if self.personnage.entite.get_points_de_vie() <= 0 {
      AfficheTexte::affiche("\nVous êtes mort !".to_string(), 30);
      let choix_mort = Choix::new(vec![
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